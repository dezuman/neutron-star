#!/bin/sh

RUST_TARGET_PATH=`pwd` xargo build --target i486-neutron "$@"