use ::std::process::Command;
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
        r"Realtek Semiconductor Corp."
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

                        let system_component: u32 = app_key.get_value("SystemComponent").unwrap_or(0);
                        let windows_installer: u32 = app_key.get_value("WindowsInstaller").unwrap_or(0);

                        if let (Ok(name), Ok(location), Ok(publisher)) = (name, location, publisher)
                        {
                            if !location.is_empty() && Path::new(&location).exists() {
                                applications.push(Application {
                                    name,
                                    location,
                                    publisher,
                                    // TODO: improved verification for system apps
                                    is_system: system_component == 1 || windows_installer == 1,
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
   