use super::{Provider, Sensor, SensorData, SensorRegisterData, SensorType, SensorUpdateData};
use battery::units::ratio::percent;
use serde::Serialize;

pub struct BatteryProvider {
    battery: battery::Battery,
    pub level: BatteryLevel,
    // charge_state: String,
}

impl Provider for BatteryProvider {
    /// Create a new BatteryProvider
    fn new() -> Self {
        let manager = battery::Manager::new().unwrap();
        BatteryProvider {
            battery: manager
                .batteries()
                .unwrap()
                .into_iter()
                .next()
                .unwrap()
                .unwrap(),
            level: BatteryLevel::new(),
            // charge_state: String::from(""),
        }
    }

    /// Update all the data
    fn update_all(&mut self) -> Result<(), &'static str> {
        self.battery = battery::Manager::new()
            .unwrap()
            .batteries()
            .unwrap()
            .into_iter()
            .next()
            .unwrap()
            .unwrap();
        self.level.state = BatteryLevel::get_current(self).unwrap();
        Ok(())
    }
}

#[derive(Serialize, Debug)]
pub struct BatteryLevel {
    pub state: u8,
}

impl Sensor<BatteryProvider> for BatteryLevel {
    type StateType = u8;

    fn new() -> Self {
        BatteryLevel { state: 0 }
    }

    fn get_current(provider: &BatteryProvider) -> Result<Self::StateType, &'static str> {
        Ok(provider.battery.state_of_charge().get::<percent>() as u8)
    }

    fn get_register_info(&self) -> SensorData<Self::StateType> {
        return SensorData::RegisterSensor::<Self::StateType> {
            data: SensorRegisterData {
                r#type: SensorType::Sensor,
                unique_id: String::from("battery_level"),
                name: String::from("Battery Level"),
                state: self.state,
                device_class: None,
                icon: Some(String::from("mdi:battery-unknown")),
                unit_of_measurement: Some(String::from("%")),
                state_class: None,
                entity_category: None,
            },
        };
    }

    fn get_update_info(&self) -> SensorData<Self::StateType> {
        let icon: Option<String>;
        match self.state {
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
        SensorData::UpdateSensorStates::<Self::StateType> {
            data: SensorUpdateData {
                r#type: SensorType::Sensor,
                unique_id: String::from("battery_level"),
                state: self.state,
                icon,
            },
        }
    }
}
