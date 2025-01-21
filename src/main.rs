#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use slint::{ Image, PlatformError, Rgb8Pixel, SharedPixelBuffer };
use qrcode::QrCode;
use image::{RgbImage, Rgb};
use nfd2::Response;
use std::path::Path;
use rand::{distributions::Alphanumeric, thread_rng, Rng};

slint::include_modules!();

fn main() -> Result<(), PlatformError>{
    let ui = MainWindow::new()?;

    // 生成二维码
    ui.on_generate_qrcode({
        let ui_handle = ui.as_weak();

        move |s| {
            let ui = ui_handle.unwrap();
            let s = s.trim();

            if s.len() > 0 {
                let code = QrCode::new(s).unwrap();
                let image = code.render::<Rgb<u8>>().build();
                let buffer = SharedPixelBuffer::<Rgb8Pixel>::clone_from_slice(
                    image.as_raw(),
                    image.width(),
                    image.height()
                );

                ui.set_qrcode(Image::from_rgb8(buffer));
                ui.set_filename(generate_filename().into());
            } else {
                ui.set_qrcode(Image::from_rgb8(SharedPixelBuffer::<Rgb8Pixel>::new(0, 0)));
            }
        }
    });

    // 保存二维码
    ui.on_save_qrcode({
        let ui_handle = ui.as_weak();

        move || {
            let ui = ui_handle.unwrap();

            let file = r"C:\Users\Public\Downloads\".to_string() + &ui.get_filename();
            let res = nfd2::open_save_dialog(Some("png"), Some(Path::new(&file))).unwrap();
            match res {
                // todo: 文件已存在的情况
                Response::Okay(path) => {
                    let rgb = ui.get_qrcode().to_rgb8().unwrap();
                    let bytes = rgb.as_bytes();
                    let mut img = RgbImage::new(rgb.width(), rgb.height());
                    for (x, y, pixel) in img.enumerate_pixels_mut() {
                        let index = ((y * rgb.width() + x) * 3) as usize;
                        *pixel = Rgb([bytes[index], bytes[index + 1], bytes[index + 2]]);
                    }
                    img.save(path).unwrap();

                    ui.set_is_save_success(true);
                },
                _ => ()
            }
        }
    });

    ui.run()?;
    Ok(())
}

// 生成随机文件名
fn generate_filename() -> String {
    let s: String = thread_rng().sample_iter(&Alphanumeric).take(8).map(char::from).collect();
    format!("{s}.png")
}