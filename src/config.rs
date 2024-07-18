use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(
        short,
        long,
        env,
        default_value = "postgres://username:password@localhost:5432/mydatabase"
    )]
    pub database_url: String,

    #[arg(short, long, env, default_value = "[::0]:50051")]
    pub server_addr: String,

    #[arg(short, long, env, default_value = "5")]
    pub max_connections: u32,
}
