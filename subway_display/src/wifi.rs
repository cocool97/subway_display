use esp_hal::peripherals::WIFI;
use esp_println::println;
use esp_radio::wifi::{
    Config, Interface, PowerSaveMode, WifiController, WifiError, sta::StationConfig,
};

pub async fn connect_wifi(
    wifi_peripherical: WIFI<'static>,
) -> Result<(WifiController<'static>, Interface<'static>), WifiError> {
    let (mut controller, interfaces) = esp_radio::wifi::new(wifi_peripherical, Default::default())?;

    let station_config = Config::Station(
        StationConfig::default()
            .with_ssid(env!("WIFI_SSID"))
            .with_password(env!("WIFI_PASSWORD").into()),
    );

    controller.set_power_saving(PowerSaveMode::Maximum)?;

    controller.set_config(&station_config)?;

    controller.connect_async().await?;

    println!("signal strength: {}", controller.ap_info()?.signal_strength);
    println!("country: {:?}", controller.ap_info()?.country);

    Ok((controller, interfaces.station))
}
