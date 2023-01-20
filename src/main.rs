#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] 
mod app;
mod spotify;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    app::run();
    Ok(())
}
