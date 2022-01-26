use crate::{sensors::Provider, sensors::Sensor, Config};
use reqwest::{self, header};
use serde::{Deserialize, Serialize};
use std::result;

#[derive(Debug, Clone)]
pub struct Client {
    pub client: reqwest::blocking::Client,
    pub config: Config,
}

#[derive(Serialize)]
pub struct RegistrationInfoReq {
    pub device_id: String,
    pub app_id: String,
    pub app_name: String,
    pub app_version: String,
    pub device_name: String,
    pub manufacturer: String,
    pub model: String,
    pub os_name: String,
    pub os_version: String,
    pub supports_encryption: bool,
}

#[derive(Deserialize)]
pub struct RegistrationInfoResp {
    pub secret: Option<String>,
    pub webhook_id: Option<String>,
}

impl Client {
    pub fn new(config: Config) -> result::Result<Self, reqwest::Error> {
        let mut headers = header::HeaderMap::new();
        let mut auth_value = reqwest::header::HeaderValue::from_str(&format!(
            "Bearer {}",
            config.auth_token.as_ref().unwrap()
        ))
        .unwrap();
        auth_value.set_sensitive(true);
        headers.insert(header::AUTHORIZATION, auth_value);
        Ok(Self {
            client: reqwest::blocking::Client::builder()
                .default_headers(headers)
                .build()?,
            config: config,
        })
    }

    pub fn register_app(&mut self) -> result::Result<(), reqwest::Error> {
        let resp = self
            .client
            .post(&format!(
                "{}/api/mobile_app/registrations",
                self.config.hass_address.as_ref().unwrap()
            ))
            .json(&RegistrationInfoReq {
                device_id: self.config.device_id.clone(),
                app_id: self.config.app_id.clone(),
                app_name: self.config.app_name.clone(),
                app_version: self.config.app_version.clone(),
                device_name: self.config.device_name.clone(),
                manufacturer: self.config.manufacturer.clone(),
                model: self.config.model.clone(),
                os_name: self.config.os_name.clone(),
                os_version: self.config.os_version.clone(),
                supports_encryption: self.config.support_encryption,
            })
            .send();
        match resp {
            Ok(resp) => {
                let resp_body: RegistrationInfoResp = resp.json()?;
                self.config.webhook_id = resp_body.webhook_id;
                self.config.webhook_secret = resp_body.secret;
                ()
            }
            Err(_) => (),
        }
        Ok(())
    }
    pub fn register_sensor<T, U: Provider>(
        &mut self,
        sensor: &T,
    ) -> result::Result<(), reqwest::Error>
    where
        T: Sensor<U>,
    {
        let resp = self
            .client
            .post(format!(
                "{}/api/webhook/{}",
                self.config.hass_address.as_ref().unwrap(),
                self.config.webhook_id.as_ref().unwrap()
            ))
            .json(&sensor.get_register_info())
            .send();
        self.config.save_to_file().unwrap();
        match resp {
            Ok(resp) => {
                println!("{}", resp.text().unwrap());
                ()
            }
            Err(_) => panic!("prout"),
        }
        Ok(())
    }
    pub fn update_sensor<T, U: Provider>(
        &mut self,
        sensor: &T,
    ) -> result::Result<(), reqwest::Error>
    where
        T: Sensor<U>,
    {
        let resp = self
            .client
            .post(format!(
                "{}/api/webhook/{}",
                self.config.hass_address.as_ref().unwrap(),
                self.config.webhook_id.as_ref().unwrap()
            ))
            .json(&sensor.get_update_info())
            .send();
        self.config.save_to_file().unwrap();
        match resp {
            Ok(resp) => {
                println!(
                    "Status: {} \n body: {}",
                    resp.status(),
                    resp.text().unwrap()
                );
                ()
            }
            Err(_) => panic!("prout"),
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {

    use crate::sensors::{battery::BatteryProvider, Provider, Sensor};

    use crate::Config;

    use super::Client;

    #[test]
    fn register_test() {
        match Client::new(Config::new()) {
            Ok(mut client) => {
                client.register_app().unwrap();
                println!(
                    "serialized = {}",
                    client.config.webhook_id.as_ref().unwrap()
                );
                client.config.save_to_file().unwrap();
                ()
            }
            Err(_) => panic!("prout"),
        }
    }
    #[test]
    fn register_sensor() {
        match Client::new(Config::new()) {
            Ok(mut client) => {
                let batt_provider = BatteryProvider::new();
                client.register_sensor(&batt_provider.level).unwrap();
                ()
            }
            Err(_) => panic!("prout"),
        }
    }
    #[test]
    fn update_sensor() {
        match Client::new(Config::new()) {
            Ok(mut client) => {
                let mut batt_provider = BatteryProvider::new();
                batt_provider.update_all().unwrap();
                println!(
                    "serialized = {}",
                    serde_json::to_string(&batt_provider.level.get_update_info()).unwrap()
                );
                client.update_sensor(&batt_provider.level).unwrap();
                ()
            }
            Err(_) => panic!("prout"),
        }
    }
}
