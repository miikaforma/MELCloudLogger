use serde::{Deserialize, Serialize};
use chrono::TimeZone;
use chrono::{NaiveDateTime, DateTime, Utc};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CurrentDataResponse {
    pub device_iD: u32,
    pub device_type: u8,
    pub power: bool,
    pub offline: bool,
    pub room_temperature: f32,
    pub set_temperature: f32,
    pub has_pending_command: bool,
    pub last_communication: String,
    pub next_communication: String,
}

impl CurrentDataResponse {
    pub fn last_communication_to_utc_datetime(&self) -> Option<DateTime<Utc>> {
        let naive_time = NaiveDateTime::parse_from_str(&self.last_communication, "%Y-%m-%dT%H:%M:%S.%f");
        if naive_time.is_err() {
            return None;
        }
        // println!("System Time UTC {}", naive_time.unwrap());

        Some(Utc.from_utc_datetime(&naive_time.unwrap()))
    }

    pub fn next_communication_to_utc_datetime(&self) -> Option<DateTime<Utc>> {
        let naive_time = NaiveDateTime::parse_from_str(&self.next_communication, "%Y-%m-%dT%H:%M:%S.%f");
        if naive_time.is_err() {
            return None;
        }
        // println!("System Time UTC {}", naive_time.unwrap());

        Some(Utc.from_utc_datetime(&naive_time.unwrap()))
    }
}
