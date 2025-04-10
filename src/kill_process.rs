use std::time::Duration;
use std::thread::sleep;
use windows::Win32::{
        Foundation::{CloseHandle, INVALID_HANDLE_VALUE},
        System::{
            Threading::{OpenProcess, PROCESS_TERMINATE, TerminateProcess},
            Diagnostics::ToolHelp::{CreateToolhelp32Snapshot, Process32FirstW, Process32NextW, PROCESSENTRY32W, TH32CS_SNAPPROCESS},
        },
    };

pub fn kill_processes(target_processes: &[&str]) -> windows::core::Result<()> {
    loop {
        /* Take a snapshot of all processes */
        let h_snapshot = unsafe { CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0) }?;
        if h_snapshot == INVALID_HANDLE_VALUE {
            panic!("Error: failed to create snapshot");
        }
        
        let mut process_entry = PROCESSENTRY32W::default();
        process_entry.dwSize = std::mem::size_of::<PROCESSENTRY32W>() as u32;
        
        /* Get the first process */
        let mut result = unsafe { Process32FirstW(h_snapshot, &mut process_entry) };
        
        while !result.is_err() {
            /* Convert process name to Rust string */
            let process_name = unsafe {
                let name_ptr = process_entry.szExeFile.as_ptr();
                let len = process_entry.szExeFile.iter()
                    .position(|&c| c == 0)
                    .unwrap_or(process_entry.szExeFile.len());
                let name_slice = std::slice::from_raw_parts(name_ptr, len);
                String::from_utf16_lossy(name_slice)
            };
            
            /* Check if this is a target process */
            let process_name_lower = process_name.to_lowercase();
            if target_processes.iter().any(|&target| target.to_lowercase() == process_name_lower) {
                let process_id = process_entry.th32ProcessID;
                
                /* Open the process with termination rights */
                let process_handle = unsafe {
                    OpenProcess(PROCESS_TERMINATE, false, process_id)
                };
                
                if let Ok(handle) = process_handle {
                    /* Terminate the process */
                    let _result = unsafe { TerminateProcess(handle, 1) }.unwrap_or_else(|e| {
                        panic!("Error: failed to terminate process: {e}")
                    });
                    /* Close the handle */
                    unsafe { let _ = CloseHandle(handle); };
                } else {
                    panic!("Failed to open process handle for {}", process_name);
                }
            }
            
            /* Move to the next process */ 
            result = unsafe { Process32NextW(h_snapshot, &mut process_entry) };
        }
        
        /* Close the snapshot handle */ 
        unsafe { let _ = CloseHandle(h_snapshot); };
        
        /* Wait before checking again */
        sleep(Duration::from_millis(100));
    }
}