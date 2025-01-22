#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::cell::RefCell;
use std::rc::Rc;
use slint::{ Image, PlatformError, Rgb8Pixel, SharedPixelBuffer };
use qrcode::QrCode;
use image::{ Rgb, ExtendedColorType, ImageFormat, save_buffer_with_format };
use rand::{ distributions::Alphanumeric, Rng, thread_rng };
use qr_code_generator::AppData;

slint::include_modules!();

fn main() -> Result<(), PlatformError> {
    let ui = MainWindow::new()?;

    let app_data = AppData {
        save_count: Rc::new(RefCell::new(0)),
        file_stem: Rc::new(RefCell::new(String::from(""))),
    };

    // 生成二维码
    ui.on_generate_qr_code({
        let ui_handle = ui.as_weak();

        let save_count = Rc::clone(&app_data.save_count);
        let file_stem = Rc::clone(&app_data.file_stem);

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

                ui.set_qr_code(Image::from_rgb8(buffer));

                *save_count.borrow_mut() = 0;
                *file_stem.borrow_mut() = thread_rng().sample_iter(&Alphanumeric).take(8).map(char::from).collect();
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
            let mut file_stem = app_data.file_stem.borrow().to_string();

            if *app_data.save_count.borrow() > 0 {
                file_stem += &format!("({})", app_data.save_count.borrow());
            }

            let result = rfd::FileDialog::new()
                .add_filter("PNG 图片文件", &["png"])
                .set_file_name(file_stem + ".png")
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
                *app_data.save_count.borrow_mut() += 1;
            }
        }
    });

    ui.run()?;
    Ok(())
}