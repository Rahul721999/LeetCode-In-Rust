#!/bin/bash

# Define the file name
FILE_NAME="tests.rs"

# Create the tests.rs file and add the content
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

# Define color codes
GREEN="\033[0;32m"
YELLOW="\033[1;33m"
NC="\033[0m" # No Color (resets color to default)

# Notify the user with the current directory, using colors
echo -e "${GREEN}Created ${YELLOW}$FILE_NAME${GREEN} in the ${YELLOW}$CURRENT_DIR${GREEN} directory.${NC}"
