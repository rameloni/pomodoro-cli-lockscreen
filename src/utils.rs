use std::path::PathBuf;

/// Return the path to the timer state file. This is the cache directory on Linux and LocalAppData on Windows.
/// In case the cache directory is not available, the current directory is used.
pub fn get_state_file() -> PathBuf {
    let mut path = dirs::cache_dir().unwrap_or_else(|| PathBuf::from("."));
    path.push("pomodoro-cli-info.json");
    path
}

/// The duration can be passed either as a number (as minutes) or as string in the format of "1h 30m 10s"
pub fn parse_duration(duration: &str) -> u64 {
    if let Ok(duration) = duration.parse::<u64>() {
        return duration * 60;
    }

    let mut duration = duration.to_lowercase();
    duration.retain(|c| !c.is_whitespace());
    duration = duration.replace("hour", "h");
    duration = duration.replace("minute", "m");
    duration = duration.replace("min", "m");
    duration = duration.replace("second", "s");
    duration = duration.replace("sec", "s");

    let mut hours = 0;
    let mut minutes = 0;
    let mut seconds = 0;
    if duration.contains("h") {
        duration.split("h");
        let parts = duration.split("h").collect::<Vec<&str>>();
        hours = parts[0].parse().unwrap_or_default();
        duration = parts[1].to_string();
    }
    if duration.contains("m") {
        duration.split("m");
        let parts = duration.split("m").collect::<Vec<&str>>();
        minutes = parts[0].parse().unwrap_or_default();
        duration = parts[1].to_string();
    }
    if duration.contains("s") {
        duration.split("s");
        let parts = duration.split("s").collect::<Vec<&str>>();
        seconds = parts[0].parse().unwrap_or_default();
    }
    hours * 60 * 60 + minutes * 60 + seconds
}

pub fn get_human_readable_time(seconds: u64) -> String {
    let mut seconds = seconds;
    let hours = seconds / 3600;
    seconds -= hours * 3600;
    let minutes = (seconds % 3600) / 60;
    seconds -= minutes * 60;

    let mut time = String::new();
    if hours > 0 {
        time.push_str(&format!("{}h ", hours));
    }
    if minutes > 0 {
        time.push_str(&format!("{}m ", minutes));
    }
    time.push_str(&format!("{}s", seconds));
    return time;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_duration() {
        assert_eq!(parse_duration("1h 30m 10s"), 5410);
        assert_eq!(parse_duration("1H 30Min 10SeC"), 5410);
        assert_eq!(parse_duration("2h15m1s"), 8101);
        assert_eq!(parse_duration("1h 30m"), 5400);
        assert_eq!(parse_duration("1hour"), 3600);
        assert_eq!(parse_duration("30m 10s"), 1810);
        assert_eq!(parse_duration("30m"), 1800);
        assert_eq!(parse_duration("10s"), 10);
        assert_eq!(parse_duration("100"), 100 * 60);
        assert_eq!(parse_duration("Invalid string"), 0);
    }

    #[test]
    fn test_get_human_readable_time() {
        assert_eq!(get_human_readable_time(5410), "1h 30m 10s");
        assert_eq!(get_human_readable_time(60), "1m 0s");
        assert_eq!(get_human_readable_time(10), "10s");
        assert_eq!(get_human_readable_time(0), "0s");
    }
}