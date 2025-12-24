use std::fs::{OpenOptions};
use std::io::Write;
use chrono::Local;

pub fn log_alert(msg: &str) {
    let path = dirs::data_dir()
        .unwrap()
        .join("sysoracle/alerts.log");

    let _ = std::fs::create_dir_all(path.parent().unwrap());

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)
        .unwrap();

    let time = Local::now().format("%Y-%m-%d %H:%M:%S");
    let _ = writeln!(file, "[{}] {}", time, msg);
}
