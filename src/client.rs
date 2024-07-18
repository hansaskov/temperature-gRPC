use std::thread;
use std::time::Duration;
use temperature::{
    temperature_service_client::TemperatureServiceClient, TemperatureReading, TemperatureRequest,
};

use tempurature_grpc::windows_hardware_monitor::HardwareMonitor;

pub mod temperature {
    tonic::include_proto!("temperature"); // The string specified here must match the proto package name
}

mod time_helper;
use time_helper::TimeHelper;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = TemperatureServiceClient::connect("http://[::1]:50051").await?;
    let mut readings = Vec::new();
    let hw = HardwareMonitor::new()?;

    for _ in 0..5 {
        let timestamp = TimeHelper::timestamp_now();

        match hw.cpu_temp() {
            Ok(cpu) => {
                println!(
                    "Recorded: Timestamp: {}, Temperature: {}°C",
                    timestamp, cpu.value
                );
                readings.push(TemperatureReading {
                    timestamp: Some(timestamp),
                    value: cpu.value,
                });
            }
            Err(x) => println!("Timestamp: {timestamp}, error: {x}"),
        }

        thread::sleep(Duration::from_secs(1));
    }

    let request = tonic::Request::new(TemperatureRequest { readings });
    let response = client.send_temperatures(request).await?;
    println!("RESPONSE={response:?}");

    Ok(())
}
