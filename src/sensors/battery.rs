use super::{Provider, Sensor, SensorData, SensorRegisterData, SensorType, SensorUpdateData};
use battery::units::ratio::percent;

pub struct BatteryProvider {
    manager: battery::Manager,
    level: u8,
    charge_state: String,
}

impl Provider for BatteryProvider {
    type DataProvider = BatteryProvider;

    fn new() -> Result<Self, &'static str> {
        let mut provider = BatteryProvider {
            manager: battery::Manager::new().unwrap(),
            level: 0,
            charge_state: String::from(""),
        };
        provider.update()?;
        Ok(provider)
    }
    fn update(&mut self) -> Result<(), &'static str> {
        match self.manager.batteries() {
            Ok(mut batteries) => {
                let battery = batteries.next().unwrap().unwrap();
                self.level = battery.state_of_charge().get::<percent>() as u8;
                self.charge_state = battery.state().to_string();
                Ok(())
            }
            Err(_) => Err("No battery found"),
        }
    }
}

pub struct BatteryLevel;

impl Sensor<BatteryProvider> for BatteryLevel {
    fn register_info(&self, provider: &BatteryProvider) -> String {
        let data = SensorData::RegisterSensor::<u8> {
            data: SensorRegisterData {
                r#type: SensorType::Sensor,
                unique_id: String::from("battery_level"),
                name: String::from("Battery Level"),
                state: provider.level,
                device_class: None,
                icon: None,
                unit_of_measurement: Some(String::from("%")),
                state_class: None,
                entity_category: None,
            },
        };
        serde_json::to_string(&data).unwrap()
    }
    fn update_info(&self, provider: &BatteryProvider) -> String {
        let icon: Option<String>;
        match provider.level {
            91..=100 => icon = Some(String::from("mdi:battery")),
            81..=90 => icon = Some(String::from("mdi:battery-90")),
            71..=80 => icon = Some(String::from("mdi:battery-80")),
            61..=70 => icon = Some(String::from("mdi:battery-70")),
            51..=60 => icon = Some(String::from("mdi:battery-60")),
            41..=50 => icon = Some(String::from("mdi:battery-50")),
            31..=40 => icon = Some(String::from("mdi:battery-40")),
            21..=30 => icon = Some(String::from("mdi:battery-30")),
            11..=20 => icon = Some(String::from("mdi:battery-20")),
            0..=10 => icon = Some(String::from("mdi:battery-alert-outline")),
            _ => icon = Some(String::from("mdi:battery-unknown")),
        }
        let data = SensorData::UpdateSensorSates::<u8> {
            data: SensorUpdateData {
                r#type: SensorType::Sensor,
                unique_id: String::from("battery_level"),
                state: provider.level,
                icon,
            },
        };
        serde_json::to_string(&data).unwrap()
    }
}
