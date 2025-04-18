My old java application re-written in rust🦀

# Tools used:
- Tauri
- Angular

# Adding UDP payloads
To improve port scanning it's possible to add your own payloads that will be used during the scan
- go to payloads.json located in src-tauri folder
- add a payload according to the format specified by payload.rs inside src-tauri/src, every object must have "packet" and "port" fields
- bytes of every byte array inside payloads.json must be in decimal
- payloads are bundled into executable at compile time
- you can add multiple payloads to the same port (or the same packet) by creating a new object in a list with the same port (or a packet)
- you may submit a pull request with the updated payloads.json file to improve port scanning
