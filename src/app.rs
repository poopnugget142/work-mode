use std::{fs, str::FromStr, time::Duration};

use chrono::{DateTime, Datelike, Local, Weekday};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct Settings {
    pub work_time: u64,
    pub blocked_sites: Vec<String>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Save {
    pub detox: bool,
    pub time_started: Option<String>,
    pub last_completion: Option<String>
}

#[derive(PartialEq)]
pub enum WorkStatus {
    Check,
    Start,
    Working,
    Break,
    Complete,
    Weekend,
}

pub struct App {
    pub work_time: Duration,
    pub time_started: Option<DateTime<Local>>,
    pub status: WorkStatus,
    pub settings: Settings,
    pub save: Save,
}

impl App {
    pub fn new() -> App {
        // Read the settings data
        let data = fs::read_to_string("Settings.toml").expect("Unable to read file");
        let settings: Settings = toml::from_str(&data).unwrap();

        // Read our save data
        let data = fs::read_to_string("Save.toml").expect("Unable to read file");
        let save: Save = toml::from_str(&data).expect("Unable to decode file");

        let mut starting_status;
        let mut time_started: Option<DateTime<Local>> = None;

        if save.detox {
            starting_status = WorkStatus::Working;

            match &save.time_started {
                Some(saved_time_started) => {
                    time_started = Some(DateTime::<Local>::from_str(&saved_time_started).expect("Unable to parse string"));
                }
                None => {}
            }
        } else {
            // Only copy to backup when detox is not active
            // Update our backup to have most up to date networking data
            let data = fs::read_to_string("/etc/hosts").expect("Unable to read hosts file");
            fs::write("hosts.backup", data).expect("Unable to write file");

            starting_status = WorkStatus::Check;
        }

        match &save.last_completion {
            Some(last_completion) => {
                let last_date = DateTime::<Local>::from_str(&last_completion).expect("Unable to convert saved completion date string into valid date");

                if last_date.date_naive() == Local::now().date_naive() {
                    starting_status = WorkStatus::Complete;
                }
            }
            None => {}
        }

        let weekday = Local::now().weekday();
        if weekday == Weekday::Sat || weekday == Weekday::Sun {
            starting_status = WorkStatus::Weekend;
        }

        App {
            work_time: Duration::from_secs(settings.work_time),
            time_started,
            status: starting_status,
            settings: settings,
            save: save,
        }
    }

    pub fn save(&mut self) {
        let data = toml::to_string(&self.save).unwrap();
        fs::write("Save.toml", data).expect("Unable to write file");
    }
}