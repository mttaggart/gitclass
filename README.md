# git-classroom
Simple tools for managing student Git repositories.

This is a collection of scripts I use to create and manage my Computer Science courses using Git and GitHub. I hope they're useful to you. Pull requests welcome!

## Getting Started
Let's download the tools:

    git clone https://github.com/mttaggart/git-classroom

Or download from the Releases page.

Then enter the new folder. You can run the scripts from where they are, or run `install` (as root) to install the files.

    sudo ./install

### Creating a classroom

    gitclass --create name-of-class

A new directory with your class name now exists.

### Adding students

    gitclass --class /path/to/class/folder --add student-name
    Enter the URL for student-name's repository: https://github.com/student/repo
    Cloning repository for student-name...
    Done.

### Pulling Student Repos
Perform this every time you want to ensure you have the latest commits from your students.

    gitclass --class /path/to/class/folder --pullall
    
