#!/bin/sh -e

. $(dirname $(dirname $0))/functions.sh

# --- Check versions ---------------------------------------------------------

log Checking versions in package metadata
python3 ci/vercheck.py
