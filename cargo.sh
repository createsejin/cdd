#!/usr/bin/bash

if [ $1 = "-t" ]; then
  cargo test $2 -- --show-output
fi
