use std::fs;
use std::io;
use std::io::{Write,Read};
use std::path::PathBuf;
#[macro_use]
extern crate json;
extern crate git2;
extern crate chrono;
use git2::Repository;
use chrono::{NaiveDateTime, NaiveDate};

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
        Ok(_) => init(name),
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
        "students" => json::JsonValue::new_object()
    };
    write_config_json(json::stringify_pretty(config_json,4), path)
}

pub fn add() {
    let mut config_json = get_config_json().unwrap();
    let student = get_student_details();
    let student_json = object!(
        "name" => student.name.trim(),
        "repo" => student.repo.trim()
    );
    config_json["students"][student.name.trim()] = student_json;
    write_config_json(json::stringify_pretty(config_json,4), "./");
    clone_repo(student.repo.as_str(), student.name.as_str());
}

pub fn remove(name: &str) {
    let mut config_json = get_config_json().unwrap();
    if config_json["students"].has_key(name) {
        println!("Removing {}", name);
        config_json["students"].remove(name);
        match fs::remove_dir_all(name) {
            Ok(_) => println!("Removed {}",name),
            Err(e) => panic!("Couldn't remove!: {}",e)
        };
        write_config_json(json::stringify_pretty(config_json,4), "./");
    } else {
        println!("Student {} not known.",name);
    }
}

pub fn update() {
    println!("Updating student repos");
    let config_json = get_config_json().unwrap();
    let students = config_json["students"].entries();
    for s in students {
        println!("Updating {}",s.0);
        update_repo(s.1.clone());
    }
}

fn print_commits(revwalk: git2::Revwalk, repo: &git2::Repository) {
    for c in revwalk {
        let commit = repo.find_commit(c).unwrap();
        let time = commit.time();
        let seconds = time.seconds().to_string();
        let offset = time.offset_minutes().to_string();
        let d = match NaiveDateTime::parse_from_str(seconds.as_str(),"%s") {
            Ok(d) => d,
            Err(e) => panic!("{}",e)
        } ;
        println!("{}:\n\t {}",d, commit.message().unwrap());
    }
}

pub fn log(name: &str) {
    println!("Logging {}", name);
    let config_json = get_config_json().unwrap();
    if config_json["students"].has_key(name) {
        let repo = Repository::open(name).unwrap();
        let mut revwalk = repo.revwalk().unwrap();
        match revwalk.push_head() {
            Ok(_) => print_commits(revwalk, &repo),
            Err(e) => panic!("{}",e)
        }
    } else {
        println!("Student {} not known.", name);
    }
}

fn write_config_json(data: String, path: &str) {
    let mut init_path: PathBuf = PathBuf::from(path);
    init_path.push("gitclass.json");
    let mut f = fs::File::create(init_path.as_path()).unwrap();
    // let out_str = json::stringify_pretty(obj, 4);
    println!("{}",data);
    match f.write(data.as_bytes()) {
        Ok(_) => println!("Config updated"),
        Err(e) => panic!("Config not updated: {}",e)
    };
}

fn get_config_json() -> Result<json::JsonValue, json::Error> {
    let mut f = fs::File::open("gitclass.json").unwrap();
    let mut config_str = String::new();
    match f.read_to_string(&mut config_str) {
        Ok(_) => println!("Config loaded"),
        Err(e) => panic!("Couldn't load config: {}",e)
    };
    json::parse(config_str.as_str())
}

fn get_init_options() -> Config {
    let mut class_name = String::new();
    println!("Enter your class's name: ");
    match io::stdin().read_line(&mut class_name) {
        Ok(_) => {},
        Err(e) => panic!("Couldn't read name: {}",e)
    };
    let mut master_repo = String::new();
    println!("Enter the master repo: ");
    match io::stdin().read_line(&mut master_repo) {
        Ok(_) => {},
        Err(e) => panic!("Couldn't read repo: {}",e)
    };
    println!("Enter the description: ");
    let mut description = String::new();
    match io::stdin().read_line(&mut description) {
        Ok(_) => {},
        Err(e) => panic!("Couldn't read description: {}",e)
    };
    return Config {
        class_name: class_name,
        master_repo: master_repo,
        description: description
    }
}

fn get_student_details() -> Student {
    let mut name = String::new();
    println!("Student name: ");
    match io::stdin().read_line(&mut name) {
        Ok(_) => {},
        Err(e) => panic!("Couldn't read name: {}",e)
    };
    let mut repo = String::new();
    println!("Student repo: ");
    match io::stdin().read_line(&mut repo) {
        Ok(_) => {},
        Err(e) => panic!("Couldn't read repo: {}",e)
    };
    return Student {
        name: name,
        repo: repo
    }
}

fn clone_repo(url: &str, path: &str) -> git2::Repository {
    match Repository::clone(url.trim(), path.trim()) {
        Ok(repo) => repo,
        Err(e) => panic!("failed to clone: {}", e),
    }
}

fn update_repo(student: json::JsonValue) {
    let path = student["name"].as_str().unwrap();
    let url = student["repo"].as_str().unwrap();
    match fs::remove_dir_all(path) {
        Ok(_) => {},
        Err(e) => panic!("Couldn't remove: {}",e)
    };
    clone_repo(url, path);
}
