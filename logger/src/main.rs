use std::time::Duration;

use api::ListDevicesResponse;
use api::current_data;
use api::listdevices_data;
use api::request_refresh;
use api::CurrentDataResponse;
use chrono::{DateTime, Utc};
use dotenv::dotenv;
use influxdb::Client;
use influxdb::InfluxDbWriteable;
use tokio::time::sleep;

#[derive(InfluxDbWriteable)]
#[allow(non_snake_case)]
struct CurrentData {
    time: DateTime<Utc>,
    #[influxdb(tag)]
    device_id: u32,

    device_type: u8,
    power: bool,
    offline: bool,
    room_temperature: f32,
    set_temperature: f32,
    last_communication: String,

    actual_fan_speed: u8,
    fan_speed: u8,
    automatic_fan_speed: Option<bool>,
    vane_vertical_direction: u8,
    vane_vertical_swing: Option<bool>,
    vane_horizontal_direction: u8,
    vane_horizontal_swing: Option<bool>,
    operation_mode: u8,
    in_standby_mode: bool,

    heating_energy_consumed_rate1: Option<f32>,
    heating_energy_consumed_rate2: Option<f32>,
    cooling_energy_consumed_rate1: Option<f32>,
    cooling_energy_consumed_rate2: Option<f32>,
    auto_energy_consumed_rate1: Option<f32>,
    auto_energy_consumed_rate2: Option<f32>,
    dry_energy_consumed_rate1: Option<f32>,
    dry_energy_consumed_rate2: Option<f32>,
    fan_energy_consumed_rate1: Option<f32>,
    fan_energy_consumed_rate2: Option<f32>,
    other_energy_consumed_rate1: Option<f32>,
    other_energy_consumed_rate2: Option<f32>,

    current_energy_consumed: Option<f32>,
    current_energy_mode: Option<u8>,
    energy_correction_model: Option<f32>,
    energy_correction_active: Option<bool>,

    wifi_signal_strength: Option<f32>,
    wifi_adapter_status: Option<String>,

    has_error: Option<bool>,
}

async fn fetch_and_log_new_entry(
    client: &Client,
    access_token: String,
    device_id: String,
    building_id: String,
) {
    println!("Logging new entry for device {}", &device_id);

    match listdevices_data(&access_token).await {
        Ok(data) => log_new_device_list_entry(client, &data).await,
        Err(_) => match current_data(&access_token, &device_id, &building_id).await {
            Ok(data) => log_new_current_data_entry(client, &data).await,
            Err(_) => {}
        },
    }
}

async fn log_new_device_list_entry(client: &Client, data: &Vec<ListDevicesResponse>) {
    let list_devices = &data[0];
    let structure = &list_devices.structure;
    let devices = &structure.devices;
    let device = &devices[0].device;

    let system_time = device.last_time_stamp_to_utc_datetime();

    if system_time.is_none() {
        println!("Skipping logging because system time couldn't be parsed");
        return;
    }

    let system_time = system_time.unwrap();
    println!("System Time UTC: {:?}", system_time);
    let current_data = CurrentData {
        time: system_time,

        device_id: device.device_iD,

        device_type: device.device_type,
        power: device.power,
        offline: device.offline,
        room_temperature: device.room_temperature,
        set_temperature: device.set_temperature,
        last_communication: device.last_time_stamp.clone(),

        actual_fan_speed: device.actual_fan_speed,
        fan_speed: device.fan_speed,
        automatic_fan_speed: device.automatic_fan_speed,
        vane_vertical_direction: device.vane_vertical_direction,
        vane_vertical_swing: device.vane_vertical_swing,
        vane_horizontal_direction: device.vane_horizontal_direction,
        vane_horizontal_swing: device.vane_horizontal_swing,
        operation_mode: device.operation_mode,
        in_standby_mode: device.in_standby_mode,
    
        heating_energy_consumed_rate1: device.heating_energy_consumed_rate1,
        heating_energy_consumed_rate2: device.heating_energy_consumed_rate2,
        cooling_energy_consumed_rate1: device.cooling_energy_consumed_rate1,
        cooling_energy_consumed_rate2: device.cooling_energy_consumed_rate2,
        auto_energy_consumed_rate1: device.auto_energy_consumed_rate1,
        auto_energy_consumed_rate2: device.auto_energy_consumed_rate2,
        dry_energy_consumed_rate1: device.dry_energy_consumed_rate1,
        dry_energy_consumed_rate2: device.dry_energy_consumed_rate2,
        fan_energy_consumed_rate1: device.fan_energy_consumed_rate1,
        fan_energy_consumed_rate2: device.fan_energy_consumed_rate2,
        other_energy_consumed_rate1: device.other_energy_consumed_rate1,
        other_energy_consumed_rate2: device.other_energy_consumed_rate2,
    
        current_energy_consumed: device.current_energy_consumed,
        current_energy_mode: device.current_energy_mode,
        energy_correction_model: device.energy_correction_model,
        energy_correction_active: device.energy_correction_active,
    
        wifi_signal_strength: device.wifi_signal_strength,
        wifi_adapter_status: device.wifi_adapter_status.clone(),
    
        has_error: device.has_error,
    };

    let write_result = client
        .query(&current_data.into_query("melCloudDeviceData"))
        .await;
    if let Err(err) = write_result {
        eprintln!("Error writing to db: {}", err)
    }
}

async fn log_new_current_data_entry(client: &Client, data: &CurrentDataResponse) {
    let system_time = data.last_communication_to_utc_datetime();

    if system_time.is_none() {
        println!("Skipping logging because system time couldn't be parsed");
        return;
    }

    let system_time = system_time.unwrap();
    println!("System Time UTC: {:?}", system_time);
    let current_data = CurrentData {
        time: system_time,

        device_id: data.device_iD,

        device_type: data.device_type,
        power: data.power,
        offline: data.offline,
        room_temperature: data.room_temperature,
        set_temperature: data.set_temperature,
        last_communication: data.last_communication.clone(),

        actual_fan_speed: data.set_fan_speed,
        fan_speed: data.set_fan_speed,
        automatic_fan_speed: None,
        vane_vertical_direction: data.vane_vertical,
        vane_vertical_swing: None,
        vane_horizontal_direction: data.vane_horizontal,
        vane_horizontal_swing: None,
        operation_mode: data.operation_mode,
        in_standby_mode: data.in_standby_mode,
    
        heating_energy_consumed_rate1: None,
        heating_energy_consumed_rate2: None,
        cooling_energy_consumed_rate1: None,
        cooling_energy_consumed_rate2: None,
        auto_energy_consumed_rate1: None,
        auto_energy_consumed_rate2: None,
        dry_energy_consumed_rate1: None,
        dry_energy_consumed_rate2: None,
        fan_energy_consumed_rate1: None,
        fan_energy_consumed_rate2: None,
        other_energy_consumed_rate1: None,
        other_energy_consumed_rate2: None,
    
        current_energy_consumed: None,
        current_energy_mode: None,
        energy_correction_model: None,
        energy_correction_active: None,
    
        wifi_signal_strength: None,
        wifi_adapter_status: None,
    
        has_error: None,
    };

    let write_result = client
        .query(&current_data.into_query("melCloudDeviceData"))
        .await;
    if let Err(err) = write_result {
        eprintln!("Error writing to db: {}", err)
    }
}

async fn refresh_device(access_token: String, device_id: String) -> bool {
    println!("Refreshing the device {}", &device_id);

    match request_refresh(access_token, device_id).await {
        Ok(data) => true,
        Err(err) => {
            println!("Failed to request a device refresh {}", err);
            false
        }
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let database_url = dotenv::var("DATABASE_URL").unwrap_or("http://localhost:8086".to_string());
    let database_name = dotenv::var("DATABASE_NAME").unwrap_or("melcloud".to_string());
    let refresh_interval: u64 = dotenv::var("REFRESH_INTERVAL")
        .map(|var| var.parse::<u64>())
        .unwrap_or(Ok(60_000))
        .unwrap();

    let fetch_interval: u64 = dotenv::var("FETCH_INTERVAL")
        .map(|var| var.parse::<u64>())
        .unwrap_or(Ok(10_000))
        .unwrap();

    let building_id = dotenv::var("BUILDING_ID").unwrap();
    let device_id = dotenv::var("DEVICE_ID").unwrap();
    let access_token = dotenv::var("ACCESS_TOKEN").unwrap();

    // Connect to database
    let client = Client::new(database_url, database_name);

    loop {
        refresh_device(access_token.to_string(), device_id.clone()).await;
        sleep(Duration::from_millis(fetch_interval)).await;
        fetch_and_log_new_entry(
            &client,
            access_token.to_string(),
            device_id.clone(),
            building_id.clone(),
        )
        .await;
        sleep(Duration::from_millis(refresh_interval)).await;
    }
}
