#!/usr/bin/env bash

# C  -> Country code
# ST -> State
# L  -> City
# O  -> Organization
# CN -> Common Name
C="xx"
ST="undefined"
L="undefined"
O="undefined"
CN="undefined"

targetdir="."
duration=3650
overwrite="no"

# Eval args
for x in "${@}"; do
    echo ">>> Setting '${x}'"
    eval "${x}"
done

privout="${targetdir}"/selfsigned.key.pem
certout="${targetdir}"/selfsigned.cert.pem

# Check whether keys already exist
if [ -f "${privout}" ] && [ -f "${certout}" ] && [ "${overwrite}" = "no" ]; then
    echo ">>> Keys already exist, won't overwrite"
    exit 2
fi

subject="/C=${C}/ST=${ST}/L=${L}/O=${O}/CN=${CN}"
echo ">>> Certificate subject: ${subject}"
echo ">>> Certificate duration: ${duration}d"

openssl req -x509 -nodes -newkey rsa:4096 \
    -days "${duration}" \
    -subj "${subject}" \
    -keyout "${privout}" -out "${certout}"
