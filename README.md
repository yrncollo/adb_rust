# Rust ADB Connector

This is a simple Rust tool for connecting to Android devices via ADB (Android Debug Bridge).  
It uses the `adb_client` crate to interface with the ADB server and detect connected devices over USB or TCP/IP.

## Features

- Connect to Android devices via ADB
- List available devices
- Retrieve device IP address
- Built with safety and custom error handling

## Requirements

- Rust (latest stable recommended)
- ADB installed and running (`adb devices`)
- Android device with USB debugging enabled

## Usage

1. Clone the repo:
   ```bash
   git clone https://github.com/yrncollo/adb_rust.git
   cd adb_rust

   ```

2. Run the project
    ```bash
    cargo run 
    ```

> Make sure you have rust installed

