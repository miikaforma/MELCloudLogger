use api::{CurrentDataResponse, ListDevicesResponse};
use influxdb::{Client, InfluxDbWriteable};

use crate::storage::influxdb::current_data::CurrentData;

pub fn is_enabled() -> bool {
    dotenv::var("INFLUXDB_ENABLED")
        .map(|var| var.parse::<bool>())
        .unwrap_or(Ok(false))
        .unwrap()
}

pub async fn upsert_device_list_entry_into_influxdb(client: &Client, data: &Vec<ListDevicesResponse>) -> Result<(), anyhow::Error> {
    if !is_enabled() {
        return Ok(());
    }

    let list_devices = &data[0];
    let structure = &list_devices.structure;
    let devices = &structure.devices;
    let device = &devices[0].device;

    let system_time = device.last_time_stamp_to_utc_datetime();
    if system_time.is_none() {
        return Err(anyhow::anyhow!("Skipping logging because system time couldn't be parsed"));
    }

    let system_time = system_time.unwrap();
    info!("System Time UTC: {:?}", system_time);
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
        return Err(anyhow::anyhow!("Error writing to db: {}", err));
    }

    Ok(())
}

pub async fn upsert_current_data_into_influxdb(client: &Client, data: &CurrentDataResponse) -> Result<(), anyhow::Error> {
    if !is_enabled() {
        return Ok(());
    }

    let system_time = data.last_communication_to_utc_datetime();
    if system_time.is_none() {
        return Err(anyhow::anyhow!("Skipping logging because system time couldn't be parsed"));
    }

    let system_time = system_time.unwrap();
    info!("System Time UTC: {:?}", system_time);
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
        return Err(anyhow::anyhow!("Error writing to db: {}", err));
    }

    Ok(())
}

pub async fn connect_to_db() -> Client {
    let database_url = dotenv::var("INFLUXDB_CONNECTION_STRING").unwrap_or("http://localhost:8086".to_string());
    let database_name = dotenv::var("INFLUXDB_DATABASE_NAME").unwrap_or("entsoe".to_string());

    Client::new(&database_url, &database_name)
}
