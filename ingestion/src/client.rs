use std::time::{Duration, SystemTime};
use temperature::{
    temperature_service_client::TemperatureServiceClient, Temperature, TemperatureReading, TemperatureRequest
};

use tokio::time::Instant;

pub mod temperature {
    tonic::include_proto!("temperature");
}

mod windows_hardware_monitor;
use windows_hardware_monitor::windows_hardware_monitor::HardwareMonitor;

const BATCH_SIZE: usize = 5;
const LOOP_DURATION: Duration = Duration::from_secs(1);

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = TemperatureServiceClient::connect("http://[::1]:50051").await?;
    let hw = HardwareMonitor::new()?;

    let mut readings = Vec::with_capacity(BATCH_SIZE);

    loop {
        let start = Instant::now();

        let timestamp = prost_types::Timestamp::from(SystemTime::now());
        match hw.cpu_temp() {
            Ok(cpu) => {
                println!(
                    "Recorded: Timestamp: {}, Temperature: {}°C",
                    timestamp, cpu.value
                );
                readings.push(TemperatureReading {
                    timestamp: Some(timestamp),
                    value: Some(Temperature { value: cpu.value }) ,
                });
            }
            Err(x) => eprintln!("Timestamp: {timestamp}, error: {x}"),
        }

        if readings.len() >= BATCH_SIZE {
            let request = tonic::Request::new(TemperatureRequest {
                readings: std::mem::take(&mut readings),
            });
            match client.send_temperatures(request).await {
                Ok(_) => println!("Request was a success!"),
                Err(e) => eprintln!("Failed to send temperatures: {e}"),
            }
        }

        let elapsed = start.elapsed();
        if elapsed < LOOP_DURATION {
            tokio::time::sleep(LOOP_DURATION - elapsed).await;
        }
    }
}
