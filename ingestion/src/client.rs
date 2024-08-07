use anyhow::{Context, Result};
use std::time::{Duration, SystemTime};
use temperature::{
    temperature_service_client::TemperatureServiceClient, Temperature, TemperatureReading,
    TemperatureRequest,
};
use tokio::{signal, sync::mpsc, time::interval};
pub mod temperature {
    tonic::include_proto!("temperature");
}
mod windows_hardware_monitor;
use windows_hardware_monitor::windows_hardware_monitor::HardwareMonitor;

const BATCH_SIZE: usize = 5;
const LOOP_DURATION: Duration = Duration::from_secs(1);

#[tokio::main]
async fn main() -> Result<()> {
    let mut client = TemperatureServiceClient::connect("http://[::1]:50051").await?;
    let hw = HardwareMonitor::new().context("Failed to initialize hardware monitor")?;
    let mut readings = Vec::with_capacity(BATCH_SIZE);

    let (shutdown_send, mut shutdown_recv) = mpsc::channel(1);
    tokio::spawn(async move {
        signal::ctrl_c().await.expect("Failed to listen for Ctrl+C");
        shutdown_send
            .send(())
            .await
            .expect("Failed to send shutdown signal");
    });

    let mut interval = interval(LOOP_DURATION);
    loop {
        tokio::select! {
            _ = interval.tick() => {
                if let Ok(reading) = record_temperature(&hw) {
                    readings.push(reading);
                    if readings.len() >= BATCH_SIZE {
                        send_readings(&mut client, &mut readings).await?;
                    }
                }
            }
            _ = shutdown_recv.recv() => {
                if !readings.is_empty() {
                    send_readings(&mut client, &mut readings).await?;
                }
                println!("Shutting down gracefully.");
                break;
            }
        }
    }
    Ok(())
}

fn record_temperature(hw: &HardwareMonitor) -> Result<TemperatureReading> {
    let timestamp = prost_types::Timestamp::from(SystemTime::now());
    let cpu_temp = hw.cpu_temp()?;
    println!(
        "Recorded: Timestamp: {}, Temperature: {}°C",
        timestamp, cpu_temp.value
    );
    Ok(TemperatureReading {
        timestamp: Some(timestamp),
        temperature: Some(Temperature {
            value: cpu_temp.value,
        }),
    })
}

async fn send_readings(
    client: &mut TemperatureServiceClient<tonic::transport::Channel>,
    readings: &mut Vec<TemperatureReading>,
) -> Result<()> {
    let request = tonic::Request::new(TemperatureRequest {
        readings: std::mem::take(readings),
    });
    client
        .send_temperatures(request)
        .await
        .context("Failed to send temperatures")?;
    println!("Request was a success!");
    Ok(())
}
