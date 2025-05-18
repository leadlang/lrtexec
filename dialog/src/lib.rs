use native_dialog::{MessageDialogBuilder, MessageLevel};

pub extern "C" fn ask_continue() -> bool {
  MessageDialogBuilder::default()
    .set_title("Warning")
    .set_text("This binary depends on Native Libraries that has to be unpacked and loaded. Native Libraries can have unwanted code. Are you sure you want to run this binary?")
    .set_level(MessageLevel::Warning)
    .confirm()
    .show()
    .unwrap_or(false)
}

pub extern "C" fn ask_install() -> bool {
  MessageDialogBuilder::default()
    .set_title("Installer")
    .set_text("Would you like to install this application?")
    .set_level(MessageLevel::Info)
    .confirm()
    .show()
    .unwrap_or(false)
}
