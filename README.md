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
