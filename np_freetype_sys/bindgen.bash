#!/usr/bin/env bash
set -efuo pipefail
cd np_freetype_sys
exec bindgen --no-layout-tests --no-prepend-enum-name bindgen.h > bindgen.rs
