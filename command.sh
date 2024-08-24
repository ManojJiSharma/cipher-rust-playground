#!/bin/bash

echo "Script started for building...".

if ! which rustc > /dev/null;then
    echo "rust unavailabl";
fi

cargo run;