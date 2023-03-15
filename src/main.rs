use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Please provide binary path");
        return;
    }

    let binary_path = &args[1];

    // Get optional arguments (--name, --icon, --type, --assets)
    let mut name = binary_path.split("/").last().unwrap().to_string();
    let mut icon = None;
    let mut type_ = "Application";
    let mut assets: Vec<String> = Vec::new();

    for i in 2..args.len() {
        if args[i] == "--name" {
            name = args[i + 1].clone();
        } else if args[i] == "--icon" {
            icon = Some(args[i + 1].clone());
        } else if args[i] == "--type" {
            type_ = &args[i + 1];
        } else if args[i] == "--assets" {
            // --assets "path1, path2, path3"
            assets = args[i + 1].split(",").map(|s| s.trim().to_string()).collect();
        }
    }

    // dlink directory exists?
    if !fs::metadata("/etc/dlink").is_ok() {
        fs::create_dir("/etc/dlink").unwrap();
    }

    // Move binary to /etc/dlink/{dir}
    fs::create_dir(format!("/etc/dlink/{}", name)).unwrap();
    fs::copy(binary_path, format!("/etc/dlink/{}/bin", name)).unwrap();

    // Move assets to /etc/dlink/{dir}
    for a in assets {
        fs::copy(&a, format!("/etc/dlink/{}/{}", name, a.split("/").last().unwrap().to_string())).unwrap();
    }

    // Create .desktop file
    let desktop_file = format!(
        "[Desktop Entry]
Name={0}
Exec=/etc/dlink/{0}/bin
Icon={1}
Type={2}
Categories=Application;",
        name,
        icon.unwrap_or("".to_string()),
        type_
    );

    fs::write(
        format!("/usr/share/applications/{}.desktop", name),
        desktop_file,
    )
    .unwrap();
}
