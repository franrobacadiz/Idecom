use crate::client::translate;
#[cfg(windows)]
use crate::ipc::Data;
#[cfg(windows)]
use hbb_common::tokio;
use hbb_common::{allow_err, log};
use std::sync::{Arc, Mutex};
#[cfg(windows)]
use std::time::Duration;

pub fn start_tray() {
    if crate::ui_interface::get_builtin_option(hbb_common::config::keys::OPTION_HIDE_TRAY) == "Y" {
        #[cfg(target_os = "macos")]
        {
            loop {
                std::thread::sleep(std::time::Duration::from_secs(1));
            }
        }
        #[cfg(not(target_os = "macos"))]
        {
            return;
        }
    }
    allow_err!();
}

