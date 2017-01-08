#!/bin/bash

compile() {
    if [[ $1 == *.lua ]]; then
        echo "compiling $1"
        base=$(echo $1 | cut -d "." -f1)
        cmd="luac -o $base $1"
        echo $cmd
        $cmd
        echo ""
    fi
}

for file in $(ls .); do
    compile $file
done