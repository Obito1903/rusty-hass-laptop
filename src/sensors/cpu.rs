use super::{Provider, Sensor, SensorData, SensorRegisterData, SensorType, SensorUpdateData};
use serde::Serialize;
use sys_info;

pub struct CpuProvider {
    pub cpu_cores: CpuCores,
    pub cpu_freq: CpuFreq,
}

impl Provider for CpuProvider {
    /// Init the CPU provider
    fn new() -> Self {
        CpuProvider {
            cpu_cores: CpuCores::new(),
            cpu_freq: CpuFreq::new(),
        }
    }

    /// Update the CPU provider
    fn update_all(&mut self) -> Result<(), &'static str> {
        self.cpu_cores.state = CpuCores::get_current(self).unwrap();
        self.cpu_freq.state = CpuFreq::get_current(self).unwrap();
        Ok(())
    }
}

#[derive(Serialize, Debug)]
pub struct CpuFreq {
    pub state: u64,
    unique_id: String,
}

impl Sensor<CpuProvider> for CpuFreq {
    type StateType = u64;

    fn new() -> Self {
        CpuFreq {
            state: 0,
            unique_id: String::from("cpu_freq"),
        }
    }

    #[allow(unused)]
    fn get_current(provider: &CpuProvider) -> Result<Self::StateType, &'static str> {
        Ok(sys_info::cpu_speed().unwrap())
    }

    fn get_register_info(&self) -> SensorData<Self::StateType> {
        SensorData::RegisterSensor::<Self::StateType> {
            data: SensorRegisterData {
                r#type: SensorType::Sensor,
                unique_id: self.unique_id.clone(),
                name: String::from("CPU Frequency"),
                state: self.state,
                device_class: Some(String::from("frequency")),
                icon: Some(String::from("mdi:sine-wave")),
                unit_of_measurement: Some(String::from("MHz")),
                state_class: None,
                entity_category: None,
            },
        }
    }

    fn get_update_info(&self) -> SensorData<Self::StateType> {
        SensorData::UpdateSensorStates::<Self::StateType> {
            data: SensorUpdateData {
                r#type: SensorType::Sensor,
                unique_id: self.unique_id.clone(),
                state: self.state,
                icon: Some(String::from("mdi:sine-wave")),
            },
        }
    }
}

#[derive(Serialize, Debug)]
pub struct CpuCores {
    pub state: u32,
    unique_id: String,
}

impl Sensor<CpuProvider> for CpuCores {
    type StateType = u32;

    fn new() -> Self {
        CpuCores {
            state: 0,
            unique_id: String::from("cpu_cores"),
        }
    }

    #[allow(unused)]
    fn get_current(provider: &CpuProvider) -> Result<Self::StateType, &'static str> {
        Ok(sys_info::cpu_num().unwrap())
    }

    fn get_register_info(&self) -> SensorData<Self::StateType> {
        SensorData::RegisterSensor::<Self::StateType> {
            data: SensorRegisterData {
                r#type: SensorType::Sensor,
                unique_id: self.unique_id.clone(),
                name: String::from("CPU Cores"),
                state: self.state,
                device_class: None,
                icon: Some(String::from("mdi:memory")),
                unit_of_measurement: None,
                state_class: None,
                entity_category: None,
            },
        }
    }

    fn get_update_info(&self) -> SensorData<Self::StateType> {
        SensorData::UpdateSensorStates::<Self::StateType> {
            data: SensorUpdateData {
                r#type: SensorType::Sensor,
                unique_id: self.unique_id.clone(),
                state: self.state,
                icon: Some(String::from("mdi:memory")),
            },
        }
    }
}
