#!/bin/bash


# Check if the "as on" date argument is provided
if [ $# -ne 1 ]; then
    echo "Usage: $0 <as_on_date>"
    exit 1
fi

# Assign the "as on" date argument to a variable
as_on_date="$2"

# Print the "as on" date
echo "As on date: $as_on_date"


