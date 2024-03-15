mod tools;

fn main() {
    let apps = tools::get_installed_applications();
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
