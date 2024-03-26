use std::collections::VecDeque;
use tauri::{State, Config};
mod system_tools;
mod user_apps;


struct AppViewer {
    applications: VecDeque<user_apps::Application>,
}

fn main() {
    
}
