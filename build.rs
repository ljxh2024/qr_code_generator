fn main() {
  let c = slint_build::CompilerConfiguration::new().with_style("cosmic".into());
  slint_build::compile_with_config("ui/app-window.slint", c).unwrap();
}