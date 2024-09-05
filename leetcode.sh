#!/bin/bash

# Check if two arguments are provided
if [ "$#" -ne 2 ]; then
    echo "Usage: leetcode <problem_no> <problem_title>"
    exit 1
fi

# Get the problem number and problem title from arguments
problem_no=$1
problem_title=$2

# Create folder name using the format: (problem_no)_(problem_title)
folder_name="${problem_no}_${problem_title}"

# Create the folder with the specified name
mkdir "$folder_name"

# Navigate into the folder
cd "$folder_name" || exit

# Initialize a new Cargo project with the binary name: leetcode_(problem_no)
cargo init --name "leetcode_${problem_no}" --bin .

echo "LeetCode project 'leetcode_${problem_no}' initialized in folder '$folder_name'."
