#[derive(clap::Parser, Clone)]
pub struct Config {
    #[arg(long, env)]
    pub database_url: String,

    #[arg(long, env)]
    pub port: String,
}
