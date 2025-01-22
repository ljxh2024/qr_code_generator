#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use slint::{ Image, PlatformError, Rgb8Pixel, SharedPixelBuffer };
use qrcode::QrCode;
use image::{ Rgb, ExtendedColorType, ImageFormat, save_buffer_with_format };
use rand::{ distributions::Alphanumeric, Rng, thread_rng };

slint::include_modules!();

fn main() -> Result<(), PlatformError> {
    let ui = MainWindow::new()?;

    // 生成二维码
    ui.on_generate_qr_code({
        let ui_handle = ui.as_weak();

        move |s| {
            let ui = ui_handle.unwrap();
            let s = s.trim();

            if s.len() > 0 {
                let img = QrCode::new(s).unwrap().render::<Rgb<u8>>().build();
                let buffer = SharedPixelBuffer::<Rgb8Pixel>::clone_from_slice(
                    img.as_raw(),
                    img.width(),
                    img.height()
                );
                let file_id: String = thread_rng().sample_iter(&Alphanumeric).take(8).map(char::from).collect();

                ui.set_qr_code(Image::from_rgb8(buffer));
                ui.set_file_id(file_id.into());
                ui.set_save_count(0);
            } else {
                ui.set_qr_code(Image::from_rgb8(SharedPixelBuffer::<Rgb8Pixel>::new(0, 0)));
            }
        }
    });

    // 保存二维码
    ui.on_save_qr_code({
        let ui_handle = ui.as_weak();

        move || {
            let ui = ui_handle.unwrap();

            let dir = std::env::current_dir().unwrap();
            let save_count = ui.get_save_count();
            let mut file_id = ui.get_file_id();

            if save_count > 0 {
                file_id += &format!("({save_count})")
            }

            let result = rfd::FileDialog::new()
                .add_filter("PNG 图片文件", &["png"])
                .set_file_name(file_id + ".png")
                .set_directory(&dir)
                .save_file();
            if let Some(path) = result {
                let img = ui.get_qr_code().to_rgb8().unwrap();
                save_buffer_with_format(
                    path,
                    img.as_bytes(),
                    img.width(),
                    img.height(),
                    ExtendedColorType::Rgb8,
                    ImageFormat::Png
                ).unwrap();

                ui.set_is_save_success(true);
                ui.set_save_count(save_count + 1);
            }
        }
    });

    ui.run()?;
    Ok(())
}