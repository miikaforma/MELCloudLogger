use chrono_tz::Europe::Helsinki;
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
    pub set_fan_speed: u8,
    pub operation_mode: u8,
    pub vane_horizontal: u8,
    pub vane_vertical: u8,
    pub in_standby_mode: bool,
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

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ListDevicesResponse {
    pub iD: u32,
    pub structure: Structure,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Structure {
    pub devices: Vec<Devices>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
#[allow(non_snake_case)]
pub struct Devices {
    pub device_iD: u32,
    pub device_name: Option<String>,
    pub building_iD: u32,
    pub building_name: Option<String>,
    pub device: Device,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
#[allow(non_snake_case)]
pub struct Device {
    pub device_iD: u32,
    pub device_type: u8,
    pub power: bool,
    pub offline: bool,
    pub room_temperature: f32,
    pub set_temperature: f32,
    pub actual_fan_speed: u8,
    pub fan_speed: u8,
    pub automatic_fan_speed: Option<bool>,
    pub vane_vertical_direction: u8,
    pub vane_vertical_swing: Option<bool>,
    pub vane_horizontal_direction: u8,
    pub vane_horizontal_swing: Option<bool>,
    pub operation_mode: u8,
    pub in_standby_mode: bool,

    pub heating_energy_consumed_rate1: Option<f32>,
    pub heating_energy_consumed_rate2: Option<f32>,
    pub cooling_energy_consumed_rate1: Option<f32>,
    pub cooling_energy_consumed_rate2: Option<f32>,
    pub auto_energy_consumed_rate1: Option<f32>,
    pub auto_energy_consumed_rate2: Option<f32>,
    pub dry_energy_consumed_rate1: Option<f32>,
    pub dry_energy_consumed_rate2: Option<f32>,
    pub fan_energy_consumed_rate1: Option<f32>,
    pub fan_energy_consumed_rate2: Option<f32>,
    pub other_energy_consumed_rate1: Option<f32>,
    pub other_energy_consumed_rate2: Option<f32>,

    pub current_energy_consumed: Option<f32>,
    pub current_energy_mode: Option<u8>,
    pub energy_correction_model: Option<f32>,
    pub energy_correction_active: Option<bool>,

    pub wifi_signal_strength: Option<f32>,
    pub wifi_adapter_status: Option<String>,

    pub has_error: Option<bool>,

    pub last_time_stamp: String,
}

impl Device {
    pub fn last_time_stamp_to_utc_datetime(&self) -> Option<DateTime<Utc>> {
        let naive_time = NaiveDateTime::parse_from_str(&self.last_time_stamp, "%Y-%m-%dT%H:%M:%S");
        if naive_time.is_err() {
            return None;
        }
        // println!("System Time UTC {}", naive_time.unwrap());

        Some(Utc.from_utc_datetime(&Helsinki.from_local_datetime(&naive_time.unwrap())
            .unwrap()
            .naive_utc()))
    }
}
