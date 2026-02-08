#!/bin/bash
# Copyright (c) Mysten Labs, Inc.
# SPDX-License-Identifier: Apache-2.0

if ! cosign version &> /dev/null
then
    echo "cosign in not installed, Please install cosign for binary verification."
    echo "https://docs.sigstore.dev/cosign/installation"
    exit
fi

commit_sha=$1
pub_key=https://myso-private.s3.us-west-2.amazonaws.com/myso_security_release.pem
url=https://myso-releases.s3-accelerate.amazonaws.com/$commit_sha

echo "[+] Downloading myso binaries for $commit_sha ..."
curl $url/myso -o myso
curl $url/myso-node -o myso-node
curl $url/myso-tool -o myso-tool

echo "[+] Verifying myso binaries for $commit_sha ..."
cosign verify-blob --insecure-ignore-tlog --key $pub_key --signature $url/myso.sig myso
cosign verify-blob --insecure-ignore-tlog --key $pub_key --signature $url/myso-node.sig myso-node
cosign verify-blob --insecure-ignore-tlog --key $pub_key --signature $url/myso-tool.sig myso-tool
