#[tokio::main]
async fn main() {
    let _ = pdfy::cli::run_cli().await;
    hide_window();
    // boquilahub::gui::run_gui();
}

#[cfg(target_os = "windows")]
fn hide_window() {
    use winapi::um::wincon::FreeConsole;
    unsafe {
        FreeConsole(); // Detaches and closes the console window
    }
}