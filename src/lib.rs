pub mod custom_error;

use std::{net::SocketAddrV4, u16};

use adb_client::{ADBDeviceExt, ADBServer, ADBServerDevice};
use custom_error::MyADBError;

pub fn launch()-> Result<(), MyADBError>{
    let mut server = ADBServer::default();
    let port = 5555;

//    let mut device = server
//        .get_device()
//        .map_or_else(|_| Err(MyADBError::NoDeviceConnected), Ok)?;


//   match server.get_device(){
//       Ok(device) => device,
//           Err(e) => {
//               eprintln!("Adb errro: {:?}", e);
//               return Err(MyADBError::ConnectionFailed);
//           }
//   };

   let mut device = server.get_device()
       .map_err(|err| match err {
         adb_client::RustADBError::DeviceNotFound(ref msg) if msg.contains("too many devices") => {
             MyADBError::TooManyDevices
         }
         adb_client::RustADBError::DeviceNotFound(_) => MyADBError::NoDeviceConnected,
         _=> MyADBError::ConnectionFailed,
       })?;

    let ip = get_device_ip(&mut device)?;

    enable_tcpip(&mut device, port)?;
    connect_to_ip(&ip, port)?;

        Ok(())
}

fn get_device_ip(device: &mut ADBServerDevice)-> Result<String, MyADBError>{

    let mut output = Vec::new();
    device.shell_command(&["ip", "-o", "-4", "addr", "show", "wlan0"], &mut output)?;

    let output_str = String::from_utf8_lossy(&output);

    let ip = output_str
        .lines()
        .next()
        .and_then(|line| line.split_whitespace().nth(3))
        .and_then(|cidr| cidr.split('/').next())
        .ok_or(MyADBError::MalformedOutput)?;


    Ok(ip.to_string())

}

fn enable_tcpip(device: &mut ADBServerDevice, port: u16) -> Result<(), MyADBError>{

    let command = format!("tcpip {}", port);
    let mut output = Vec::new();

    let _ = device.shell_command(&["sh", "-c", &command], &mut output);

    Ok(())
}

fn connect_to_ip(ip: &str, port: u16) -> Result<(), MyADBError>{
    let mut server = ADBServer::default();
    let full_addr = SocketAddrV4::new(ip.parse().expect("Invalid IP"), port);

    server.connect_device(full_addr).map_err(|e|{
        println!("Failed to connect to {full_addr}: {e}");
        MyADBError::ConnectionFailed
    })?;
    
    println!("Connected successfully to {}", full_addr);
    Ok(())
}

