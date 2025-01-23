fn main() {
  let config = slint_build::CompilerConfiguration::new().with_style("cosmic".into());
  slint_build::compile_with_config("ui/app-window.slint", config).unwrap();

  if cfg!(target_os = "windows") {
    let mut res = winresource::WindowsResource::new();
    res.set_icon("ui/assets/32x32.ico").compile().unwrap();
  }
}
