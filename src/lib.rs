use std::fs;
use std::io;
use std::io::{Write,Read};
use std::path::PathBuf;
#[macro_use]
extern crate json;

struct Config {
    class_name: String,
    master_repo: String,
    description: String
}

struct Student {
    name: String,
    repo: String
}

pub fn new(name: &str) {
    match fs::create_dir(name) {
        Ok(v) => init(name),
        Err(e) => println!("Couldn't create: {}",e)
    }
}

pub fn init(path: &str) {
    println!("initalizing at {}", path);
    let config: Config = get_init_options();
    let config_json = object!{
        "className" => config.class_name.trim(),
        "description" => config.description.trim(),
        "masterRepo" => config.master_repo.trim(),
        "students" => json::JsonValue::new_array()
    };
    write_config(json::stringify_pretty(config_json,4), path)
}

pub fn add() {
    let mut f = fs::File::open("gitclass.json").unwrap();
    let mut config_str = String::new();
    f.read_to_string(&mut config_str);
    let mut config_json = json::parse(config_str.as_str()).unwrap();
    let student = get_student_details();
    let student_json = object!(
        "name" => student.name.trim(),
        "repo" => student.repo.trim()
    );
    config_json["students"].push(student_json);
    write_config(json::stringify_pretty(config_json,4), "./");
}

fn write_config(data: String, path: &str) {
    let mut init_path: PathBuf = PathBuf::from(path);
    init_path.push("gitclass.json");
    let mut f = fs::File::create(init_path.as_path()).unwrap();
    // let out_str = json::stringify_pretty(obj, 4);
    println!("{}",data);
    f.write(data.as_bytes());
}

fn get_init_options() -> Config {
    let mut class_name = String::new();
    println!("Enter your class's name: ");
    io::stdin().read_line(&mut class_name);
    let mut master_repo = String::new();
    println!("Enter the master repo: ");
    io::stdin().read_line(&mut master_repo);
    println!("Enter the description: ");
    let mut description = String::new();
    io::stdin().read_line(&mut description);
    return Config {
        class_name: class_name,
        master_repo: master_repo,
        description: description
    }
}

fn get_student_details() -> Student {
    let mut name = String::new();
    println!("Student name: ");
    io::stdin().read_line(&mut name);
    let mut repo = String::new();
    println!("Student repo: ");
    io::stdin().read_line(&mut repo);
    return Student {
        name: name,
        repo: repo
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
