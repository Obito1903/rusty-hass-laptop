use serde::Serialize;

pub mod battery;

pub trait Provider {
    type DataProvider;
    fn new() -> Result<Self::DataProvider, &'static str>;
    fn update(&mut self) -> Result<(), &'static str>;
}

pub trait Sensor<T: Provider> {
    fn register_info(&self, provider: &T) -> String;
    fn update_info(&self, provider: &T) -> String;
}

#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SensorType {
    Sensor,
    BinarySensor,
}

#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "type")]
pub enum SensorData<T: Serialize> {
    UpdateSensorSates { data: SensorUpdateData<T> },
    RegisterSensor { data: SensorRegisterData<T> },
}

#[derive(Serialize)]
pub struct SensorRegisterData<T: Serialize> {
    pub r#type: SensorType,
    pub unique_id: String,
    pub name: String,
    pub state: T,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_class: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_of_measurement: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_class: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_category: Option<String>,
}

#[derive(Serialize)]
pub struct SensorUpdateData<T: Serialize> {
    pub r#type: SensorType,
    pub unique_id: String,
    pub state: T,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
}

#[cfg(test)]
mod tests {
    use crate::sensors::battery::BatteryLevel;

    use super::{battery, Sensor};
    use super::{Provider, SensorData, SensorType, SensorUpdateData};

    #[test]
    fn serialize_test() {
        let req = SensorData::UpdateSensorSates::<u32> {
            data: SensorUpdateData {
                r#type: SensorType::Sensor,
                unique_id: String::from("battery_level"),
                state: 32,
                icon: None,
            },
        };
        println!("serialized = {}", serde_json::to_string(&req).unwrap());
    }
    #[test]
    fn batlevel_test() {
        let bat_info = battery::BatteryProvider::new().unwrap();
        println!("serialized = {}", BatteryLevel.register_info(&bat_info));
        println!("serialized = {}", BatteryLevel.update_info(&bat_info));
    }
}
