use std::time::Duration;

use api::CurrentDataResponse;
use api::request_refresh;
use api::current_data;
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
    has_pending_command: bool,
    last_communication: String,
    next_communication: String,
}

async fn fetch_and_log_new_entry(client: &Client, access_token: String, device_id: String, building_id: String) {
    println!("Logging new entry for device {}", &device_id);

    match current_data(access_token, device_id, building_id).await {
        Ok(data) => {
            log_new_entry(client, &data).await
        }
        Err(_) => {}
    }
}

async fn log_new_entry(client: &Client, data: &CurrentDataResponse) {
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
        has_pending_command: data.has_pending_command,
        last_communication: data.last_communication.clone(),
        next_communication: data.next_communication.clone(),
    };

    let write_result = client.query(&current_data.into_query("melCloudDeviceData")).await;
    if let Err(err) = write_result {
        eprintln!("Error writing to db: {}", err)
    }
}

async fn refresh_device(access_token: String, device_id: String) -> bool {
    println!("Refreshing the device {}", &device_id);

    match request_refresh(access_token, device_id).await {
        Ok(data) => {
            true
        }
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
        fetch_and_log_new_entry(&client, access_token.to_string(), device_id.clone(), building_id.clone()).await;
        sleep(Duration::from_millis(refresh_interval)).await;
    }
}
