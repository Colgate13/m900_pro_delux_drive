# Delux M900 Pro Mouse Battery Drive for Linux

## Overview

This is a simple program written in **pure Rust** to monitor the battery level of the **Delux M900 Pro** mouse on **Linux** systems. It uses the **HIDRAW** interface directly, without external libraries, to read battery and button data. Notifications are sent via `notify-send`.

## Features

* Automatically detects the device based on the HID ID `0003:00001D57:0000FA65`.
* Continuously monitors:

  * **Battery percentage** via the details channel (`input2`).
  * **Button actions** via the buttons channel (`input1`).
* Sends desktop notifications:

  * When the battery is low (5%).
  * When the appropriate button is pressed, showing the current battery level (Mouse3 + Mouse4 = Code(24)).

## Requirements

* **Linux** with access to `/sys/class/hidraw` and `/dev/hidraw*` devices.
* **Rust** installed ([rust-lang.org](https://www.rust-lang.org/tools/install)).
* Read permissions for `/dev/hidraw*` (might require `sudo` or `udev` rules).
* `notify-send` installed (typically included with `libnotify`).

## How to Use

1. Clone this repository and enter the project folder.
2. Build with:

   ```bash
   cargo build --release
   ```
3. Run the binary with proper permissions:

   ```bash
   sudo ./target/release/m900_drive_linux
   ```

## TODOS

* [ ] Implement button handling for other actions.
* [ ] Improve notification system (e.g., prevent message spamming).
* [ ] Make the HID ID user-configurable.
* [ ] Add support for other mouse models.

## Future Ideas

* [ ] Add graphical user interface (GUI).
* [ ] Log events to a file.
* [ ] Allow dynamic updates without restarting the program.
