use api::Device;
use api::errors::ApiError;

use api::current_data;
use api::listdevices_data;
use api::request_refresh;

use crate::storage::influxdb::influx::upsert_device_list_entry_into_influxdb;
use crate::storage::influxdb::influx::upsert_current_data_into_influxdb;
use crate::storage::timescaledb::timescale::upsert_current_data_into_timescaledb;
use crate::storage::timescaledb::timescale::upsert_device_list_entry_into_timescaledb;

pub async fn get_access_token() -> String {
    let email = dotenv::var("MELCLOUD_EMAIL").unwrap();
    let password = dotenv::var("MELCLOUD_PASSWORD").unwrap();

    info!("Logging in with email {}", &email);

    match api::get_access_token(&email, &password).await {
        Ok(data) => {
            info!("Login successful");
            data.token()
        }
        Err(err) => {
            error!("Failed to get access token: {}", err);
            "".to_string()
        }
    }
}

pub async fn get_device(access_token: String) -> Result<(u32, Device), ApiError> {
    match listdevices_data(&access_token).await {
        Ok(data) => {
            let list_devices = &data[0];
            let structure = &list_devices.structure;
            let devices = &structure.devices;
            let device = &devices[0].device;

            Ok((list_devices.iD, device.clone()))
        },
        Err(ApiError::Unauthorized) => {
            error!("Failed to request list devices data because of unauthorized");
            Err(ApiError::Unauthorized)
        }
        Err(ApiError::Other(err)) => {
            error!("Failed to request list devices data {}", err);
            Err(api::errors::ApiError::Other(err))
        }
    }
}

pub async fn refresh_device(access_token: String, device_id: String) -> Result<bool, ApiError> {
    info!("Refreshing the device {}", &device_id);

    match request_refresh(access_token, device_id).await {
        Ok(_data) => {
            Ok(true)
        }
        Err(ApiError::Unauthorized) => {
            error!("Failed to request a device refresh because of unauthorized");
            Err(ApiError::Unauthorized)
        }
        Err(ApiError::Other(err)) => {
            error!("Failed to request a device refresh {}", err);
            Err(api::errors::ApiError::Other(err))
        }
    }
}

pub async fn fetch_and_log_new_entry(
    influxdb_client: &Option<influxdb::Client>,
    timescaledb_client: &Option<tokio_postgres::Client>,
    access_token: String,
    device_id: String,
    building_id: String,
) -> Result<(), ApiError> {
    info!("Logging new entry for device {}", &device_id);

    match listdevices_data(&access_token).await {
        Ok(data) => {
            if let Some(client) = influxdb_client {
                if let Err(e) = upsert_device_list_entry_into_influxdb(client, &data).await {
                    error!("Failed to log device list entry into influxdb: {}", e);
                }
            }

            if let Some(client) = timescaledb_client {
                if let Err(e) = upsert_device_list_entry_into_timescaledb(client, &data).await {
                    error!("Failed to log device list entry into timescaledb: {}", e);
                }
            }

            Ok(())
        },
        Err(ApiError::Unauthorized) => {
            error!("Failed to request list devices data because of unauthorized");
            Err(ApiError::Unauthorized)
        }
        Err(ApiError::Other(err)) => {
            error!("Failed to request list devices data {}", err);
            match current_data(&access_token, &device_id, &building_id).await {
                Ok(data) => {
                    if let Some(client) = influxdb_client {
                        if let Err(e) = upsert_current_data_into_influxdb(client, &data).await {
                            error!("Failed to log current data into influxdb: {}", e);
                        }
                    }
        
                    if let Some(client) = timescaledb_client {
                        if let Err(e) = upsert_current_data_into_timescaledb(client, &data).await {
                            error!("Failed to log current data into timescaledb: {}", e);
                        }
                    }
    
                    Ok(())
                },
                Err(ApiError::Unauthorized) => {
                    error!("Failed to request current data because of unauthorized");
                    Err(ApiError::Unauthorized)
                }
                Err(ApiError::Other(err)) => {
                    error!("Failed to request current data {}", err);
                    Err(api::errors::ApiError::Other(err))
                }
            }
        },
    }
}
