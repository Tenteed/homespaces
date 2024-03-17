use ::std::process::Command;
use enigo::{Enigo, Key, KeyboardControllable};
use std::collections::VecDeque;
use std::path::Path;
use winreg::enums::*;
use winreg::RegKey;

// TODO file with common types
#[derive(Clone, Debug)]
pub struct Application {
    pub name: String,
    pub location: String,
    pub publisher: String,
    pub is_system: bool,
}

pub fn get_installed_applications() -> VecDeque<Application> {
    let mut applications: Vec<Application> = Vec::new();
    let known_system_publishers: Vec<&str> = vec![
        r"NVIDIA Corporation",
        r"Microsoft Corporation",
        r"Advanced Micro Devices, Inc.",
        r"Realtek",
    ];

    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let uninstall_paths: Vec<&str> = vec![
        r"SOFTWARE\Microsoft\Windows\CurrentVersion\Uninstall",
        r"SOFTWARE\WOW6432Node\Microsoft\Windows\CurrentVersion\Uninstall",
    ];

    for path in uninstall_paths.iter() {
        if let Ok(uninstall) = hklm.open_subkey_with_flags(path, KEY_READ) {
            for subkey_result in uninstall.enum_keys() {
                if let Ok(subkey_name) = subkey_result {
                    if let Ok(app_key) = uninstall.open_subkey_with_flags(&subkey_name, KEY_READ) {
                        let name: Result<String, _> = app_key.get_value("DisplayName");
                        let location: Result<String, _> = app_key.get_value("InstallLocation");
                        let publisher: Result<String, _> = app_key.get_value("Publisher");

                        if let (Ok(name), Ok(location), Ok(publisher)) = (name, location, publisher)
                        {
                            if !location.is_empty() && Path::new(&location).exists() {
                                applications.push(Application {
                                    name,
                                    location,
                                    publisher,
                                    // TODO: is_system based on whether the publisher is in the known publishers list
                                    is_system: false,
                                });
                            }
                        }
                    }
                }
            }
        }
    }

    applications.into()
}

pub fn start_application(app_path: String) {
    match Command::new(app_path).spawn() {
        Ok(_) => println!("App started"),
        Err(e) => eprintln!("{}", e),
    }
}

pub fn create_desktop() {
    let mut enigo = Enigo::new();

    enigo.key_down(Key::Meta);
    enigo.key_down(Key::Control);
    enigo.key_click(Key::Layout('d'));
    enigo.key_up(Key::Meta);
    enigo.key_up(Key::Control);
}

pub fn list_apps() {
    let apps = get_installed_applications();
    for app in apps {
        // Skipping system apps for now, might add an option to show them later on
        if !app.is_system {
            println!(
                "Name: {}, Location: {}, Publisher: {}",
                app.name, app.location, app.publisher
            );
        }
    }
}
