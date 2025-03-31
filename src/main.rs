use std::{
    path::Path
};
use std::thread;
use std::time::Duration;

mod wallpaper;
mod play_music;
fn main() {
    /* Where our image will reside */
    let desktop_path = "C:\\users\\vigilant\\Documents\\extracted_image.jpeg";

    /* Extract chef image from PE to filesystem */
    wallpaper::extract_image_to_filesystem(
            "CHEF_IMAGE", 
            Path::new(&desktop_path)
    );

    /* set wallpaper to image */
    wallpaper::set_wallpaper(&desktop_path);

    /* Play annoying music with increasing speed and crescendo */
    thread::spawn(|| {
        play_music::play_embedded_mp3_with_increasing_speed_volume(240, 0.1, 0.2);
    });

    /* load the image for deep frying */
    println!("{}", &desktop_path);
    let mut img = image::open(&desktop_path).expect("Error: failed to load image!");

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

        deep_fried.save(&desktop_path).expect("Error: failed to save deep fried image!");

        /* set new wallpaper */
        wallpaper::set_wallpaper(&desktop_path);

        /* wait for 1 minute before continuing deep fry loop */
        thread::sleep(Duration::from_secs(1));
    }
}

