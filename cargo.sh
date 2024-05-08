#!/usr/bin/bash

if [ $1 = "-t" ]; then
  cargo test $2 -- --show-output
fi
if [ $1 = "-l" ]; then
  \cd /home/bae/Projects/cdd/target/debug
  ln -sr ../../cdd_data.txt cdd_data.txt
fi
  
