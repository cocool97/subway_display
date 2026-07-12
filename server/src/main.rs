mod constants;
mod opts;
mod prim_api_manager;
mod tcp_listener;
mod utils;

use anyhow::Result;
use clap::Parser;

use crate::{
    opts::Opts, prim_api_manager::PrimApiManager, tcp_listener::TcpServer, utils::setup_logger,
};

fn main() -> Result<()> {
    let opts = Opts::parse();

    setup_logger(opts.versosity);

    log::info!("starting server v{}...", env!("CARGO_PKG_VERSION"));
    log::info!("listening on {}", opts.listen_address);
    log::debug!("monitoring line {}", opts.line_ref);
    log::debug!("monitoring stop {}", opts.monitoring_ref);

    let prim_api_manager =
        PrimApiManager::new(opts.prim_api_key, opts.line_ref, opts.monitoring_ref);

    let tcp_server = TcpServer::new(
        opts.listen_address,
        opts.poll_interval_secs,
        prim_api_manager,
    )?;
    tcp_server.serve_forever()
}
