use temperature::{TemperatureReply, TemperatureRequest, TemperatureReading};
use temperature::temperature_service_server::{TemperatureService, TemperatureServiceServer};
use tonic::IntoRequest;
use tonic::{transport::Server, Request, Response, Status};


pub mod temperature {
    tonic::include_proto!("temperature"); // The string specified here must match the proto package name
}

fn process_temperature_readings(readings: &[TemperatureReading]) -> TemperatureReply {
    let reading_count = readings.len() as i32;
    
    if reading_count == 0 {
        return TemperatureReply{
            average_temperature: 0.0,
            reading_count: 0,
            latest_timestamp: 0
        };
    }

    TemperatureReply {
        average_temperature: readings.iter().map(|r| r.value).sum::<f32>() / reading_count as f32,
        reading_count,
        latest_timestamp: readings.iter().map(|r| r.timestamp).max().unwrap(),
    }
}


#[derive(Debug, Default)]
pub struct MyTemperature {}

#[tonic::async_trait]
impl TemperatureService for MyTemperature {
    async fn send_temperatures(
        &self,
        request: Request<TemperatureRequest>
    ) -> Result<Response<TemperatureReply>, Status> {
        // Extract the inner TemperatureRequest from the Request object
        let temp_request = request.into_inner();
        
        println!("Got a request: {:?}", temp_request);
        
        // Now you can access the fields of temp_request directly
        let reply = process_temperature_readings(&temp_request.readings);
        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::0]:50051".parse()?;
    let temperature_service = MyTemperature::default();

    Server::builder()
        .add_service(TemperatureServiceServer::new(temperature_service))
        .serve(addr)
        .await?;
    


    Ok(())
}