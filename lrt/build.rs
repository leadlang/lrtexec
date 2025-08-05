fn main() -> Result<(), Box<dyn std::error::Error>> {
  if cfg!(target_os = "windows") {
    use tauri_winres;

    let gh = if option_env!("CI").map_or_else(|| false, |x| x == "true") {
      "Built with GitHub Actions"
    } else {
      "Built Locally"
    };

    let mut res = tauri_winres::WindowsResource::new();
    res.set_icon("LRT.ico")
      .set("CompanyName", "The Lead Programming Language Foundation")
      .set("ProductName", "Lead Runtime Interpreter Suite")
      .set("FileDescription", "This is an interpreter program built under the \"Lead Runtime Interpreter Suite\"")
      .set("LegalCopyright", "The Lead Programming Language Foundation")
      .set("Comments", gh)
      .set_language(0x0009);
    
    res.compile()?;
  }

  Ok(())
}