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

# Define color codes
GREEN="\033[0;32m"
YELLOW="\033[1;33m"
NC="\033[0m" # No Color (resets color to default)

printf "${GREEN}LeetCode project '${YELLOW}leetcode_${problem_no}${GREEN}' initialized in folder '${YELLOW}$folder_name${GREEN}'.${NC}\n"


#Initialize a tests.rs file inside the project's /src dir

# 1. CD into the dir
cd "src" || exit

# 2. Create the tests.rs file
FILE_NAME="tests.rs"

# 3. Add the boilerplate code.
cat <<EOL > $FILE_NAME
#![allow(unused)]
use crate::*;

#[test]
fn test1() {
    // Add test here
}

#[test]
fn test2() {
    // Add test here
}

#[test]
fn test3() {
    // Add test here
}
EOL

# Get the current directory
CURRENT_DIR=$(pwd)

# Notify the user with the current directory, using colors
printf "${GREEN}Created ${YELLOW}$FILE_NAME${GREEN} in the ${YELLOW}$CURRENT_DIR${GREEN} directory.${NC}\n"


# Add "mod tests;" in main.rs line 1
MAIN_RS_FILE="main.rs"

# insert "mod tests;" at the first line (portable way)
{ echo "mod tests;"; cat "$MAIN_RS_FILE"; } > "$MAIN_RS_FILE.tmp" && mv "$MAIN_RS_FILE.tmp" "$MAIN_RS_FILE"

# append "pub struct Solution;" to the end
echo "pub struct Solution;" >> "$MAIN_RS_FILE"

# Notify the user that the line has been added
printf "${GREEN}Added ${YELLOW}mod tests;${GREEN} and ${YELLOW}pub struct Solution;${GREEN} to ${YELLOW}$MAIN_RS_FILE${GREEN}.${NC}\n"

cat "$MAIN_RS_FILE"