use windows::{
    Win32::UI::{
        Shell::ShellExecuteA,
        WindowsAndMessaging::{SW_SHOW, MessageBoxA, MB_OK, MB_ICONWARNING}
    },
    core::PCSTR
};
use rand::prelude::IndexedRandom;
use rand::Rng;
use std::{
    time::Duration,
    thread
};

const OPERATION: PCSTR = PCSTR("open\0".as_ptr());

pub fn popup_images_randomly(image_paths: Vec<String>, count: usize, min_delay_ms: u64, max_delay_ms: u64) {
    let mut rng = rand::rng();

    unsafe {
        for _ in 0..count {
            if let Some(random_image_path) = &image_paths.choose(&mut rng) {
                /* image_path needs to be a null-terminated c string */
                let image_path_null = format!("{}\0", random_image_path);
                let file = PCSTR(image_path_null.as_ptr());
        
                 ShellExecuteA(
                    None,
                    OPERATION,
                    file,
                    None,
                    None,
                    SW_SHOW,
                );
            }
            let delay = rng.random_range(min_delay_ms..=max_delay_ms);
            thread::sleep(Duration::from_millis(delay));
        }
    }
}

pub fn popup_dominos_randomly(count: usize, min_delay_ms: u64, max_delay_ms: u64){
    let mut rng = rand::rng();
    let url = PCSTR("https://dominos.com\0".as_ptr());

    unsafe {
        for _ in 0..count {
            ShellExecuteA(
                None,
                OPERATION,
                url,
                None,
                    None,
                    SW_SHOW,
            );
            let delay = rng.random_range(min_delay_ms..=max_delay_ms);
            thread::sleep(Duration::from_millis(delay));
        }
    }
}

pub fn popup_message(message: &str) {
    unsafe {
        let message_null = format!("{}\0", message);
        let title_null = "Warning\0";
                
        MessageBoxA(
            None,
            PCSTR(message_null.as_ptr()),
            PCSTR(title_null.as_ptr()),
            MB_OK | MB_ICONWARNING,
        );
    }
}