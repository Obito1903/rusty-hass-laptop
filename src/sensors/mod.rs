use serde::Serialize;

pub mod battery;
pub mod cpu;

pub trait Provider {
    fn new() -> Self;
    fn update_all(&mut self) -> Result<(), &'static str>;
}

pub trait Sensor<T: Provider> {
    type StateType: Serialize;

    fn new() -> Self;
    fn get_current(provider: &T) -> Result<Self::StateType, &'static str>;
    fn get_register_info(&self) -> SensorData<Self::StateType>;
    fn get_update_info(&self) -> SensorData<Self::StateType>;
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
    UpdateSensorStates { data: SensorUpdateData<T> },
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

    use super::{battery, Sensor};
    use super::{Provider, SensorData, SensorType, SensorUpdateData};

    #[test]
    fn serialize_test() {
        let req = SensorData::UpdateSensorStates::<u32> {
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
        let bat_info = battery::BatteryProvider::new();
        println!(
            "serialized = {}",
            serde_json::to_string(&bat_info.level.get_register_info()).unwrap()
        );
        println!(
            "serialized = {}",
            serde_json::to_string(&bat_info.level.get_update_info()).unwrap()
        );
    }
}
