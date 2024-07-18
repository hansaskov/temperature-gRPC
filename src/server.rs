use anyhow::Result;
use clap::Parser;
use proto::temperature_service_server::{TemperatureService, TemperatureServiceServer};
use proto::{TemperatureReading, TemperatureReply, TemperatureRequest};
use sqlx::postgres::PgPoolOptions;
use tonic::{transport::Server, Request, Response, Status};

mod config;
use config::Args;

pub mod proto {
    tonic::include_proto!("temperature");
    pub(crate) const FILE_DESCRIPTOR_SET: &[u8] =
        tonic::include_file_descriptor_set!("temperature_descriptor");
}

fn process_temperature_readings(readings: &[TemperatureReading]) -> TemperatureReply {
    TemperatureReply {
        average_temperature: readings.iter().map(|r| r.value).sum::<f32>() / readings.len() as f32,
        reading_count: readings.len() as i32,
        latest_timestamp: readings.iter().map(|r| r.timestamp).max().unwrap(),
    }
}

#[derive(Debug, Default)]
pub struct MyTemperature {}

#[tonic::async_trait]
impl TemperatureService for MyTemperature {
    async fn send_temperatures(
        &self,
        request: Request<TemperatureRequest>,
    ) -> Result<Response<TemperatureReply>, Status> {
        let readings = &request.get_ref().readings;
        println!("Got the following readings: {:?}", readings);

        if readings.is_empty() {
            return Err(Status::invalid_argument("The provided request is empty"));
        }

        let reply = process_temperature_readings(readings);
        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    // Set up database connection pool
    let pool = PgPoolOptions::new()
        .max_connections(args.max_connections)
        .connect(&args.database_url)
        .await?;

    // Verify database connection
    let row: (i64,) = sqlx::query_as("SELECT $1")
        .bind(150_i64)
        .fetch_one(&pool).await?;

    println!("Connection to server successful \n {row:?}");

    // Set up temperature service
    let temperature_service = MyTemperature::default();
    let reflection_service = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(proto::FILE_DESCRIPTOR_SET)
        .build()
        .expect("Failed to create reflection service");

    // Start the server
    println!("Starting server on {}", args.server_addr);
    Server::builder()
        .add_service(reflection_service)
        .add_service(TemperatureServiceServer::new(temperature_service))
        .serve(args.server_addr.parse()?)
        .await?;

    Ok(())
}
