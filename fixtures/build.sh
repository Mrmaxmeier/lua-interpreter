#!/bin/bash

compile() {
    if [[ $1 == *.lua ]]; then
        echo "compiling $1"
        # TODO: handle files like "a.b.lua"
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