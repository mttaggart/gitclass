# gitclass
Simple tool for managing student Git repositories.

## WARNING
This project works, but is a learning project for me.

## Getting Started
Let's download the tools:

    git clone https://github.com/mttaggart/gitclass

Or download from the Releases page.

Then enter the new folder. You can run the scripts from where they are, or copy the executable to a directory in your path, `PATH` for example:

    sudo cp gitclass /usr/local/bin/

### Creating a classroom

    gitclass init name-of-class

A new directory with your class name now exists. If `name-of-class` is omitted, a the current directory will be used as the container for the class. You will be prompted for some information about the class, like a name, description, and master repository for the teacher's code.

### Adding students

    gitclass add-student

This will prompt for `name` and `repo` for the student, then clone the given repo into a folder of the name provided.

### Pulling Student Repos
Perform this every time you want to ensure you have the latest commits from your students.

    gitclass update

This will update all repos.

**This feature is a little hacky as I figure out the `git2` library for Rust.**

### Removing students

    gitclass remove name
