use std::env;
pub fn fetch_args() -> Vec<String>{
    let args: Vec<String> = env::args().collect();
    return args;
}
pub fn fetch_action(raw: &str) -> String {
    match raw {
        "/r" | "-r" | "/recursive" => String::from("r"),
        _ => String::from("n")
    }
}