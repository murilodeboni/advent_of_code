#!/bin/bash

# README
# make it executable 
# $ chmod +x create_daily_files.sh
# cd into the folder and run it with desired date as the argument 
# $ ./create_daily_files.sh 21
# will create txt file in inputs folder for test and full input (day_xx.txt day_xx_test.txt)
# will create rs file (day_xx.rs)

# Check if the argument is provided
if [ -z "$1" ]; then
    echo "Usage: $0 <day_number (e.g., 01, 02)>"
    exit 1
fi

# Extract the day number from the argument
day_number="$1"

# Validate that day_number is numeric
if ! [[ "$day_number" =~ ^[0-9]{2}$ ]]; then
    echo "Error: day_number must be a two-digit numeric value (e.g., 01, 02)."
    exit 1
fi

# Define the file paths
inputs_folder="../inputs"
bin_folder="../"

test_file="$inputs_folder/day_${day_number}_test.txt"
input_file="$inputs_folder/day_${day_number}.txt"
rs_file="$bin_folder/day_${day_number}.rs"

# Create the inputs folder if it doesn't exist
mkdir -p "$inputs_folder" || exit 1

# Create the files
if ! touch "$test_file"; then
    echo "Error: Failed to create $test_file"
    exit 1
fi
if ! touch "$input_file"; then
    echo "Error: Failed to create $input_file"
    exit 1
fi
if ! touch "$rs_file"; then
    echo "Error: Failed to create $rs_file"
    exit 1
fi

# Provide feedback to the user
echo "Checking created files..."
[ -e "$test_file" ] && echo "$test_file" || echo "Error: Failed to create $test_file"
[ -e "$input_file" ] && echo "$input_file" || echo "Error: Failed to create $input_file"
[ -e "$rs_file" ] && echo "$rs_file" || echo "Error: Failed to create $rs_file"