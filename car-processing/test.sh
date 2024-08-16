#!/bin/bash
sdf clean
initial_command="sdf run --ephemeral"
new_command="fluvio consume speeding"

$initial_command > output.log 2>&1 &
initial_pid=$!

echo "Waiting for SDF to Start..."
tail -f output.log | while read line; do
    echo "$line"
    if [[ "$line" == *"Welcome to SDF"* ]]; then
        echo "Running SDF"
        $new_command
        pkill -P $$ tail
        break
    fi
done
