
# About this repo
Welcome to my ***LeetCode Practice*** repository! This repo is designed to help me keep track of my progress on LeetCode problems over time. Currently, it contains Rust solutions for various LeetCode problems.

### This repo contains
- `leetcode.sh`: A reusable bash script for creating new LeetCode project folders and initializing Cargo projects.
- ***Rust Solutions***: Contains Rust code for solving LeetCode problems.
    - ***Automated Test Cases***: Each project includes automated test cases to verify the correctness of the solutions.

# LeetCode Bash Script

This script automates the process of creating a folder for a LeetCode problem, navigating into that folder, and initializing a new Cargo project with a specific binary file name.

## Prerequisites

Before using the script, ensure the following:
- You are using **Windows 11** with **WSL (Windows Subsystem for Linux)** or **Git Bash**.
- Rust and Cargo are installed in your environment.
- The script file has Unix line endings (LF). If you encounter issues, refer to the [Fixing Line Endings](#fixing-line-endings) section.

## **Script Usage**

### Option 1: Copy the script to Git's bin directory
1. Copy the file and paste it into this following directory ```..\Git\usr\bin```.
2. **Open Terminal with Admin Privilage**
3. Navigate to the folder where your script is saved:
    ```bash 
        cd path/to/repo
    ```
4. **Use the following command** to copy the script to Git's bin directory:
    ``` bash
        # for Windows
        Copy-Item ./leetcode.sh "C:/Program Files/Git/usr/bin/" -Force

        # //TODO: for linux 
    ```
5. The script is now globally accessible from any terminal.
    ```bash
    leetcode.sh <problem_no> <problem_title>
    ```



### Option 2: Save the script in your desired directory
1. Or Save the script as `leetcode.sh` in your desired directory. 

2. Open **WSL** or **Git Bash**.
3. Make the script executable:
```bash
chmod +x leetcode.sh
```
4. Navigate to the folder where the script is saved.
```bash
cd path/to/repo
```

### Command Structure

```bash
./leetcode.sh <problem_no> <problem_title>
```

 ***Arguments***
- `<problem_no>`: The number the problem from leetcode.
- `<problem_title>`: The title of the problem mentioned in leetcode. 

### Example
To create a folder `001_leetcode` and initialize a Cargo project with the binary file `leetcode_001`, run:
```bash
./leetcode.sh 001 problem-title`
```

## Fixing Line Endings
If you encounter the error:
```bash
-bash: ./leetcode.sh: /bin/bash^M: bad interpreter: No such file or directory
```

This happens because the script file has Windows-style line endings (CRLF). To fix this, convert the file to Unix-style line endings (LF):

### Option 1: Using dos2unix in WSL
1. Install dos2unix (if not installed):
```bash 
sudo apt-get install dos2unix
```

2. Convert the script:
```bash
dos2unix leetcode.sh
```
3. Re-run the script.

### Option 2: Using Visual Studio Code or Notepad++
- Visual Studio Code:

    1. Open the script in VS Code.
    2. Click on `CRLF` in the bottom-right corner.
    3. Select `LF` and save the file.

- Notepad++:

    1. Open the script in Notepad++.
    2. Go to `Edit > EOL Conversion`.
    3. Select `Unix (LF)` and save the file.
    4. Now you can run the script without errors.