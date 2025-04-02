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

pub fn extract_resource_to_filesystem(resource_name: &str, output_path: &Path) {
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