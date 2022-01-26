use super::{Provider, Sensor, SensorData, SensorRegisterData, SensorType, SensorUpdateData};
use sys_info;
use serde::Serialize;


pub struct CpuProvider {
pub cpu_num:u32,
pub cpu_freq : u64,
}

impl Provider for CpuProvider {
    /// Init the CPU provider
    fn new() -> Self {
        CpuProvider {
            cpu_num: sys_info::cpu_num().unwrap(),
            cpu_freq: sys_info::cpu_speed().unwrap(),
        }
    }

    /// Update the CPU provider
    fn update_all(&mut self) -> Result<(), &'static str> {
        self.cpu_num = sys_info::cpu_num().unwrap();
        self.cpu_freq = sys_info::cpu_speed().unwrap();
        Ok(())
    }
}

#[derive(Serialize, Debug)]
pub struct CpuFreq {
    pub state: u64,
}

impl Sensor<CpuProvider> for CpuFreq {
    type StateType = u64;

    fn new() -> Self {
        CpuFreq {state : 0}
    }

    #[allow(unused)]
    fn get_current(provider: &CpuProvider) -> Result<Self::StateType, &'static str> {
        Ok(sys_info::cpu_speed().unwrap())
    }

    fn get_register_info(&self) -> SensorData<Self::StateType> {
        SensorData::RegisterSensor::<Self::StateType> {
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
        }
    }

    fn get_update_info(&self) -> SensorData<Self::StateType> {
        SensorData::UpdateSensorStates::<Self::StateType> {
            data: SensorUpdateData {
                r#type: SensorType::Sensor,
                unique_id: String::from("cpu_freq"),
                state: self.state,
                icon: Some(String::from("mdi:sine-wave")),
            }
        }
    }

}
