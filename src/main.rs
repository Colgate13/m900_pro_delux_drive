use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{Error, ErrorKind, Read};
use std::process;
use std::thread;

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
enum EHidraws {
    Buttons,
    Details,
}

enum ENotifications {
    Battery,
    DialogInformation,
}

#[derive(Debug, Clone)]
struct Details {
    battery: u8,
}

#[derive(Debug, Clone)]
struct Mouse {
    hidraw_paths: HashMap<EHidraws, String>,
    details: Details,
}

impl Mouse {
    fn new(hid: String) -> Result<Mouse, Error> {
        let hidraw_paths: HashMap<EHidraws, String> = match Mouse::find_hidraw(hid.as_str()) {
            Ok(result) => result,
            Err(e) => {
                return Err(e);
            }
        };

        Ok(Mouse {
            details: Details { battery: 0 },
            hidraw_paths,
        })
    }

    fn find_hidraw(hid: &str) -> Result<HashMap<EHidraws, String>, Error> {
        let mut hidraw_paths_current: HashMap<EHidraws, String> = HashMap::new();
        let dir = match fs::read_dir("/sys/class/hidraw") {
            Ok(dir) => dir,
            Err(e) => return Err(e),
        };

        for entry in dir {
            let hidraw_path = entry?.path(); // Return Error if entry is Error
            let uevent_path = hidraw_path.join("device/uevent");

            if !uevent_path.exists() {
                continue;
            }

            if let Ok(mut uevent_file) = File::open(&uevent_path) {
                let mut content = String::new();
                match uevent_file.read_to_string(&mut content) {
                    Ok(_) => {}
                    Err(e) => {
                        return Err(e);
                    }
                }

                let device_hid_id = format!("{}{}", "HID_ID=", hid);

                if content.contains(device_hid_id.as_str()) {
                    if content.contains("input1") {
                        let file_name = hidraw_path.file_name();
                        match file_name {
                            Some(name) => {
                                hidraw_paths_current.insert(
                                    EHidraws::Buttons,
                                    String::from(
                                        format!("/dev/{}", name.to_string_lossy()).as_str(),
                                    ),
                                );
                            }
                            None => {
                                return Err(Error::new(ErrorKind::Other, "Error to get file name"));
                            }
                        }
                    }
                    if content.contains("input2") {
                        let file_name = hidraw_path.file_name();
                        match file_name {
                            Some(name) => {
                                hidraw_paths_current.insert(
                                    EHidraws::Details,
                                    String::from(
                                        format!("/dev/{}", name.to_string_lossy()).as_str(),
                                    ),
                                );
                            }
                            None => {
                                return Err(Error::new(ErrorKind::Other, "Error to get file name"));
                            }
                        }
                    }
                }
            }
        }

        if hidraw_paths_current.is_empty() {
            return Err(Error::new(ErrorKind::Other, "Error to get file name"));
        }

        Ok(hidraw_paths_current)
    }

    fn read(&mut self, hidraw_type: EHidraws) -> Result<bool, Error> {
        let path = match self.hidraw_paths.get(&hidraw_type) {
            Some(path) => path,
            None => {
                return Err(Error::new(ErrorKind::Other, "Error to get file name"));
            }
        };

        let mut buffer = [0u8; 7];
        let mut file = match File::open(&path) {
            Ok(file) => file,
            Err(e) => return Err(e),
        };

        match file.read(&mut buffer) {
            Ok(_) => {
                self.parse_report_to_data(hidraw_type, buffer);
            }
            Err(e) => return Err(e),
        }

        Ok(true)
    }

    fn parse_report_to_data(&mut self, hidraw_type: EHidraws, buffer: [u8; 7]) {
        match hidraw_type {
            EHidraws::Details => {
                self.details.battery = buffer[4];
                // TODO: Call functions to use battery value. Exemple: function to send notification when low battery
                self.handler_details();
            }
            EHidraws::Buttons => {
                // TODO: Call functions to use buttons values. Exemple: Handler buttons to call things
                self.handler_buttons(buffer);
            }
        }
    }

    fn handler_details(&self) {
        if self.details.battery == 5 {
            send_notification(
                String::from("Battery level is low: 5%"),
                ENotifications::Battery,
            );
        }
    }

    fn handler_buttons(&mut self, bytes: [u8; 7]) {
        let mouse1 = bytes[0];

        match mouse1 {
            24 => {
                match self.read(EHidraws::Details) {
                    Ok(_) => {}
                    Err(err) => {
                        eprintln!("Error in read details: {}", err);
                        process::exit(1);
                    }
                }
                send_notification(String::from(format!("Battery level: {}%", self.details.battery)), ENotifications::Battery);
            }
            _ => {}
        }
    }
}

fn send_notification(message: String, notification_type: ENotifications) {
    let icon: String = match notification_type {
        ENotifications::Battery => String::from("battery"),
        ENotifications::DialogInformation => String::from("dialog-information"),
    };

    process::Command::new("notify-send")
        .arg("-i")
        .arg(icon)
        .arg("M900 PRO")
        .arg(message)
        .spawn()
        .expect("Error to emitted notification");
}

fn main() {
    let mut mouse = match Mouse::new(String::from("0003:00001D57:0000FA65")) {
        Ok(instance) => instance,
        Err(err) => {
            eprintln!("Error in setup mouse: {}", err);
            process::exit(1);
        }
    };

    send_notification(String::from("On!"), ENotifications::DialogInformation);

    // Battery Monitor thread
    let mut mouse_battery_instance = mouse.clone();
    thread::spawn(move || {
        loop {
            match mouse_battery_instance.read(EHidraws::Details) {
                Ok(_) => {}
                Err(err) => {
                    eprintln!("Error in read details: {}", err);
                    process::exit(1);
                }
            }
        }
    });

    // Buttons Monitor run in main thread
    loop {
        match mouse.read(EHidraws::Buttons) {
            Ok(_) => {}
            Err(err) => {
                eprintln!("Error in read buttons: {}", err);
                process::exit(1);
            }
        }
    }
}
