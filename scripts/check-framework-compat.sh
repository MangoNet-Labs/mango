#!/bin/bash
# Copyright (c) MangoNet Labs, Inc.
# SPDX-License-Identifier: Apache-2.0
#
# Check whether the version of framework in the repo is compatible
# with the version on chain, as reported by the currently active
# environment, using the binary in environment variable $MGO.

set -e

MGO=${MGO:-mgo}
REPO=$(git rev-parse --show-toplevel)

for PACKAGE in "$REPO"/crates/mgo-framework/packages/*; do
    $MGO client verify-source "$PACKAGE"
done

