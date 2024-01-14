use chrono_tz::Europe::Helsinki;
use serde::{Deserialize, Serialize};
use chrono::TimeZone;
use chrono::{NaiveDateTime, DateTime, Utc};

pub const LOGIN_ERRORS : [&str; 10] = [
    "The latest terms and conditions have not been uploaded by Mitsubishi in your language. This is an error on our part. Please contact support.",
    "Please check your email address and password are both correct.",
    "You must verify your email address before logging in. You should have received an email message with a link to perform verification.",
    "Please contact administrator, your account has been disabled.",
    "We have sent you an email with a link to verify your email address. You must verify your email address to login.",
    "This version of MELCloud is no longer supported. Please download an updated version from the app store.",
    "Your account has temporarily been locked due to repeated attempts to login with incorrect password. It will be unlocked in %MINUTES% minute(s)",
    "Please re-enter the captcha",
    "Since you last logged in to MELCloud, we have made security improvements to the way we store user account information. As a consequence, we are no longer able to verify your current password. Please use the 'Forgotten Password' button below to reset your password.",
    "Due to high load on our servers, we are temporarily requesting you enter the code below in order to log in."
];

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LoginRequest {
    pub app_version: String,
    pub captcha_response: Option<String>,
    pub email: String,
    pub password: String,
    pub language: i32,
    pub persist: bool,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LoginResponse {
    pub error_id: Option<i32>,
    pub error_message: Option<String>,
    pub login_data: Option<LoginData>,
}

impl LoginResponse {
    pub fn has_error(&self) -> bool {
        self.error_id.is_some()
    }

    pub fn error_message(&self) -> String {
        if self.error_message.is_none() {
            let error_id = self.error_id.as_ref().unwrap();
            return LOGIN_ERRORS[*error_id as usize].to_string();
        }
        self.error_message.as_ref().unwrap().clone()
    }

    pub fn token(&self) -> String {
        if self.login_data.is_none() {
            panic!("No token found");
        }
        self.login_data.as_ref().unwrap().context_key.clone()
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LoginData {
    pub context_key: String,
    pub client: i32,
    pub duration: i32,
    pub expiry: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
#[allow(non_snake_case)]
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
        debug!("System Time UTC {}", naive_time.unwrap());

        Some(Utc.from_utc_datetime(&naive_time.unwrap()))
    }

    pub fn next_communication_to_utc_datetime(&self) -> Option<DateTime<Utc>> {
        let naive_time = NaiveDateTime::parse_from_str(&self.next_communication, "%Y-%m-%dT%H:%M:%S.%f");
        if naive_time.is_err() {
            return None;
        }
        debug!("System Time UTC {}", naive_time.unwrap());

        Some(Utc.from_utc_datetime(&naive_time.unwrap()))
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
#[allow(non_snake_case)]
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

#[derive(Debug, Serialize, Deserialize, Clone)]
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
        debug!("System Time UTC {}", naive_time.unwrap());

        Some(Utc.from_utc_datetime(&Helsinki.from_local_datetime(&naive_time.unwrap())
            .unwrap()
            .naive_utc()))
    }
}
