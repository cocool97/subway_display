use anyhow::Result;
use proto::{
    NetworkSendable,
    messages::{ApiInfo, Message, MessageType},
};
use std::{
    io::Write,
    net::{TcpListener, TcpStream, ToSocketAddrs},
    thread::sleep,
    time::Duration,
};

use crate::prim_api_manager::PrimApiManager;

pub struct TcpServer {
    listener: TcpListener,
    poll_interval: Duration,
    prim_api_manager: PrimApiManager,
}

impl TcpServer {
    pub fn new<A: ToSocketAddrs>(
        address: A,
        poll_interval: u64,
        prim_api_manager: PrimApiManager,
    ) -> Result<Self> {
        let listener = TcpListener::bind(address)?;

        Ok(Self {
            listener,
            poll_interval: Duration::from_secs(poll_interval),
            prim_api_manager,
        })
    }

    pub fn serve_forever(self) -> Result<()> {
        loop {
            let (tcp_stream, remote_client_addr) = self.listener.accept()?;
            tcp_stream.set_read_timeout(None)?;
            tcp_stream.set_write_timeout(None)?;

            log::info!("received connection from {remote_client_addr}...");

            if let Err(e) = self.serve_client(tcp_stream) {
                log::error!("failed to serve client: {e}");
            }

            log::warn!("client {remote_client_addr} disconnected... waiting for new connection.");
        }
    }

    pub fn serve_client(&self, mut tcp_stream: TcpStream) -> Result<()> {
        loop {
            let latest_api_results = self.prim_api_manager.get_next_subway_arrivals()?;

            let message = Message {
                message_type: MessageType::ApiInfo,
                data: latest_api_results,
            };

            let mut data = [0u8; Message::<ApiInfo>::OUT_SIZE];
            message.serialize(&mut data);

            tcp_stream.write_all(&data)?;

            log::debug!(
                "next API results push: {} seconds",
                self.poll_interval.as_secs()
            );
            sleep(self.poll_interval);
        }
    }
}
