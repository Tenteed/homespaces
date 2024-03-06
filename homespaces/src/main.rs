use deunicode::deunicode;
use std::process::Command;

fn main() {
    let output: std::process::Output = Command::new("powershell")
        .args(&[
            "-Command",
            "Get-ItemProperty HKLM:\\Software\\Wow6432Node\\Microsoft\\Windows\\CurrentVersion\\Uninstall\\* | Select-Object DisplayName | Format-Table -AutoSize",
        ])
        .output()
        .expect("Failed to execute command");

    let output_str: std::borrow::Cow<'_, str> = String::from_utf8_lossy(&output.stdout);

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
}
