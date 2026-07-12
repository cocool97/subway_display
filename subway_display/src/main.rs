#![no_std]
#![no_main]

mod wifi;

use core::mem;

use embassy_executor::Spawner;
use embedded_io_async::Read;

use embassy_net::{DhcpConfig, Runner};
use esp_backtrace as _;

use esp_bootloader_esp_idf::esp_app_desc;

use esp_hal::{
    Config, interrupt::software::SoftwareInterruptControl, rng::Rng, timer::timg::TimerGroup,
};

use esp_println::println;
use esp_radio::wifi::Interface;

use proto::messages::{ApiInfo, Message};
use static_cell::StaticCell;

use proto::NetworkSendable;

esp_app_desc!();

#[esp_rtos::main]
async fn main(spawner: Spawner) {
    let peripherals = esp_hal::init(Config::default());

    esp_alloc::heap_allocator!(size: 64 * 1024);
    esp_alloc::heap_allocator!(#[esp_hal::ram(reclaimed)] size: 64000);

    let timg0 = TimerGroup::new(peripherals.TIMG0);

    let software_interrupt = SoftwareInterruptControl::new(peripherals.SW_INTERRUPT);

    esp_rtos::start(timg0.timer0, software_interrupt.software_interrupt0);

    let (controller, wifi_interface) = match wifi::connect_wifi(peripherals.WIFI).await {
        Ok((controller, interface)) => {
            println!("WiFi connected");
            (controller, interface)
        }
        Err(e) => {
            println!("Failed to connect to wifi: {:?}", e);
            return;
        }
    };

    let config = embassy_net::Config::dhcpv4(DhcpConfig::default());

    let rng = Rng::new();
    let seed = (rng.random() as u64) << 32 | rng.random() as u64;

    // only two socket are needed here
    // - DHCP
    // - TCP
    static RESOURCES: StaticCell<embassy_net::StackResources<2>> = StaticCell::new();
    let resources = RESOURCES.init(embassy_net::StackResources::new());

    // Init network stack
    let (stack, runner) = embassy_net::new(wifi_interface, config, resources, seed);

    spawner.spawn(net_task(runner).unwrap());

    stack.wait_config_up().await;

    if let Some(config) = stack.config_v4() {
        println!("Got IP: {}", config.address);
    }

    let mut rx_buffer = [0u8; 1024];
    let mut tx_buffer = [0u8; 1024];

    let server_endpoint =
        embassy_net::IpEndpoint::new(embassy_net::IpAddress::v4(192, 168, 1, 225), 8080);

    let mut socket = embassy_net::tcp::TcpSocket::new(stack, &mut rx_buffer, &mut tx_buffer);
    socket
        .connect(server_endpoint)
        .await
        .expect("cannot connect");

    println!("connected to server!");

    let mut data = [0u8; mem::size_of::<Message<ApiInfo>>()];

    socket.read_exact(&mut data).await.expect("cannot read");

    let message = Message::<ApiInfo>::deserialize(&data).expect("cannot deserialize");

    println!(
        "api_info: message_type={} next_subway_interval_mins={} second_subway_interval_mins={}",
        message.message_type as u8,
        message.data.next_subway_interval_mins,
        message.data.second_subway_interval_mins
    );

    controller
        .wait_for_disconnect_async()
        .await
        .expect("cannot wait for disconnect");
}

#[embassy_executor::task]
async fn net_task(mut runner: Runner<'static, Interface<'static>>) {
    runner.run().await
}
