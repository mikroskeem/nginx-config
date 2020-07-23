#!/bin/sh
set -e

NGINX_DIR="${NGINX_DIR:-/etc/nginx}"

check_file () {
    f="${1}"
    printf ">>> Checking if '%s' exists... " "${f}"
    if [ ! -f "${f}" ]; then
        echo ""
        echo "File '${f}' is not present!"
        echo ">>> Cannot continue the install"
        exit 1
    else
        echo "OK"
    fi
}

printf ">>> nginx configuration directory is set to '%s'" "${NGINX_DIR}"
if [ ! -d "${NGINX_DIR}" ]; then
    echo "... which does not exist!"
    echo ">>> Make sure you specified the correct directory"
    exit 1
else
    echo ""
fi

# Check for required files
check_file "${NGINX_DIR}"/mime.types
check_file "${NGINX_DIR}"/fastcgi_params

# Set up base files
echo ">>> Copying base files"
cp -rv nginx.conf extra_mime.types sites-available/ snippets/ \
    "${NGINX_DIR}"

echo ">>> Installing required directories"
install -d -m 0755 "${NGINX_DIR}"/sites-enabled
install -d -m 0700 "${NGINX_DIR}"/keys

# Generate keys
echo ">>> Generating selfsigned keys and dhparams"
old_umask="$(umask)"
umask 0077
bash scripts/gen_ss.sh targetdir="${NGINX_DIR}/keys" || { [ ! "${?}" -eq 2 ] && echo ">>> Failed to generate keys, cannot continue the install." && exit 1; }
if [ -f "${NGINX_DIR}"/keys/dhparams.pem ]; then
    # dhparams generation takes a lot of time, skip if possible but with a warning
    echo ">>> WARNING: Using existing '${NGINX_DIR}/keys/dhparams.pem', regenerate manually using"
    echo ">>> 'openssl dhparam -out \"${NGINX_DIR}/keys/dhparams.pem\" 4096' if you want to recreate it"
else
    openssl dhparam -out "${NGINX_DIR}"/keys/dhparams.pem 4096
fi
umask "${old_umask}"

# Symlink default config
echo ">>> Symlinking default vhost"
if [ -f "${NGINX_DIR}"/sites-enabled/default.conf ]; then
    if [ ! -L "${NGINX_DIR}"/sites-enabled/default.conf ] || [ ! "$(readlink "${NGINX_DIR}"/sites-enabled/default.conf)" = "../sites-available/default.conf" ]; then
        echo ">>> File already present, please check '${NGINX_DIR}/sites-enabled/default.conf' manually"
    fi
else
    ln -s ../sites-available/default.conf "${NGINX_DIR}"/sites-enabled/default.conf
fi

echo ""
echo ""
echo ">>> Initial setup done. You are free to continue setting up things by hand"
echo ">>> You might also want to check out 'deploy_certbot.sh'"
