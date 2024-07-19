use std::time::Duration;
use temperature::{
    temperature_service_client::TemperatureServiceClient, TemperatureReading, TemperatureRequest,
};
use tempurature_grpc::windows_hardware_monitor::HardwareMonitor;
use tokio::time::Instant;

pub mod temperature {
    tonic::include_proto!("temperature");
}

mod time_helper;
use time_helper::TimeHelper;

const BATCH_SIZE: usize = 5;
const LOOP_DURATION: Duration = Duration::from_secs(1);

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = TemperatureServiceClient::connect("http://[::1]:50051").await?;
    let hw = HardwareMonitor::new()?;

    let mut readings = Vec::with_capacity(BATCH_SIZE);

    loop {
        let start = Instant::now();

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
