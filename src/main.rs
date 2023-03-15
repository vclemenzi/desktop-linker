use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Please provide binary path");
        return;
    }

    let binary_path = &args[1];

    // Get optional arguments (--name, --icon, --version)
    let mut name = binary_path.split("/").last().unwrap().to_string();
    let mut icon = None;
    let mut type_ = "Application";

    for i in 2..args.len() {
        if args[i] == "--name" {
            name = args[i + 1].clone();
        } else if args[i] == "--icon" {
            icon = Some(args[i + 1].clone());
        } else if args[i] == "--type" {
            type_ = &args[i + 1];
        }
    }

    // dlink directory exists?
    if !fs::metadata("/etc/dlink").is_ok() {
        fs::create_dir("/etc/dlink").unwrap();
    }

    // Move binary to /etc/dlink/{dir}
    fs::create_dir(format!("/etc/dlink/{}", name)).unwrap();
    fs::copy(binary_path, format!("/etc/dlink/{}/bin", name)).unwrap();

    // Create .desktop file
    let desktop_file = format!(
        "[Desktop Entry]
Name={0}
Exec=/etc/dlink/{0}/bin
Icon={1}
Type={2}
Categories=Application;",
        name, icon.unwrap_or("".to_string()), type_
    );

    fs::write(
        format!("/usr/share/applications/{}.desktop", name),
        desktop_file,
    )
    .unwrap();
}
