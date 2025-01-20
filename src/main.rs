use slint::{ Image, PlatformError, Rgb8Pixel, SharedPixelBuffer };
use qrcode::QrCode;
use image::Rgb;

slint::include_modules!();

fn main() -> Result<(), PlatformError>{
    let ui = MainWindow::new()?;

    let ui_handle = ui.as_weak();
    ui.on_generate_qr_code(move |s| {
        let ui = ui_handle.unwrap();
        let text = s.trim();

        if text.len() > 0 {
            let code = QrCode::new(text).unwrap();
            let image = code.render::<Rgb<u8>>().build();
            let buffer = SharedPixelBuffer::<Rgb8Pixel>::clone_from_slice(
                image.as_raw(),
                image.width(),
                image.height()
            );
            ui.set_qrcodeurl(Image::from_rgb8(buffer));
        }
    });

    ui.run()?;
    Ok(())
}