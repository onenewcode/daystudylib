use std::sync::OnceLock;
static LOG_FILE_REGEX: OnceLock<String> = OnceLock::new();


fn main() {
    LOG_FILE_REGEX.set(String::from("DSd"));
    println!("{:?}", LOG_FILE_REGEX.get());
}
