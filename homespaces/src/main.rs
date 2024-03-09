use deunicode::deunicode;
use std::io::{self, Write};
use std::process::{Command, Stdio};

struct Application {
    name: String,
    location: String,
    is_system: bool,
}

fn main() {
    let ps_command = r#"Get-ItemProperty HKLM:\Software\Wow6432Node\Microsoft\Windows\CurrentVersion\Uninstall\* |
    Where-Object { $_.DisplayName -and $_.InstallLocation } |
    Select-Object DisplayName, InstallLocation, @{Name="IsSystem"; Expression={if ($_.SystemComponent -eq 1) {"Yes"} else {"No"}}} |
    Format-Table -AutoSize | Out-String -Width 4096"#;

    let output: std::process::Output = Command::new("powershell")
        .args(&["-Command", ps_command])
        .output()
        .expect("Failed to execute command");

    let output_str = String::from_utf8_lossy(&output.stdout);

    let mut result: Vec<String> = output_str
        .lines()
        .filter_map(|line| {
            let name = line.trim();
            // TODO - better conditions ensuring only apps are displayed, not based on constant values that may vary between users and language versions
            // for now it's Windows 11 specific
            if !name.is_empty() && name != "DisplayName" && name != "-----------" {
                Some(deunicode(name).to_string())
            } else {
                None
            }
        })
        .collect();

    result.sort_unstable_by(|a, b| {
        let a_lower = a.to_lowercase();
        let b_lower = b.to_ascii_lowercase();
        a_lower.cmp(&b_lower)
    });

    for app in result {
        println!("{}", app);
    }

    let mut app_to_run = String::new();
    println!("Enter app name");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut app_to_run).unwrap();

    let lines = output_str.lines();
    let mut found = false;

    for line in lines {
        if line.contains("DisplayName") && line.contains(&app_to_run) {
            found = true;
        } else if found && line.contains("InstallLocation") {
            let install_location = line.split(":").nth(1).unwrap_or("").trim();
            if !install_location.is_empty() {
                println!("Attempting to open {}", install_location);
                if let Ok(_) = Command::new("explorer")
                    .arg(install_location)
                    .stdout(Stdio::inherit())
                    .stderr(Stdio::inherit())
                    .spawn()
                {
                    println!("Opened");
                } else {
                    println!("Failed to open");
                }
                break;
            }
        }
    }
}
