// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn read_file(name: &str) -> String {
    std::fs::read_to_string(name).unwrap()
}

#[tauri::command]
fn list_files(basepath: &str) -> Vec<String> {
    // List all files in a directory recursively
    // Return the full file names
    let mut files = Vec::new();
    for entry in std::fs::read_dir(basepath).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_dir() {
            files.push(path.to_str().unwrap().to_owned() + "/");
            files.append(&mut list_files(path.to_str().unwrap()));
        } else {
            files.push(path.to_str().unwrap().to_string());
        }
    }

    return files;
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![read_file, list_files])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
