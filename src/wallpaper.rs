use std::fs::File;
use std::io::Write;
use std::path::Path;
use image::{DynamicImage, GenericImageView, RgbImage};
use rand::Rng;
use windows::{
    core::*, 
    Win32::{
        System::{
            LibraryLoader::{FindResourceA, LoadResource, LockResource, SizeofResource},
            Com::*
        },
        UI::Shell::{{IDesktopWallpaper, DesktopWallpaper}}
    }
};

pub fn extract_image_to_filesystem(resource_name: &str, output_path: &Path) {
    unsafe {
        // Create PCSTR for resource name and type
        let resource_name_c = format!("{}\0", resource_name);
        
        let rcdata_value = 10u16 as usize; // RCDATA is resource type 10 (passed to FindResourceA)
        let rt_rcdata = PCSTR(rcdata_value as *const u8);

        // Find the resource
        let h_resource = FindResourceA(
            None,
            PCSTR(resource_name_c.as_ptr() as *const u8),
            rt_rcdata
        ).unwrap_or_else(|e|{
            panic!("Error: could not find resource: {e}")
        });
        
        // Get resource size
        let resource_size = SizeofResource(None, h_resource);
        if resource_size == 0 {
           panic!("Error: resource has a size of zero");
        }
        
        // Load the resource
        let h_loaded = LoadResource(None, h_resource).unwrap_or_else(|e| {
            panic!("Error: failed to load resource: {e}");
        });
        
        // Get pointer to resource data
        let data_ptr = LockResource(h_loaded) as *const u8;
        if data_ptr.is_null() {
            panic!("Error: failed to get pointer for resource");
        }
        
        // Create a slice from the resource data
        let image_data = std::slice::from_raw_parts(data_ptr, resource_size as usize);
        
        // Create output file
        let mut file = File::create(output_path).unwrap_or_else(|e| {
            panic!("Error: Failed to create output file: {e}")
        });
        
        // Write data to file
        file.write_all(image_data).unwrap_or_else(|e| {
            panic!("Error: Failed to create output file: {e}")
        });
    }
}


pub fn set_wallpaper(wallpaper_path: &str) {
    /* To manipulate the wallpaper, we must do it through the IDesktopWallpaper COM interface */
    unsafe {
        CoInitialize(None);
        
        /*  Create an instance of IDesktopWallpaper */
        let wallpaper: IDesktopWallpaper = CoCreateInstance(
            &DesktopWallpaper as *const _, 
            None, 
            CLSCTX_ALL
        ).unwrap_or_else(|e| {
            panic!("Error: failed to create instance of DesktopWallpaper: {e}")
        });
        
        /* Convert the path to a wide string */
        let wide_path = HSTRING::from(wallpaper_path);
        
        /* Set the wallpaper */
        wallpaper.SetWallpaper(None, &wide_path).unwrap_or_else(|e| {
            panic!("Error: failed to set desktop wallpaper: {e}")
        });
        
        CoUninitialize();
        
    }
}

pub fn deep_fry(original: &DynamicImage, intensity: f32) -> DynamicImage {
    let (width, height) = original.dimensions();
    let mut img = original.clone();

    /* increase saturation + contrast of image */
    img = increase_saturation(&img, 1.0 + intensity * 2.0);
    img = increase_contrast(&img, 1.0 + intensity * 1.5);

    /* add noise */
    img = add_noise(&img, intensity * 0.5);

    /* add redness and increase brigthness */
    img = add_red_tint(&img, intensity * 0.5);
    img = increase_brightness(&img, 1.0 + intensity * 0.1);

    if intensity > 0.5 {
        img = apply_bulge(&img, (intensity - 0.5) * 2.0);
    }

    return img
}

fn add_red_tint(img: &DynamicImage, intensity: f32) -> DynamicImage {
    let mut result = img.clone();
    for (_, _, pixel) in result.as_mut_rgb8().unwrap().enumerate_pixels_mut() {
        
        let new_red = (pixel[0] as f32 + (255.0 - pixel[0] as f32) * intensity).clamp(0.0, 255.0) as u8;
        pixel[0] = new_red;
        
        if intensity > 0.2 {
            let blue_reduction = pixel[2] as f32 * (intensity * 0.2);
            pixel[2] = (pixel[2] as f32 - blue_reduction).clamp(0.0, 255.0) as u8;
        }
    }
    result
}

fn increase_brightness(img: &DynamicImage, factor: f32) -> DynamicImage {
    let mut result = img.clone();
    for (_, _, pixel) in result.as_mut_rgb8().unwrap().enumerate_pixels_mut() {
        for c in 0..3 {
            let new_value = pixel[c] as f32 * factor;
            pixel[c] = new_value.clamp(0.0, 255.0) as u8;
        }
    }
    result
}

fn increase_saturation(img: &DynamicImage, factor: f32) -> DynamicImage {
    let mut result = img.clone();
    for (x, y, pixel) in result.as_mut_rgb8().unwrap().enumerate_pixels_mut() {
        let [r, g, b] = [pixel[0] as f32, pixel[1] as f32, pixel[2] as f32];
        
        let max = r.max(g).max(b);
        let min = r.min(g).min(b);
        let delta = (max - min) * factor;
        
        pixel[0] = ((r - min) * factor + min).clamp(0.0, 255.0) as u8;
        pixel[1] = ((g - min) * factor + min).clamp(0.0, 255.0) as u8;
        pixel[2] = ((b - min) * factor + min).clamp(0.0, 255.0) as u8;
    }
    result
}

fn increase_contrast(img: &DynamicImage, factor: f32) -> DynamicImage {
    let mut result = img.clone();
    for (_, _, pixel) in result.as_mut_rgb8().unwrap().enumerate_pixels_mut() {
        for c in 0..3 {
            let value = pixel[c] as f32;

            let new_value = 128.0 + (value - 128.0) * factor;
            pixel[c] = new_value.clamp(0.0, 255.0) as u8;
        }
    }
    result
}

fn add_noise(img: &DynamicImage, intensity: f32) -> DynamicImage {
    let mut rng = rand::thread_rng();
    let mut result = img.clone();
    
    if let Some(buffer) = result.as_mut_rgb8() {
        for pixel in buffer.pixels_mut() {
            for c in 0..3 {
                if rng.random_range(0.0..1.0) < intensity {
                    /* generate noise value - either positive or negative */
                    let noise = if rng.random_bool(0.5) {
                        rng.random_range(0..=50)
                    } else {
                        -rng.random_range(0..=50)
                    };
                    
                    let value = pixel[c] as i32;
                    let new_value = (value + noise).clamp(0, 255) as u8;
                    pixel[c] = new_value;
                }
            }
        }
    }
    
    return result
}

/* distorting image by adding "bulge" effect */
fn apply_bulge(img: &DynamicImage, intensity: f32) -> DynamicImage {
    let (width, height) = img.dimensions();
    let center_x = width as f32 / 2.0;
    let center_y = height as f32 / 2.0;
    let max_radius = (width.min(height) / 2) as f32;
    
    let mut result = RgbImage::new(width, height);
    
    for y in 0..height {
        for x in 0..width {
            let dx = x as f32 - center_x;
            let dy = y as f32 - center_y;
            let distance = (dx * dx + dy * dy).sqrt();
            
            if distance < max_radius {
                let bulge_amount = 1.0 - intensity * (distance / max_radius);
                let source_x = center_x + dx * bulge_amount;
                let source_y = center_y + dy * bulge_amount;
                
                let source_x_floor = source_x.floor();
                let source_y_floor = source_y.floor();
                let x_frac = source_x - source_x_floor;
                let y_frac = source_y - source_y_floor;
                
                let x1 = source_x_floor as u32;
                let y1 = source_y_floor as u32;
                let x2 = (x1 + 1).min(width - 1);
                let y2 = (y1 + 1).min(height - 1);
                
                let p11 = img.get_pixel(x1, y1);
                let p12 = img.get_pixel(x1, y2);
                let p21 = img.get_pixel(x2, y1);
                let p22 = img.get_pixel(x2, y2);
                
                let mut pixel_color = [0u8; 3];
                for c in 0..3 {
                    let top = p11[c] as f32 * (1.0 - x_frac) + p21[c] as f32 * x_frac;
                    let bottom = p12[c] as f32 * (1.0 - x_frac) + p22[c] as f32 * x_frac;
                    let value = top * (1.0 - y_frac) + bottom * y_frac;
                    pixel_color[c] = value.clamp(0.0, 255.0) as u8;
                }
                
                result.put_pixel(x, y, image::Rgb(pixel_color));
            } else {
                result.put_pixel(x, y, image::Rgb(img.get_pixel(x, y).0[0..3].try_into().unwrap()));
            }
        }
    }
    
    DynamicImage::ImageRgb8(result)
}