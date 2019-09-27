# Let's Encrypt certbot support

Avoid certbot's nginx plugin at any cost, it does not play nice with nginx configuration.

## Setup

### Packages
On Debian and Arch, you can freely install `certbot` package

Don't forget to enable `certbot.timer` (if not already enabled)

### Changes to service file

As we're going to use webroot plugin, then certbot is not aware what it has to do after obtaining new certificates, so you need
to do it yourself.

Run `sudo systemctl edit certbot.service` and insert this:

```ini
[Service]
ExecStopPost=/bin/systemctl --no-block reload nginx.service
```

(If you have custom nginx build/service file name, use that instead)

That makes sure that nginx loads new certificates.

### Paths on filesystem

You need to create `/var/www/certbot`. It can be owned by root or whatever user, as long as it can be read by web server user.

### Fetching certificates

I use following script:

```sh
sudo certbot certonly -n -m <your email address here> --agree-tos --webroot --webroot-path /var/www/certbot --domains ${@}
```

### Setting up nginx configuration

`default.conf` in this repository provides working certbot configuration suitable for `http-01` verification (TODO: verify that this info is correct?),
thus all requested certificates will be accepted out of box.

### Cloudflare proxying

If you want (you probably should) http-\>https redirect, then do that with nginx instead of Cloudflare. Also make sure that you use `Full (strict)`
HTTPS certificate setup
