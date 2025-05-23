use std::fs::{self, File};
use std::io::{Read, BufReader, BufRead};
use std::path::Path;

fn main() {
    let target_hid_id = "0003:00001D57:0000FA65";

    let device_path = match find_hidraw_by_hid_id(target_hid_id) {
        Some(path) => path,
        None => {
            eprintln!("Dispositivo com HID_ID {} não encontrado.", target_hid_id);
            return;
        }
    };

    println!("Dispositivo encontrado: {}", device_path);

    let mut file = File::open(&device_path).expect("Erro ao abrir HID");

    loop {
        let mut buffer = [0u8; 5]; // Revicer 5 bytes
        match file.read(&mut buffer) {
            Ok(5) => {
                let report_id = buffer[0];
                let battery = buffer[4]; // battery

                println!(
                    "Report ID: {:#04X}, Dados: {:02X} {:02X} {:02X} {:02X} => ⚡ Bateria: {}%",
                    report_id,
                    buffer[1], buffer[2], buffer[3], battery,
                    battery
                );
            }
            Ok(n) => {
                println!("Leitura parcial: {} bytes", n);
            }
            Err(e) => {
                eprintln!("Erro ao ler: {}", e);
                break;
            }
        }

        std::thread::sleep(std::time::Duration::from_secs(30));
    }
}

// Find hidraw by hid id
fn find_hidraw_by_hid_id(target_hid_id: &str) -> Option<String> {
    let base_path = Path::new("/sys/class/hidraw");

    for entry in fs::read_dir(base_path).ok()? {
        let entry = entry.ok()?;
        let hidraw_path = entry.path();
        let uevent_path = hidraw_path.join("device/uevent");

        if uevent_path.exists() {
            if let Ok(file) = File::open(&uevent_path) {
                let reader = BufReader::new(file);
                for line in reader.lines().flatten() {
                    if line.starts_with("HID_ID=") && line.contains(target_hid_id) {
                        // Mounting /dev/hidrawX
                        if let Some(file_name) = hidraw_path.file_name() {
                            return Some(format!("/dev/{}", file_name.to_string_lossy()));
                        }
                    }
                }
            }
        }
    }

    None
}
