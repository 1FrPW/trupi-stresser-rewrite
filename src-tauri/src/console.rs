#[cfg(windows)]
mod console {
    use std::ptr::null_mut;
    use winapi::um::consoleapi::AllocConsole;
    use winapi::um::fileapi::CreateFileA;
    use winapi::um::handleapi::INVALID_HANDLE_VALUE;
    use winapi::um::processenv::SetStdHandle;
    use winapi::um::winbase::{STD_ERROR_HANDLE, STD_OUTPUT_HANDLE};
    use winapi::um::winnt::{
        FILE_ATTRIBUTE_NORMAL, FILE_SHARE_READ, FILE_SHARE_WRITE, GENERIC_READ, GENERIC_WRITE,
    };

    pub fn spawn_console() {
        unsafe {
            AllocConsole();
            let con = CreateFileA(
                b"CONOUT$\0".as_ptr() as *const i8,
                GENERIC_READ | GENERIC_WRITE,
                FILE_SHARE_READ | FILE_SHARE_WRITE,
                null_mut(),
                3, // OPEN_EXISTING
                FILE_ATTRIBUTE_NORMAL,
                null_mut(),
            );
            if con != INVALID_HANDLE_VALUE {
                SetStdHandle(STD_OUTPUT_HANDLE, con);
                SetStdHandle(STD_ERROR_HANDLE, con);
            }
            println!("Allocated Windows console.");
        }
    }
}

#[cfg(unix)]
mod console {
    use std::process::Command;

    pub fn spawn_console() {
        let terminal_cmds = vec![
            (
                "xterm",
                vec!["-e", "bash -c 'echo Console started; exec bash'"],
            ),
            (
                "gnome-terminal",
                vec!["--", "bash", "-c", "echo Console started; exec bash"],
            ),
            (
                "konsole",
                vec!["-e", "bash", "-c", "echo Console started; exec bash"],
            ),
        ];

        for (term, args) in terminal_cmds {
            if Command::new("which").arg(term).output().is_ok() {
                let _ = Command::new(term).args(args).spawn();
                return;
            }
        }

        eprintln!("No supported terminal emulator found.");
    }
}

pub use console::spawn_console;