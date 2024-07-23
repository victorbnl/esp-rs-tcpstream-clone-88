use std::net::{TcpListener, TcpStream};

use esp_idf_svc::{eventloop::EspSystemEventLoop, nvs::EspDefaultNvsPartition, wifi::{AccessPointConfiguration, BlockingWifi, Configuration, EspWifi}};

fn main() -> anyhow::Result<()> {
    esp_idf_svc::sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();

    let sys_loop = EspSystemEventLoop::take()?;
    let nvs = EspDefaultNvsPartition::take()?;
    let peripherals = esp_idf_svc::hal::peripherals::Peripherals::take()?;

    let mut wifi = BlockingWifi::wrap(EspWifi::new(peripherals.modem, sys_loop.clone(), Some(nvs))?, sys_loop)?;

    let configuration: Configuration = Configuration::AccessPoint(AccessPointConfiguration {
        ssid: "ESP32".try_into().unwrap(),
        password: "password".try_into().unwrap(),
        ..Default::default()
    });

    wifi.set_configuration(&configuration)?;
    wifi.start()?;
    wifi.wait_netif_up()?;

    std::thread::spawn(|| {
        let listener = TcpListener::bind("0.0.0.0:8080").unwrap();
        for stream in listener.incoming() {
            let stream = stream.unwrap();
            stream.try_clone().expect("EXPECTED FAILURE OF try_clone");
        }
    });

    std::thread::sleep(std::time::Duration::from_secs(1));

    TcpStream::connect("127.0.0.1:8080").unwrap();

    Ok(())
}
