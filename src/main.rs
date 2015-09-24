use std::env;
use std::ffi::OsString;

fn print_path(path_variable: OsString) {
    let path_string: String = match path_variable.into_string() {
        Ok(string) => string,
        Err(_) => panic!("failure"),
    };
    let paths = path_string.split(":");
    for path in paths {
        println!("{}", path)
    }
}

fn main() {
    let key = "PATH";
    match env::var_os(key) {
        Some(val) => print_path(val),
        None => println!("{} is not defined in this environment", key)
    }
}
