use std::env;

fn main() {
    let key = "PATH";
    match env::var_os(key) {
        Some(val) => {
            let path_variable: String = match val.into_string() {
                Ok(string) => string,
                Err(_) => panic!("failure"),
            };
            let paths = path_variable.split(":");
            for path in paths {
                println!("{}", path)
            }
        },
        None => println!("{} is not defined in this environment", key)
    }
}
