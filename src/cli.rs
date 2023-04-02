use std::env;

pub fn get_path() -> String {
    let cwd = env::current_dir()
        .unwrap()
        .to_str()
        .expect("parseable cwd")
        .to_string();
    return format!("file:///{}/test/vocab.sqlite", cwd);
}
