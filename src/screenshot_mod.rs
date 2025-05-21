use chrono::Local;
use screenshots::Screen;
use std::{
    fs::{self, File},
    io::BufWriter,
    path::{Path, PathBuf},
    thread,
    time::Duration,
};

use image::codecs::jpeg::JpegEncoder;
use image::{imageops::FilterType, ImageBuffer, Rgba};

/// Capture a screenshot, optionally resize, and save it as JPEG.
fn capture_and_save_screenshot(output_dir: &Path, resolution: Option<(u32, u32)>) {
    if let Some(screen) = get_primary_screen() {
        match screen.capture() {
            Ok(image_buf) => {
                let timestamp = Local::now().format("%Y-%m-%d_%H-%M-%S").to_string();
                let file_path = output_dir.join(format!("screenshot_{}.jpg", timestamp));

                if let Some(img) = convert_to_image_buffer(&image_buf) {
                    let final_img = match resolution {
                        Some((w, h)) => image::imageops::resize(&img, w, h, FilterType::Triangle),
                        None => img,
                    };

                    match File::create(&file_path) {
                        Ok(file) => {
                            let writer = BufWriter::new(file);
                            let mut encoder = JpegEncoder::new_with_quality(writer, 80);
                            if let Err(e) = encoder.encode_image(&final_img) {
                                eprintln!("Failed to encode and save JPEG: {}", e);
                            }
                        }
                        Err(e) => {
                            eprintln!("Failed to create screenshot file: {}", e);
                        }
                    }
                } else {
                    eprintln!("Failed to convert screenshot buffer to image.");
                }
            }
            Err(e) => {
                eprintln!("Error capturing screenshot: {}", e);
            }
        }
    }
}

/// Convert raw screenshot buffer to `ImageBuffer`.
fn convert_to_image_buffer(image: &screenshots::Image) -> Option<ImageBuffer<Rgba<u8>, Vec<u8>>> {
    let width = image.width();
    let height = image.height();
    let buffer = image.rgba();
    ImageBuffer::from_raw(width, height, buffer.to_vec())
}

/// Get the primary screen to capture.
fn get_primary_screen() -> Option<Screen> {
    match Screen::all() {
        Ok(screens) => screens.into_iter().next(),
        Err(e) => {
            eprintln!("Failed to enumerate screens: {}", e);
            None
        }
    }
}

/// Ensure the screenshots directory exists.
fn ensure_output_dir(base_dir: &Path) -> Option<PathBuf> {
    let dir = base_dir.join("screenshots");
    if let Err(e) = fs::create_dir_all(&dir) {
        eprintln!("Failed to create screenshot directory: {}", e);
        return None;
    }
    Some(dir)
}

/// Start screenshot loop with optional resolution.
pub fn start_screenshot_loop(
    output_base_dir: &Path,
    interval_secs: u64,
    stop_flag: bool,
    resolution: Option<(u32, u32)>,
) -> thread::JoinHandle<()> {
    let output_dir = match ensure_output_dir(output_base_dir) {
        Some(dir) => dir,
        None => return thread::spawn(|| {}),
    };

    thread::spawn(move || {
        while !stop_flag {
            capture_and_save_screenshot(&output_dir, resolution);
            thread::sleep(Duration::from_secs(interval_secs));
        }
    })
}

/// Capture one screenshot with optional resolution.
pub fn capture_one_screenshot(
    output_base_dir: &Path,
    capture_flag: bool,
    resolution: Option<(u32, u32)>,
) {
    if !capture_flag {
        return;
    }

    if let Some(output_dir) = ensure_output_dir(output_base_dir) {
        capture_and_save_screenshot(&output_dir, resolution);
    }
}
