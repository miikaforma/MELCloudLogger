use chrono::{DateTime, Utc};
use influxdb::InfluxDbWriteable;
use serde::{Deserialize, Serialize};

#[derive(Debug, InfluxDbWriteable, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct CurrentData {
    pub time: DateTime<Utc>,
    #[influxdb(tag)]
    pub device_id: u32,

    pub device_type: u8,
    pub power: bool,
    pub offline: bool,
    pub room_temperature: f32,
    pub set_temperature: f32,
    pub last_communication: String,

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
}
