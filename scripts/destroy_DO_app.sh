#!/usr/bin/env bash

# Run the doctl apps list command and capture the output
app_list_output=$(doctl apps list)

# Extract the app ID from the output using awk
app_id=$(echo "$app_list_output" | awk 'NR==2 {print $1}')

# Print the extracted app ID
echo "deleting: $app_id"

# doctl apps delete "$app_id"