pub mod models;

use http::{StatusCode, header::USER_AGENT};
use dotenv::dotenv;
pub use models::*;

pub async fn current_data(access_token: &String, device_id: &String, building_id: &String) -> Result<CurrentDataResponse, anyhow::Error> {
    let res = reqwest::Client::new()
        .get(format!("https://app.melcloud.com/Mitsubishi.Wifi.Client/Device/Get?id={}&buildingID={}", device_id, building_id))
        .header(USER_AGENT, "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:101.0) Gecko/20100101 Firefox/101.0")
        .header("X-MitsContextKey", access_token)
        .send()
        .await?;

    let status = res.status();

    let data_str = res
        .text()
        .await?;
    //println!("{}", data_str);

    if status != StatusCode::OK {
        return Err(anyhow::anyhow!(data_str));
    }

    let data: CurrentDataResponse = serde_json::from_str(&data_str)?;
    //println!("CurrentDataResponse: {:#?}", data);

    Ok(data)
}

pub async fn listdevices_data(access_token: &String) -> Result<Vec<ListDevicesResponse>, anyhow::Error> {
    let res = reqwest::Client::new()
        .get(format!("https://app.melcloud.com/Mitsubishi.Wifi.Client/User/ListDevices"))
        .header(USER_AGENT, "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:101.0) Gecko/20100101 Firefox/101.0")
        .header("X-MitsContextKey", access_token)
        .send()
        .await?;

    let status = res.status();

    let data_str = res
        .text()
        .await?;
    //println!("{}", data_str);

    if status != StatusCode::OK {
        return Err(anyhow::anyhow!(data_str));
    }

    let data: Vec<ListDevicesResponse> = serde_json::from_str(&data_str)?;
    //println!("Vec<ListDevicesResponse>: {:#?}", data);

    Ok(data)
}

pub async fn request_refresh(access_token: String, device_id: String) -> Result<bool, anyhow::Error> {
    let res = reqwest::Client::new()
    .get(format!("https://app.melcloud.com/Mitsubishi.Wifi.Client/Device/RequestRefresh?id={}", device_id))
        .header(USER_AGENT, "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:101.0) Gecko/20100101 Firefox/101.0")
        .header("X-MitsContextKey", access_token)
        .send()
        .await?;

    let status = res.status();

    let data_str = res
        .text()
        .await?;
    //println!("{}", data_str);

    if status != StatusCode::OK {
        return Err(anyhow::anyhow!(data_str));
    }

    let data: bool = serde_json::from_str(&data_str)?;
    // println!("bool: {:#?}", data);

    Ok(data)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_current_data() {
        dotenv().ok();

        let building_id = dotenv::var("BUILDING_ID").unwrap();
        let device_id = dotenv::var("DEVICE_ID").unwrap();
        let access_token = dotenv::var("ACCESS_TOKEN").unwrap();

        let response = current_data(&access_token, &device_id, &building_id).await.unwrap();
        println!("Last communication {}", response.last_communication_to_utc_datetime().unwrap());
        assert_eq!(device_id, response.device_iD.to_string());
    }

    #[tokio::test]
    async fn test_listdevices_data() {
        dotenv().ok();

        let device_id = dotenv::var("DEVICE_ID").unwrap();
        let access_token = dotenv::var("ACCESS_TOKEN").unwrap();

        let response = listdevices_data(&access_token)
            .await
            .unwrap();
        
        let list_devices = &response[0];
        let structure = &list_devices.structure;
        let devices = &structure.devices;

        println!("Last communication {}", devices[0].device.last_time_stamp_to_utc_datetime().unwrap());
        assert_eq!(1, response.len());
        assert_eq!(1, devices.len());
    }

    #[tokio::test]
    async fn test_request_refresh() {
        dotenv().ok();

        let device_id = dotenv::var("DEVICE_ID").unwrap();
        let access_token = dotenv::var("ACCESS_TOKEN").unwrap();

        let response = request_refresh(access_token, device_id).await.unwrap();
        assert_eq!(true, response);
    }
}
