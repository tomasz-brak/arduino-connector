use std::time::Duration;

use comfy_table::presets::NOTHING;
use comfy_table::{Cell, Color, Table};
use log::{info, warn};
use serialport::{SerialPortInfo, available_ports};

use crate::cli::DevicesArgs;
use crate::commands::{Run, device_list};

enum PortStatus {
    Ready,
    Busy,
    Unavailable,
}

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

fn probe_port_status(port: &SerialPortInfo) -> PortStatus {
    match serialport::new(&port.port_name, 9600)
        .timeout(Duration::from_millis(10))
        .open()
    {
        Ok(_) => PortStatus::Ready,
        Err(e) => match e.kind() {
            serialport::ErrorKind::Io(std::io::ErrorKind::PermissionDenied) => PortStatus::Busy,
            serialport::ErrorKind::Io(std::io::ErrorKind::AlreadyExists) => PortStatus::Busy,
            _ => PortStatus::Unavailable,
        },
    }
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

        let mut table = Table::new();
        table.load_preset(NOTHING);

        table.set_header(vec![
            Cell::new("PORT").add_attribute(comfy_table::Attribute::Bold),
            Cell::new("NAME").add_attribute(comfy_table::Attribute::Bold),
            Cell::new("STATUS").add_attribute(comfy_table::Attribute::Bold),
        ]);

        for device in devices {
            table.add_row(vec![
                Cell::new(&device.port_name),
                Cell::new(info(&device)?),
                match probe_port_status(&device) {
                    PortStatus::Ready => Cell::new("Ready").fg(Color::Green),
                    PortStatus::Busy => Cell::new("Busy").fg(Color::Yellow),
                    PortStatus::Unavailable => Cell::new("Unavailable").fg(Color::Red),
                },
            ]);
        }

        println!("{table}");

        Ok(())
    }
}
