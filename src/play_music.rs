use windows::{
    core::PCSTR,
    Win32::{
        Media::MediaFoundation::{MFStartup, MFSTARTUP_FULL, MF_VERSION}, System::{
            Com::{CoInitializeEx, COINIT_MULTITHREADED},
            LibraryLoader::{FindResourceA, LoadResource, LockResource, SizeofResource}
        }
    }
};
use rodio::{Decoder, OutputStream, Sink, Source};
use std::io::Cursor;

pub fn play_embedded_mp3_with_increasing_speed_volume(iterations: usize, speed_increment: f32, volume_increment: f32) {
    unsafe {
        /* Initializing COM and media foundation */
        let _ = CoInitializeEx(None, COINIT_MULTITHREADED);
        let _ = MFStartup(MF_VERSION, MFSTARTUP_FULL);

        /* Load MP3 from PE */
        let mp3_name = PCSTR("ITALIAN_MP3\0".as_ptr() as *const u8);
        let rcdata_value = 10u16 as usize;
        let rt_rcdata = PCSTR(rcdata_value as *const u8);

        let h_resource = FindResourceA(None, mp3_name, rt_rcdata).unwrap_or_else(|e| {
            panic!("Error: failed to find mp3 file: {e}");
        });

        let h_loaded = LoadResource(None, h_resource).unwrap_or_else(|e| {
            panic!("Error: failed to load mp3 file: {e}");
        });
        let resource_size = SizeofResource(None, h_resource);
        let mp3_data = LockResource(h_loaded) as *const u8;

        /* Put mp3 raw bytes in a data vec, initialize rodio requisites */
        let data_vec = std::slice::from_raw_parts(mp3_data, resource_size as usize).to_vec();
        let original_data = data_vec.clone();

        /* handle mp3 iterations with increasing speed and volume */
        for i in 0..iterations {

        let cursor = Cursor::new(original_data.clone());
        let (_stream, stream_handle) = OutputStream::try_default().unwrap_or_else(|e| {
            panic!("Error: failed to initialize audio output stream: {e}");
        });
        let sink = Sink::try_new(&stream_handle).unwrap_or_else(|e|{
            panic!("Error: failed to initialize audio sink: {e}");
        });
        let source = Decoder::new(cursor).unwrap_or_else(|e|{
            panic!("Error: failed to decode mp3 data: {e}");
        });

        /* Calculate speed and volume for iteration */
        let speed = 1.0 + (i as f32 * speed_increment);
        let volume = 1.0 + (i as f32 * volume_increment);

        let speed_adjusted = source.speed(speed);
        let volume_adjusted = speed_adjusted.amplify(volume);

        /* add source to sink */
        sink.append(volume_adjusted);

        sink.sleep_until_end();
    }

    }
}