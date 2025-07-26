use std::{
    collections::{HashSet, VecDeque},
    path::PathBuf,
    sync::{Arc, Mutex},
};

use eyre::{Context, ContextCompat};
use minijinja::{context, value::Rest};
use rand::{Rng, RngCore, SeedableRng};
use serde::Serialize;
use sha2::Digest;
use tracing::{info, level_filters::LevelFilter};
use tracing_subscriber::{
    EnvFilter,
    fmt::{self, format::FmtSpan},
    layer::SubscriberExt,
    util::SubscriberInitExt,
};

fn main() -> eyre::Result<()> {
    tracing_subscriber::registry()
        .with(
            fmt::layer()
                .with_writer(|| Box::new(std::io::stderr()))
                .with_target(true)
                .with_span_events(FmtSpan::CLOSE),
        )
        .with(
            EnvFilter::builder()
                .with_default_directive(LevelFilter::INFO.into())
                .from_env_lossy(),
        )
        .init();

    render()
}

#[derive(Clone, Debug, Serialize)]
pub struct AcmeCerts {
    // pub base_dir: PathBuf,
    pub certificate_pem: PathBuf,
    pub trusted_certificate_pem: PathBuf,
    pub certificate_key_pem: PathBuf,
}

impl From<AcmeCerts> for minijinja::Value {
    fn from(value: AcmeCerts) -> Self {
        minijinja::Value::from_serialize(value)
    }
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct Site {
    pub name: String,
    pub domain: String,
    pub domain_aliases: Option<Vec<String>>,
    pub https_enable: bool,
    pub https_redirect_to_primary_domain: bool,
    pub locations: Vec<Location>,
}

impl From<Site> for minijinja::Value {
    fn from(value: Site) -> Self {
        minijinja::Value::from_serialize(value)
    }
}

#[derive(Clone, Debug, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Location {
    Static(SiteStaticLocation),
    ReverseProxy(SiteReverseProxy),
}

impl From<Location> for minijinja::Value {
    fn from(value: Location) -> Self {
        minijinja::Value::from_serialize(value)
    }
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct SiteStaticLocation {
    pub path: String,
    pub webroot: PathBuf,
}

impl From<SiteStaticLocation> for minijinja::Value {
    fn from(value: SiteStaticLocation) -> Self {
        minijinja::Value::from_serialize(value)
    }
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct SiteReverseProxy {
    pub path: String,
    pub target: String,
}

impl From<SiteReverseProxy> for minijinja::Value {
    fn from(value: SiteReverseProxy) -> Self {
        minijinja::Value::from_serialize(value)
    }
}

// From nanoid@0.4.0, adjusted to take Rng instead
fn nanoid_format(rng: &mut dyn RngCore, alphabet: &[char], size: usize) -> String {
    assert!(
        alphabet.len() <= u8::max_value() as usize,
        "The alphabet cannot be longer than a `u8` (to comply with the `random` function)"
    );

    let mask = alphabet.len().next_power_of_two() - 1;
    let step: usize = 8 * size / 5;

    // Assert that the masking does not truncate the alphabet. (See #9)
    debug_assert!(alphabet.len() <= mask + 1);

    let mut id = String::with_capacity(size);

    loop {
        let mut bytes: Vec<u8> = vec![0; step];
        rng.fill(&mut bytes[..]);

        for byte in bytes {
            let byte = byte as usize & mask;

            if alphabet.len() > byte {
                id.push(alphabet[byte]);

                if id.len() == size {
                    return id;
                }
            }
        }
    }
}

fn gen_site_seed(site: &Site) -> String {
    const HEX_ALPHABET: [char; 16] = [
        '1', '2', '3', '4', '5', '6', '7', '8', '9', '0', 'a', 'b', 'c', 'd', 'e', 'f',
    ];

    let seed = sha2::Sha256::digest(&site.name);
    let mut rng = rand::rngs::StdRng::from_seed(seed.into());

    nanoid_format(&mut rng, &HEX_ALPHABET, 16)
}

fn render() -> eyre::Result<()> {
    let sites_enabled = PathBuf::from("/etc/nginx/sites-enabled");
    let sites_available = PathBuf::from("/etc/nginx/sites-available");
    let acme_webroot_directory = PathBuf::from("/var/www/certbot");

    let sites = vec![
        Site {
            name: "example.com".to_string(),
            domain: "example.com".to_string(),
            domain_aliases: Some(vec!["www.example.com".to_string()]),
            https_enable: true,
            ..Default::default()
        },
        Site {
            name: "foobar.io".to_string(),
            domain: "foobar.io".to_string(),
            domain_aliases: Some(vec!["www.foobar.io".to_string()]),
            https_enable: true,
            https_redirect_to_primary_domain: true,
            locations: vec![
                Location::Static(SiteStaticLocation {
                    path: "/".to_string(),
                    webroot: PathBuf::from("/var/www/foobar.io"),
                }),
                Location::ReverseProxy(SiteReverseProxy {
                    path: "/api".to_string(),
                    target: "http://127.0.0.1:8080/api".to_string(),
                }),
            ],
            ..Default::default()
        },
    ];

    #[derive(Debug, Hash, PartialEq, Eq)]
    struct AcmeRequest {
        primary_domain: String,
        additional_domains: Vec<String>,
    }

    let collected_acme_requests: Arc<Mutex<HashSet<AcmeRequest>>> = Default::default();

    let mut env = minijinja::Environment::new();
    env.add_global(
        "acme_webroot_directory",
        minijinja::Value::from_serialize(&acme_webroot_directory),
    );

    env.add_template(
        "snippets/server_name.conf.j2",
        include_str!("snippets/server_name.conf.j2"),
    )?;
    env.add_template(
        "snippets/http_connection_map.conf.j2",
        include_str!("snippets/http_connection_map.conf.j2"),
    )?;
    env.add_template(
        "snippets/certbot_webroot.conf.j2",
        include_str!("snippets/certbot_webroot.conf.j2"),
    )?;
    env.add_template(
        "snippets/https_redirect.conf.j2",
        include_str!("snippets/https_redirect.conf.j2"),
    )?;
    env.add_template(
        "snippets/https_redirect_primary_domain.conf.j2",
        include_str!("snippets/https_redirect_primary_domain.conf.j2"),
    )?;
    env.add_template(
        "snippets/generic_reverse_proxy.conf.j2",
        include_str!("snippets/generic_reverse_proxy.conf.j2"),
    )?;

    env.add_template(
        "templates/default.conf.j2",
        include_str!("templates/default.conf.j2"),
    )?;

    env.add_function(
        "invariant",
        |message: &str, args: Rest<minijinja::Value>| -> bool {
            panic!("template error: {message} {args:?}")
        },
    );

    env.add_function("acme_certs_dir", {
        let collected_acme_requests = Arc::clone(&collected_acme_requests);
        move |primary_domain: &str, additional_domains: Vec<String>| {
            let mut collected_acme_requests = collected_acme_requests.lock().unwrap();
            collected_acme_requests.insert(AcmeRequest {
                primary_domain: primary_domain.to_string(),
                additional_domains,
            });

            let base_dir = PathBuf::from("/etc/letsencrypt/live").join(primary_domain);
            AcmeCerts {
                certificate_pem: base_dir.join("fullchain.pem"),
                trusted_certificate_pem: base_dir.join("chain.pem"),
                certificate_key_pem: base_dir.join("privkey.pem"),
            }
        }
    });

    let mut sites_to_link: VecDeque<PathBuf> = Default::default();
    for site in sites {
        let site_seed = gen_site_seed(&site);
        let domain = site.domain.clone();
        let available_config_path = sites_available
            .join(&site.name)
            .join(format!("{domain}.conf"));

        let mut site_env = env.clone();
        site_env.add_global(
            "site_seed",
            minijinja::Value::from_safe_string(site_seed.clone()),
        );
        site_env.add_global("site", minijinja::Value::from(site));
        site_env.add_function("site_var", move |name: &str| {
            format!("$sitevar{site_seed}_{name}")
        });

        let site_template = site_env.get_template("templates/default.conf.j2")?;
        let (rendered, state) = site_template.render_and_return_state(context! {})?;

        info!(path = ?available_config_path, domain, "creating available site");
        println!("{}", rendered);
        // dbg!(state);

        sites_to_link.push_front(available_config_path);
    }

    drop(env);
    {
        let collected_acme_requests = Arc::into_inner(collected_acme_requests)
            .wrap_err("someone still references acme requests")?
            .into_inner()
            .wrap_err("lock poisoned")?;

        info!(?collected_acme_requests, "requested acme domains");
    }

    info!(paths = ?sites_to_link, "linking site configs");
    for available_site in sites_to_link {
        let link_target =
            sites_enabled.join(available_site.file_name().wrap_err("no filename in path")?);

        let relative_path =
            pathdiff::diff_paths(&available_site, &sites_enabled).wrap_err_with(|| {
                format!(
                    "unable to resolve relative path from {available_site:?} to {link_target:?}"
                )
            })?;

        info!("ln -svf {relative_path:?} {link_target:?}")
    }

    Ok(())
}
