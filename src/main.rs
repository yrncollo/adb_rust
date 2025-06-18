use adb_rust::{custom_error::MyADBError, launch};

fn main()-> Result<(), MyADBError> {
    launch()?;
    Ok(())
}
