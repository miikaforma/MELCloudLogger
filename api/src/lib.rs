#[macro_use]
extern crate log;

pub mod errors;
pub mod models;

use errors::ApiError;
use http::{header::USER_AGENT, StatusCode};
pub use models::*;

const API_URL: &str = r#"https://app.melcloud.com"#;
const CUSTOM_USER_AGENT: &str =
    r#"Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:121.0) Gecko/20100101 Firefox/121.0"#;
const APP_VERSION: &str = r#"1.31.0.0"#;

pub async fn get_access_token(email: &str, password: &str) -> Result<LoginResponse, anyhow::Error> {
    let login_request = LoginRequest {
        app_version: APP_VERSION.to_string(),
        captcha_response: None,
        email: email.to_string(),
        password: password.to_string(),
        language: 17,
        persist: true,
    };

    let res = reqwest::Client::new()
        .post(format!(
            "{}/Mitsubishi.Wifi.Client/Login/ClientLogin",
            API_URL
        ))
        .header(USER_AGENT, CUSTOM_USER_AGENT)
        .json(&login_request)
        .send()
        .await?;

    let status = res.status();

    let data_str = res.text().await?;
    debug!("{}", data_str);

    if status != StatusCode::OK {
        return Err(anyhow::anyhow!(data_str));
    }

    let data: LoginResponse = serde_json::from_str(&data_str)?;
    debug!("LoginResponse: {:#?}", data_str);

    if data.has_error() {
        return Err(anyhow::anyhow!(data.error_message()));
    }

    Ok(data)
}

pub async fn current_data(
    access_token: &String,
    device_id: &String,
    building_id: &String,
) -> Result<CurrentDataResponse, ApiError> {
    let res = reqwest::Client::new()
        .get(format!(
            "{}/Mitsubishi.Wifi.Client/Device/Get?id={}&buildingID={}",
            API_URL, device_id, building_id
        ))
        .header(USER_AGENT, CUSTOM_USER_AGENT)
        .header("X-MitsContextKey", access_token)
        .send()
        .await
        .map_err(anyhow::Error::from)?;

    let status = res.status();

    if status == StatusCode::UNAUTHORIZED {
        return Err(ApiError::Unauthorized);
    }

    let data_str = res.text().await.map_err(anyhow::Error::from)?;
    debug!("{}", data_str);

    if status != StatusCode::OK {
        return Err(ApiError::Other(anyhow::anyhow!(data_str)));
    }

    let data: CurrentDataResponse = serde_json::from_str(&data_str).map_err(anyhow::Error::from)?;
    debug!("CurrentDataResponse: {:#?}", data_str);

    Ok(data)
}

pub async fn listdevices_data(access_token: &String) -> Result<Vec<ListDevicesResponse>, ApiError> {
    let res = reqwest::Client::new()
        .get(format!(
            "{}/Mitsubishi.Wifi.Client/User/ListDevices",
            API_URL
        ))
        .header(USER_AGENT, CUSTOM_USER_AGENT)
        .header("X-MitsContextKey", access_token)
        .send()
        .await
        .map_err(anyhow::Error::from)?;

    let status = res.status();

    if status == StatusCode::UNAUTHORIZED {
        return Err(ApiError::Unauthorized);
    }

    let data_str = res.text().await.map_err(anyhow::Error::from)?;
    debug!("{}", data_str);

    if status != StatusCode::OK {
        return Err(ApiError::Other(anyhow::anyhow!(data_str)));
    }

    let data: Vec<ListDevicesResponse> = serde_json::from_str(&data_str).map_err(anyhow::Error::from)?;
    debug!("Vec<ListDevicesResponse>: {:#?}", data_str);

    Ok(data)
}

pub async fn request_refresh(access_token: String, device_id: String) -> Result<bool, ApiError> {
    let res = reqwest::Client::new()
        .get(format!(
            "{}/Mitsubishi.Wifi.Client/Device/RequestRefresh?id={}",
            API_URL, device_id
        ))
        .header(USER_AGENT, CUSTOM_USER_AGENT)
        .header("X-MitsContextKey", access_token)
        .send()
        .await
        .map_err(anyhow::Error::from)?;

    let status = res.status();

    if status == StatusCode::UNAUTHORIZED {
        return Err(ApiError::Unauthorized);
    }

    let data_str = res.text().await.map_err(anyhow::Error::from)?;
    debug!("{}", data_str);

    if status != StatusCode::OK {
        return Err(ApiError::Other(anyhow::anyhow!(data_str)));
    }

    let data: bool = serde_json::from_str(&data_str).map_err(anyhow::Error::from)?;
    debug!("bool: {:#?}", data_str);

    Ok(data)
}

#[cfg(test)]
mod tests {
    use super::*;
    use dotenv::dotenv;

    #[tokio::test]
    async fn test_get_access_token() {
        dotenv().ok();

        let email = dotenv::var("MELCLOUD_EMAIL").unwrap();
        let password = dotenv::var("MELCLOUD_PASSWORD").unwrap();

        println!("Email {}", email);
        println!("Password {}", password);

        let response = get_access_token(&email, &password).await.unwrap();
        println!("Token {}", response.token());
    }

    #[tokio::test]
    async fn test_get_current_data() {
        dotenv().ok();

        let building_id = dotenv::var("BUILDING_ID").unwrap();
        let device_id = dotenv::var("DEVICE_ID").unwrap();
        let access_token = dotenv::var("ACCESS_TOKEN").unwrap();

        let response = current_data(&access_token, &device_id, &building_id)
            .await
            .unwrap();
        println!(
            "Last communication {}",
            response.last_communication_to_utc_datetime().unwrap()
        );
        assert_eq!(device_id, response.device_iD.to_string());
    }

    #[tokio::test]
    async fn test_listdevices_data() {
        dotenv().ok();

        let access_token = dotenv::var("ACCESS_TOKEN").unwrap();

        let response = listdevices_data(&access_token).await.unwrap();

        let list_devices = &response[0];
        let structure = &list_devices.structure;
        let devices = &structure.devices;

        println!(
            "Last communication {}",
            devices[0].device.last_time_stamp_to_utc_datetime().unwrap()
        );
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
