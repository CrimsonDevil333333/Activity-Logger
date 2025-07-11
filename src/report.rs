use crate::config::Config;
use std::fs::{File};
use std::io::{self, Write, BufRead};
use std::path::Path;
use chrono::{Local, Duration};

/// Represents a summary report generator.
pub struct SummaryReporter<'a> {
    config: &'a Config,
}

impl<'a> SummaryReporter<'a> {
    pub fn new(config: &'a Config) -> Self {
        SummaryReporter { config }
    }

    /// Generates a summary report for the given interval (in days).
    /// This is a simple example that counts lines in the key and window log files.
    pub fn generate_report(&self) -> io::Result<()> {
        if !self.config.summary_report_enabled() {
            return Ok(());
        }

        let interval_days = self.config.summary_report_interval_days();
        let key_log_path = self.config.full_key_log_path();
        let window_log_path = self.config.active_window_log_path();

        let key_log_count = count_lines_since(&key_log_path, interval_days)?;
        let window_log_count = count_lines_since(&window_log_path, interval_days)?;

        let now = Local::now();
        let report = format!(
            "Activity Summary Report\n\
            Date: {}\n\
            Interval: Last {} day(s)\n\
            --------------------------\n\
            Key log entries: {}\n\
            Window log entries: {}\n",
            now.format("%Y-%m-%d %H:%M:%S"),
            interval_days,
            key_log_count,
            window_log_count
        );

        let report_file_name = self.config.summary_report_output_file()
            .map(|s| s.to_string())
            .unwrap_or_else(|| "activity_summary_report.txt".to_string());

        let log_dir = self.config.log_directory_path(); // Add this: get log directory from config
        let output_path = Path::new(&log_dir).join(report_file_name); // Join log dir with report file name

        let mut file = File::create(&output_path)?;
        file.write_all(report.as_bytes())?;

        Ok(())
    }
}

/// Counts the number of lines in a log file that were written in the last `interval_days`.
fn count_lines_since(path: &Path, interval_days: u32) -> io::Result<usize> {
    if !path.exists() {
        return Ok(0);
    }
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);
    let _since = Local::now() - Duration::days(interval_days as i64);

    let mut count = 0;
    for line in reader.lines() {
        if let Ok(_line) = line {
            // If your log lines start with a timestamp, you can parse and filter here.
            // For now, just count all lines.
            count += 1;
        }
    }

    Ok(count)
}