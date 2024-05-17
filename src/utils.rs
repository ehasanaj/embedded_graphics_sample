use chrono::Local;

pub fn get_time() -> String {
    let now = Local::now();
    now.format("%H:%M:%S").to_string()
}
