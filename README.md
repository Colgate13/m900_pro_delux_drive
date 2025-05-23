# Delux M900 Pro Mouse Battery Monitor

## Overview
This project is a simple Linux driver written in Rust to monitor the battery percentage of the Delux M900 Pro mouse. It reads data from the mouse's HID (Human Interface Device) interface and extracts the battery level, displaying it in the console at regular intervals.

## Features
- Detects the Delux M900 Pro mouse using its HID ID (`0003:00001D57:0000FA65`).
- Reads battery percentage from the mouse's HID reports.
- Outputs the battery level and raw HID data every 30 seconds.
- Handles errors gracefully if the device is not found or cannot be read.

## Requirements
- **Linux OS** with access to `/sys/class/hidraw` and `/dev/hidraw*` devices.
- **Rust** (install via `rustup`, see [rust-lang.org](https://www.rust-lang.org/tools/install)).
- Root or sufficient permissions to access `/dev/hidraw*` devices (e.g., via `sudo` or udev rules).

## How It Works
- The program searches for the Delux M900 Pro mouse by matching its HID ID in `/sys/class/hidraw/*/device/uevent`.
- Once found, it opens the corresponding `/dev/hidrawX` device.
- It reads 5-byte HID reports, where the last byte represents the battery percentage.
- The battery level and raw data are printed to the console every 30 seconds.