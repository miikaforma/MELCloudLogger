#[macro_use]
extern crate log;

use api::errors::ApiError;
use chrono_tz::Tz;
use std::time::Duration;

use dotenv::dotenv;
use tokio::time::sleep;

use crate::{
    app::app::{fetch_and_log_new_entry, get_access_token, refresh_device, get_device},
    storage::{influxdb::influx::{self}, timescaledb::timescale::{self}},
};

mod app;
mod logging;
mod storage;

fn validate_configs() {
    let _ = dotenv::var("MELCLOUD_EMAIL").unwrap();
    let _ = dotenv::var("MELCLOUD_PASSWORD").unwrap();
}

fn get_timezone() -> Tz {
    let timezone = dotenv::var("CHRONO_TIMEZONE").unwrap_or("Europe/Helsinki".to_string());
    timezone.parse().unwrap()
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    logging::init_logging();

    info!("MELCloud Logger starting");
    info!("Using time zone: {}", get_timezone().name());

    validate_configs();

    let refresh_interval: u64 = dotenv::var("REFRESH_INTERVAL")
        .map(|var| var.parse::<u64>())
        .unwrap_or(Ok(60_000))
        .unwrap();

    let fetch_interval: u64 = dotenv::var("FETCH_INTERVAL")
        .map(|var| var.parse::<u64>())
        .unwrap_or(Ok(10_000))
        .unwrap();

    let mut access_token = dotenv::var("ACCESS_TOKEN").ok();

    if access_token.is_none() {
        access_token = Some(get_access_token().await);
    }
    let mut access_token = access_token.unwrap();

    // Get the device information
    let (building_id, device) = get_device(access_token.to_string()).await.unwrap();
    let building_id = dotenv::var("BUILDING_ID").unwrap_or_else(|_| building_id.to_string());
    let device_id = dotenv::var("DEVICE_ID").unwrap_or_else(|_| device.device_iD.to_string());

    // Connect to influx database
    let mut influx_client: Option<influxdb::Client> = None;
    if influx::is_enabled() {
        influx_client = Some(influx::connect_to_db().await);
    }

    // Connect to timescale database
    let mut timescale_client: Option<tokio_postgres::Client> = None;
    if timescale::is_enabled() {
        match timescale::connect_to_db().await {
            Ok(client) => {
                timescale_client = Some(client);
            }
            Err(err) => {
                error!("Failed to connect to timescale database: {}", err);
            }
        }
    }

    // Logging loop
    loop {
        match refresh_device(access_token.to_string(), device_id.clone()).await {
            Err(ApiError::Unauthorized) => {
                error!("Failed to request a device refresh because of unauthorized");
                // Fetch a new access token and try refreshing again
                access_token = get_access_token().await;
                let _ = refresh_device(access_token.to_string(), device_id.clone()).await;
            }
            Err(ApiError::Other(err)) => {
                error!("Failed to request a device refresh {}", err);
            }
            Ok(_) => {}
        }
        sleep(Duration::from_millis(fetch_interval)).await;
        match fetch_and_log_new_entry(
            &influx_client,
            &timescale_client,
            access_token.to_string(),
            device_id.clone(),
            building_id.clone(),
        )
        .await
        {
            Err(ApiError::Unauthorized) => {
                error!("Failed to request a new entry because of unauthorized");
                // Fetch a new access token and try fetching again
                access_token = get_access_token().await;
                let _ = fetch_and_log_new_entry(
                    &influx_client,
                    &timescale_client,
                    access_token.to_string(),
                    device_id.clone(),
                    building_id.clone(),
                )
                .await;
            }
            Err(ApiError::Other(err)) => {
                error!("Failed to request a new entry {}", err);
            }
            Ok(_) => {}
        };
        sleep(Duration::from_millis(refresh_interval)).await;
    }
}
