# gitclass
Simple tool for managing student Git repositories.

## WARNING
This project works, but is a learning project for me.

## Getting Started

### Installation
You can either download the compiled executable from the [releases](https://github.com/mttaggart/gitclass/releases)

Or build from source using Rust!

#### Install Rust

Follow the instructions [here](https://www.rust-lang.org) to install Rust, and [here](https://crates.io/install) to install Cargo, Rust's build system.

#### Cargo

From here, you can simply use Cargo to get the project:

    cargo install gitclass

This requires `~/.cargo` to be in your `PATH` to run.

#### Git

Clone the repo:

    git clone https://github.com/mttaggart/gitclass


Then enter the new folder. Use Cargo to build the project

    cargo build --release

This will compile the project on your platform. You can then copy the result to a directory known to your `PATH`. For example:

    sudo cp target/release/gitclass /usr/local/bin/


### Creating a classroom

    gitclass init 

This begins the process of creating a class. You'll be asking for a name, description, and master repository for the class. The name and description will be saved in `gitclass.json`, and the master repo will be cloned within the current folder. 

### Adding students

    gitclass add

This will prompt for `name` and `repo` for the student, then clone the given repo into a folder of the name provided.

### Pulling Student Repos
Perform this every time you want to ensure you have the latest commits from your students.

    gitclass update

This will update all repos.

**This feature is a little hacky as I figure out the `git2` library for Rust.**

### Removing students

    gitclass remove $name

### Getting information

    gitclass log $student

This will get the commit log from the master branch. Just vanilla git log for now. For more detailed info, run `git log` inside the students' folder.
