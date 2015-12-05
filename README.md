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

    gitclass init name-of-class

A new directory with your class name now exists. If `name-of-class` is omitted, a the current directory will be used as the container for the class. You will be prompted for some information about the class, like a name, description, and master repository for the teacher's code.

### Adding students

    gitclass add-student [name] [repo]

If either `name` or `repo` is omitted, you will be prompted to enter a value.

### Pulling Student Repos
Perform this every time you want to ensure you have the latest commits from your students.

    gitclass update [name] [-n --num-commits] [-d --date-format]

This will update all repos if `[name]` is omitted, or a specific repo if stated. `--num-commits` and `--date-format` refer to the same options in Git. Defaults are `2` and `local`, respectively.

### Removing students

    gitclass remove-student name
