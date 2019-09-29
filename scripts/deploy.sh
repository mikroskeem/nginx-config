#!/usr/bin/env bash
set -e

NGINX_DIR="/etc/nginx"

check_file () {
    local f="${1}"
    if [ ! -f "${f}" ]; then
        echo ">>> File '${f}' is not present, cannot continue the install!"
        exit 1
    fi
}

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
bash scripts/gen_ss.sh targetdir="${NGINX_DIR}/keys"
openssl dhparam -out "${NGINX_DIR}"/keys/dhparams.pem 4096
umask "${old_umask}"

# Symlink default config
echo ">>> Symlinking default vhost"
ln -s ../sites-available/default.conf "${NGINX_DIR}"/sites-enabled/default.conf

echo ""
echo ""
echo ">>> Initial setup done. You are free to continue setting up things by hand"
