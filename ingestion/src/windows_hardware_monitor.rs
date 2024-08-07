#[cfg(target_os = "windows")]
pub mod windows_hardware_monitor {
    use anyhow::{anyhow, Result};
    use serde::Deserialize;
    use wmi::{COMLibrary, WMIConnection};

    #[derive(Deserialize, Debug, Clone)]
    #[serde(rename_all = "PascalCase")]
    pub enum SensorType {
        Voltage,
        Clock,
        Temperature,
        Load,
        Fan,
        Flow,
        Control,
        Level,
    }

    #[derive(Deserialize, Debug, Clone)]
    #[serde(rename_all = "PascalCase")]
    pub struct Sensor {
        pub sensor_type: SensorType,
        pub name: String,
        pub value: f32,
        pub min: f32,
        pub max: f32,
    }

    pub struct HardwareMonitor {
        pub wmi_con: WMIConnection,
    }

    impl HardwareMonitor {
        pub fn new() -> Result<Self> {
            let com_con = COMLibrary::new()?;
            let wmi_con =
                WMIConnection::with_namespace_path("ROOT\\LibreHardwareMonitor", com_con)?;

            Ok(Self { wmi_con })
        }

        pub fn query_sensor_type(
            &self,
            sensor_type: SensorType,
            name_filter: &str,
        ) -> Result<Vec<Sensor>> {
            let query = format!("SELECT * FROM Sensor WHERE SensorType = '{sensor_type:?}' AND Name LIKE '%{name_filter}%'");

            let results: Vec<Sensor> = self.wmi_con.raw_query(query)?;

            Ok(results)
        }

        pub fn cpu_temp(&self) -> Result<Sensor> {
            let result = self.query_sensor_type(SensorType::Temperature, "Core")?;

            let sensor_reading = result.first();

            match sensor_reading {
                Some(sensor) => Ok(sensor.clone()),
                None => Err(anyhow!(
                    "Found nothing, are you sure Libre Hardware Monitor is running?"
                )),
            }
        }
    }
}
