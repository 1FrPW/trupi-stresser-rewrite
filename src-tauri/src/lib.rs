use std::{
    array,
    net::UdpSocket,
    ops::Sub,
    thread::{self},
    time::{Duration, Instant},
};

#[tauri::command]
fn send_packets(target_address: String, data_size: usize) {
    thread::spawn(move || {
        let socket = UdpSocket::bind("127.0.0.1:0").unwrap();

        let target_address = target_address + ":0";
        let packet: [u8; 1024] = array::from_fn(|_| u8::MAX as u8);
        let remaining_time = Duration::new(1, 0);
        let mut time_elapsed;

        loop {
            time_elapsed = Instant::now();
            for i in (0 as usize)..data_size {
                match (&socket).send_to(&packet, &target_address) {
                    Ok(bytes_written) => println!("{i} succesfully sent {} bytes", bytes_written),
                    Err(error) => println!("failed sending bytes: {}", error),
                }
            }
            thread::sleep((&remaining_time).sub(time_elapsed.elapsed()));
        }
    });
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![send_packets])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
