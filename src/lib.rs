use std::cell::RefCell;
use std::rc::Rc;
use slint::{ Image, Rgb8Pixel, SharedPixelBuffer };
use qrcode::QrCode;
use image::{ Rgb, ExtendedColorType, ImageFormat, save_buffer_with_format };
use chrono::Local;

slint::include_modules!();

pub fn main() {
  let window = init();
  window.run().unwrap();
}

fn init() -> MainWindow {
  let window = MainWindow::new().unwrap();

  let app_data = AppData {
    save_count: Rc::new(RefCell::new(0)),
    file_stem: Rc::new(RefCell::new(String::from("")))
  };

  // 生成二维码
  window.on_generate_qr_code({
    let weak_window = window.as_weak();

    let save_count = Rc::clone(&app_data.save_count);
    let file_stem = Rc::clone(&app_data.file_stem);

    move |s| {
      let ui = weak_window.unwrap();
      let s = s.trim();

      if s.len() > 0 {
        let img = QrCode::new(s).unwrap().render::<Rgb<u8>>().build();
        let buffer = SharedPixelBuffer::<Rgb8Pixel>::clone_from_slice(img.as_raw(), img.width(), img.height());

        *save_count.borrow_mut() = 0;
        *file_stem.borrow_mut() = format!("qr_code_{}", Local::now().timestamp());

        ui.set_qr_code(Image::from_rgb8(buffer));
      } else {
        ui.set_qr_code(Image::from_rgb8(SharedPixelBuffer::<Rgb8Pixel>::new(0, 0)));
      }
    }
  });

  // 保存二维码
  window.on_save_qr_code({
    let weak_window = window.as_weak();

    move || {
      let ui = weak_window.unwrap();

      let dir = std::env::current_dir().unwrap();
      let mut file_stem = app_data.file_stem.borrow().to_string();

      if *app_data.save_count.borrow() > 0 {
        file_stem += &format!("({})", app_data.save_count.borrow());
      }

      // 打开保存文件对话框
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

        *app_data.save_count.borrow_mut() += 1;
        ui.invoke_show_success_popup();
      }
    }
  });

  window
}

struct AppData {
  // 文件名称（不含扩展名）
  file_stem: Rc<RefCell<String>>,
  // file_stem 保存次数
  save_count: Rc<RefCell<u8>>
}