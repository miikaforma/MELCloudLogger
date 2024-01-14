use api::{ListDevicesResponse, CurrentDataResponse};
use tokio_postgres::{Error, NoTls, Client};

pub fn is_enabled() -> bool {
    dotenv::var("TIMESCALEDB_ENABLED")
        .map(|var| var.parse::<bool>())
        .unwrap_or(Ok(false))
        .unwrap()
}

pub async fn upsert_device_list_entry_into_timescaledb(client: &Client, data: &Vec<ListDevicesResponse>) -> Result<(), anyhow::Error> {
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

    let _ = client
    .execute(
        "INSERT INTO melcloud (
            time, device_id, device_type, power, offline, room_temperature, set_temperature, 
            last_communication, actual_fan_speed, fan_speed, automatic_fan_speed, 
            vane_vertical_direction, vane_vertical_swing, vane_horizontal_direction, 
            vane_horizontal_swing, operation_mode, in_standby_mode, heating_energy_consumed_rate1, 
            heating_energy_consumed_rate2, cooling_energy_consumed_rate1, cooling_energy_consumed_rate2, 
            auto_energy_consumed_rate1, auto_energy_consumed_rate2, dry_energy_consumed_rate1, 
            dry_energy_consumed_rate2, fan_energy_consumed_rate1, fan_energy_consumed_rate2, 
            other_energy_consumed_rate1, other_energy_consumed_rate2, current_energy_consumed, 
            current_energy_mode, energy_correction_model, energy_correction_active, 
            wifi_signal_strength, wifi_adapter_status, has_error
        ) VALUES (
            $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, 
            $19, $20, $21, $22, $23, $24, $25, $26, $27, $28, $29, $30, $31, $32, $33, $34, $35, $36
        ) ON CONFLICT (time, device_id) DO UPDATE SET 
            device_type = $3, power = $4, offline = $5, room_temperature = $6, set_temperature = $7, 
            last_communication = $8, actual_fan_speed = $9, fan_speed = $10, automatic_fan_speed = $11, 
            vane_vertical_direction = $12, vane_vertical_swing = $13, vane_horizontal_direction = $14, 
            vane_horizontal_swing = $15, operation_mode = $16, in_standby_mode = $17, 
            heating_energy_consumed_rate1 = $18, heating_energy_consumed_rate2 = $19, 
            cooling_energy_consumed_rate1 = $20, cooling_energy_consumed_rate2 = $21, 
            auto_energy_consumed_rate1 = $22, auto_energy_consumed_rate2 = $23, 
            dry_energy_consumed_rate1 = $24, dry_energy_consumed_rate2 = $25, 
            fan_energy_consumed_rate1 = $26, fan_energy_consumed_rate2 = $27, 
            other_energy_consumed_rate1 = $28, other_energy_consumed_rate2 = $29, 
            current_energy_consumed = $30, current_energy_mode = $31, 
            energy_correction_model = $32, energy_correction_active = $33, 
            wifi_signal_strength = $34, wifi_adapter_status = $35, has_error = $36",
        &[&system_time, &(device.device_iD as i32), &(device.device_type as i16), &device.power, &device.offline, &device.room_temperature, &device.set_temperature, 
          &device.last_time_stamp.clone(), &(device.actual_fan_speed as i16), &(device.fan_speed as i16), &device.automatic_fan_speed, 
          &(device.vane_vertical_direction as i16), &device.vane_vertical_swing, &(device.vane_horizontal_direction as i16), 
          &device.vane_horizontal_swing, &(device.operation_mode as i16), &device.in_standby_mode, &device.heating_energy_consumed_rate1, 
          &device.heating_energy_consumed_rate2, &device.cooling_energy_consumed_rate1, &device.cooling_energy_consumed_rate2, 
          &device.auto_energy_consumed_rate1, &device.auto_energy_consumed_rate2, &device.dry_energy_consumed_rate1, 
          &device.dry_energy_consumed_rate2, &device.fan_energy_consumed_rate1, &device.fan_energy_consumed_rate2, 
          &device.other_energy_consumed_rate1, &device.other_energy_consumed_rate2, &device.current_energy_consumed, 
          &(device.current_energy_mode.map(|num| num as i16)), &device.energy_correction_model, &device.energy_correction_active, 
          &device.wifi_signal_strength, &device.wifi_adapter_status, &device.has_error]
    )
    .await?;

    Ok(())
}

pub async fn upsert_current_data_into_timescaledb(client: &Client, data: &CurrentDataResponse) -> Result<(), anyhow::Error> {
    if !is_enabled() {
        return Ok(());
    }

    let system_time = data.last_communication_to_utc_datetime();
    if system_time.is_none() {
        return Err(anyhow::anyhow!("Skipping logging because system time couldn't be parsed"));
    }

    let system_time = system_time.unwrap();
    info!("System Time UTC: {:?}", system_time);

    let _ = client
    .execute(
        "INSERT INTO melcloud (
            time, device_id, device_type, power, offline, room_temperature, set_temperature, 
            last_communication, actual_fan_speed, fan_speed, vane_vertical_direction, 
            vane_horizontal_direction, operation_mode, in_standby_mode
        ) VALUES (
            $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14
        ) ON CONFLICT (time, device_id) DO UPDATE SET 
            device_type = $3, power = $4, offline = $5, room_temperature = $6, set_temperature = $7, 
            last_communication = $8, actual_fan_speed = $9, fan_speed = $10, vane_vertical_direction = $11, 
            vane_horizontal_direction = $12, operation_mode = $13, in_standby_mode = $14",
        &[&system_time, &(data.device_iD as i32), &(data.device_type as i16), &data.power, &data.offline, &data.room_temperature, &data.set_temperature, 
          &data.last_communication.clone(), &(data.set_fan_speed as i16), &(data.set_fan_speed as i16), &(data.vane_vertical as i16), 
          &(data.vane_horizontal as i16), &(data.operation_mode as i16), &data.in_standby_mode]
    )
    .await?;

    Ok(())
}

pub async fn connect_to_db() -> Result<tokio_postgres::Client, Error> {
    let (client, connection) = tokio_postgres::connect(
        &dotenv::var("TIMESCALEDB_CONNECTION_STRING").unwrap_or(
            "host=localhost user=myuser password=mysecretpassword dbname=melcloud".to_string(),
        ),
        NoTls,
    )
    .await?;

    // The connection object performs the actual communication with the database,
    // so spawn it off to run on its own.
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            error!("connection error: {}", e);
        }
    });

    Ok(client)
}
