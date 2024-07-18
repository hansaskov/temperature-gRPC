use anyhow;
use clap::Parser;
use proto::temperature_service_server::{TemperatureService, TemperatureServiceServer};
use proto::{TemperatureReading, TemperatureRequest};
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use tonic::{transport::Server, Request, Response, Status};
use sqlx::Row;

mod config;
use config::Args;

mod time_helper;
use time_helper::TimeHelper;

pub mod proto {
    tonic::include_proto!("temperature");
}
pub struct MyTemperature {
    pool: sqlx::PgPool
}

#[tonic::async_trait]
impl TemperatureService for MyTemperature {
    async fn send_temperatures(
        &self,
        request: Request<TemperatureRequest>,
    ) -> tonic::Result<Response<proto::Empty>, Status> {
        let readings = &request.get_ref().readings;
        println!("Got the following readings: {:?}", readings);

        if readings.is_empty() {
            return Err(Status::invalid_argument("The provided request is empty"));
        }

        insert_many_readings(readings, &self.pool).await.unwrap();

        let res = sqlx::query("SELECT * FROM conditions")
            .fetch_all(&self.pool)
            .await
            .unwrap();


        for row in res {
            let reading = TemperatureReading {
                timestamp: Some(TimeHelper::from_offset_date_time(row.get("time"))),
                value: row.get("temperature") 
            };

            println!("{reading:?}");
        }

        let empty = proto::Empty {};

        Ok(Response::new(empty))
    }
}

pub async fn insert_many_readings(
    readings: &[TemperatureReading],
    pool: &PgPool,
) -> anyhow::Result<()> {
    if readings.is_empty() {
        return Err(anyhow::anyhow!("No readings provided"));
    }

    let mut times = Vec::new();
    let mut temperatures = Vec::new();

    for reading in readings {
        if let Some(timestamp) = &reading.timestamp {
            times.push(TimeHelper::to_offset_date_time(timestamp));
            temperatures.push(reading.value);
        }
    }

    if times.is_empty() {
        return Err(anyhow::anyhow!("No valid readings with timestamps found"));
    }

    sqlx::query(
        "
        INSERT INTO conditions (time, temperature)
        SELECT * FROM UNNEST($1::timestamptz[], $2::float8[])
        "
    )
    .bind(times)
    .bind(temperatures)
    .execute(pool)
    .await?;

    Ok(())
}


#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let pool = PgPoolOptions::new()
        .max_connections(args.max_connections)
        .connect(&args.database_url)
        .await?;

    println!("Connection to DB was successful");

    // Set up temperature service
    let temperature_service = MyTemperature{ pool };

    // Start the server
    println!("Starting server on {}", args.server_addr);
    Server::builder()
        .add_service(TemperatureServiceServer::new(temperature_service))
        .serve(args.server_addr.parse()?)
        .await?;

    Ok(())
}
