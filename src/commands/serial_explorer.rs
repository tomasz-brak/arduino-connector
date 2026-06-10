use log::{info, warn};
use serialport::{SerialPortInfo, available_ports};

use crate::cli::DevicesArgs;
use crate::commands::{Run, device_list};

fn info(s: &SerialPortInfo) -> Result<String, Box<dyn std::error::Error>> {
    Ok(match &s.port_type {
        serialport::SerialPortType::UsbPort(usb_info) => {
            device_list::find_device(usb_info.vid, usb_info.pid)
                .or(usb_info.product.as_deref())
                .unwrap_or("Generic usb device")
                .to_string()
        }
        serialport::SerialPortType::PciPort => "Build in PciPort".to_string(),
        _ => "Not Recognised".to_string(),
    })
}

impl Run for DevicesArgs {
    fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        let devices = match available_ports() {
            Ok(d) => d,
            Err(e) => {
                warn!("Failed to list available serial inteface devices, {}", e);
                return Ok(());
            }
        };

        println!("Found {} devices", devices.len());

        println!("PORT\tINFO");
        for device in devices {
            println!("{}\t{}", device.port_name, info(&device)?);
        }

        Ok(())
    }
}
