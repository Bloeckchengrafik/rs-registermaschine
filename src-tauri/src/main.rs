// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use lazy_static::lazy_static;
use crate::vm::{CompileError, ExecutionError, ExecutionResult};

mod vm;

lazy_static! {
    // workspace
    pub static ref WORKSPACE: String = {
        let userprofile = std::env::var("USERPROFILE");
        if userprofile.is_ok() {
            userprofile.unwrap() + "/registermaschine-workspace/"
        } else {
            "~/registermaschine-workspace/".to_owned()
        }
    };

    pub static ref VM: std::sync::Mutex<vm::VirtualMachine> = std::sync::Mutex::new(vm::VirtualMachine::new());
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn read_file(name: &str) -> String {
    std::fs::read_to_string(name).unwrap()
}

#[tauri::command]
fn get_workspace() -> String {
    WORKSPACE.clone()
}

#[tauri::command]
fn vm_compile(filepath: &str) -> String {
    println!("Compiling {}", filepath);
    let mut vm = VM.lock().unwrap();
    vm.reuse();
    let path_buf = std::path::PathBuf::from(filepath);
    return if let Err(error) = vm.load(&path_buf) {
        let error_str = match error {
            CompileError::ParamError{file, line} => format!("Param error@{}:{}", file, line),
            CompileError::LabelError{file, line} => format!("Label error@{}:{}", file, line),
            CompileError::InvalidInstruction{file, line} => format!("Invalid Instruction@{}:{}", file, line),
        };

        error_str
    } else {
        "Compile OK".to_owned()
    };
}

#[tauri::command]
fn vm_step() -> Result<ExecutionResult, ExecutionError> {
    let mut vm = VM.lock().unwrap();
    let a = vm.step();
    println!("{:?}", a);

    a
}

#[tauri::command]
fn vm_upload(numbers: Vec<String>){
    let mut vm = VM.lock().unwrap();
    println!("Uploading {:?} to VM", numbers);
    let mut numbers_int = Vec::new();
    for number in numbers {
        numbers_int.push(number.parse::<u32>().unwrap());
    }
    vm.memory = numbers_int;
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
    std::fs::create_dir_all(WORKSPACE.clone()).unwrap();

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            read_file,
            list_files,
            get_workspace,
            vm_compile,
            vm_step,
            vm_upload
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
