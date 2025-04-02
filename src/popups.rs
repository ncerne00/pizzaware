use windows::{
    Win32::UI::{
        Shell::ShellExecuteA,
        WindowsAndMessaging::SW_SHOW
    },
    core::PCSTR
};
use rand::prelude::IndexedRandom;
use rand::Rng;
use std::{
    time::Duration,
    thread
};

pub fn popup_images_randomly(image_paths: Vec<String>, count: usize, min_delay_ms: u64, max_delay_ms: u64) {
    let mut rng = rand::rng();
    let operation = PCSTR("open\0".as_ptr());

    unsafe {
        for _ in 0..count {
            if let Some(random_image_path) = &image_paths.choose(&mut rng) {
                /* image_path needs to be a null-terminated c string */
                let image_path_null = format!("{}\0", random_image_path);
                let file = PCSTR(image_path_null.as_ptr());
        
                 ShellExecuteA(
                    None,
                    operation,
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