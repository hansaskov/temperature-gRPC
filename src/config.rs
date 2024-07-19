use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(short, long = "db-url", env)]
    pub database_url: String,

    #[arg(short, long ="addres", env, default_value = "[::0]:50051")]
    pub server_addres: String,

    #[arg(short, long , env, default_value = "5")]
    pub max_connections: u32,
}
