use std::io;
use std::fs;
use std::io::Read;
use std::thread;
use std::time::Duration;
use std::time::{SystemTime, UNIX_EPOCH};
use temperature::{
    temperature_service_client::TemperatureServiceClient, TemperatureReading, TemperatureRequest,
};

pub mod temperature {
    tonic::include_proto!("temperature"); // The string specified here must match the proto package name
}

fn read_file(path: &str) -> io::Result<String> {
    let mut s = String::new();
    fs::File::open(path)?.read_to_string(&mut s)?;
    Ok(s)
}

fn cpu_temp() -> io::Result<f32> {
    let data = read_file("/sys/class/hwmon/hwmon2/temp1_input")?;
    let temp: f32 = data
        .trim()
        .parse()
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Could not parse float"))?;
    Ok(temp / 1000.0)
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = TemperatureServiceClient::connect("http://[::1]:50051").await?;
    let mut readings = Vec::new();

    for _ in 0..5 {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();
        
        match cpu_temp() {
            Ok(cpu_temp) => {
                println!("Recorded: Timestamp: {timestamp}, Temperature: {cpu_temp:.2}°C");
                readings.push(TemperatureReading {
                    timestamp: timestamp,
                    value: cpu_temp,
                });
            }
            Err(_) => println!("Timestamp: {timestamp}, No Value read")
        }
        
        thread::sleep(Duration::from_secs(1));
    }

    let request = tonic::Request::new(TemperatureRequest { readings });
    let response = client.send_temperatures(request).await?;
    println!("RESPONSE={response:?}");

    Ok(())
}
