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

# Eval args
for x in "${@}"; do
    echo ">>> Setting '${x}'"
    eval "${x}"
done

subject="/C=${C}/ST=${ST}/L=${L}/O=${O}/CN=${CN}"
echo ">>> Certificate subject: ${subject}"
echo ">>> Certificate duration: ${duration}d"

openssl req -x509 -nodes -newkey rsa:4096 \
    -days "${duration}" \
    -subj "${subject}" \
    -keyout "${targetdir}"/selfsigned.key.pem -out "${targetdir}"/selfsigned.cert.pem
