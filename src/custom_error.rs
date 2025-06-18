use std::fmt;

use adb_client::RustADBError as ADBError;

#[derive(Debug)]
pub enum MyADBError{
    ADB(ADBError),
    NoDeviceConnected,
    MalformedOutput,
    ConnectionFailed,
    TooManyDevices
}

impl fmt::Display for MyADBError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MyADBError::ADB(e) => write!(f, "ADB error: {}", e),
            MyADBError::NoDeviceConnected => write!(f, "No device connected"),
            MyADBError::MalformedOutput => write!(f, "Could not parse IP from output"),
            MyADBError::ConnectionFailed => write!(f, "Could not connect to the IP"),
            MyADBError::TooManyDevices => write!(f, "Too many devices connected"),
            
        }
    }
    
}

impl std::error::Error for MyADBError {}

impl From<ADBError>  for MyADBError {
    
    fn from(err: ADBError) -> Self {
        MyADBError::ADB(err)
    }
}
