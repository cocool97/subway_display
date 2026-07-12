use std::net::SocketAddr;

use clap::{ArgAction, Parser};

use crate::constants::{DEFAULT_API_POLL_INTERVAL_SECS, DEFAULT_LISTEN_ADDRESS};

#[derive(Parser)]
pub struct Opts {
    /// Set the verbosity level (from 0 to 2 occurences)
    #[clap(short, env, default_value = "0", action = ArgAction::Count)]
    pub versosity: u8,
    /// Set the address to bind to
    #[clap(short, env, default_value = DEFAULT_LISTEN_ADDRESS)]
    pub listen_address: SocketAddr,
    /// Poll interval to get latest API results
    #[clap(short = 'i', long, env, default_value = DEFAULT_API_POLL_INTERVAL_SECS)]
    pub poll_interval_secs: u64,
    /// API Key for PRIM
    /// Can be retrieved from <https://prim.iledefrance-mobilites.fr/fr/mes-jetons-authentification>
    #[clap(short = 'k', long, env)]
    pub prim_api_key: String,
    /// `MonitoringRef` for the station to monitor
    #[clap(long, env)]
    pub monitoring_ref: String,
    /// `LineRef` for the line to monitor
    #[clap(long, env)]
    pub line_ref: String,
}
