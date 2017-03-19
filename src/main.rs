#[macro_use]
extern crate clap;
extern crate gitclass;
use gitclass::*;
use clap::App;

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();
    if let Some(new_m) = matches.subcommand_matches("new") {
        match new_m.value_of("DIR") {
            Some(p) => new(p),
            _ => panic!("Need a directory, idiot")
        };
    }
    if let Some(init_m) = matches.subcommand_matches("init") {
        init("./");
    }
    if let Some(add_m) = matches.subcommand_matches("add") {
        add();
    }
    if let Some(rem_m) = matches.subcommand_matches("remove") {
        match rem_m.value_of("STUDENT") {
            Some(s) => remove(s),
            _ => panic!("No student provided")
        }
    }
    if let Some(up_m) = matches.subcommand_matches("update") {
        update();
    }
    if let Some(log_m) = matches.subcommand_matches("log") {
        let options: &str;
        match log_m.value_of("LOGOPTIONS") {
            Some(o) => options = o,
            _ => options = ""
        }
        println!("{}",options);
        match log_m.value_of("STUDENT") {
            Some(s) => log(s, options),
            _ => panic!("No student provided")
        }
    }
}
