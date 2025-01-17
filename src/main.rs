use std::{env, fs::File, path::PathBuf, process::exit};

use futures::StreamExt;
use librespot_core::config::DeviceType;
use librespot_discovery::Discovery;
use log::{debug, error, info, warn};
use sha1::{Digest, Sha1};
use std::io::Write;

struct Arguments {
    force: bool,
    path: PathBuf,
}

fn parse_arguments(args: &Vec<String>) -> Result<Arguments, String> {
    let mut force: Option<bool> = None;
    let mut path: Option<PathBuf> = None;

    let mut skip = 1;
    // If '--' is provided, all arguments before it are skipped
    if let Some(index) = args.iter().position(|arg| arg == "--") {
        skip = index + 1;
    }

    let args = &args[skip..];
    debug!("Arguments: {:?}", args);

    for arg in args.iter() {
        match arg.as_str() {
            "-f" | "--force" => {
                if force.is_some() {
                    return Err("Force flag provided multiple times".to_string());
                }
                force = Some(true)
            }
            _ => {
                if path.is_some() {
                    return Err("Path provided multiple times".to_string());
                }

                // Parse the path, validate that the argument looks like a path
                let parsed = PathBuf::from(arg);
                debug!("Parsed path: {}", parsed.display());

                if parsed.exists() {
                    if parsed.is_dir() {
                        path = Some(parsed.join("credentials.json"));
                    } else if parsed.is_file() {
                        if path.is_some() {
                            return Err("Path provided multiple times".to_string());
                        }
                        path = Some(parsed);
                    } else {
                        return Err("Path is not a file or directory".to_string());
                    }
                } else {
                    // File does not exist, check if it looks like a directory
                    if parsed.ends_with("/") || parsed.ends_with("\\") {
                        // If the parent directory exists, it's okay to create a directory then the file in it
                        if parsed.parent().is_some_and(|p| p.exists()) {
                            path = Some(parsed.join("credentials.json"));
                        } else {
                            return Err(
                                "Cannot create more than one folder for output path".to_string()
                            );
                        }
                    } else {
                        // No need to create a directory, just create the file
                        if parsed.parent().is_some_and(|p| p.exists()) {
                            path = Some(parsed);
                        } else {
                            return Err(
                                "Cannot create a file in a non-existent directory".to_string()
                            );
                        }
                    }
                }
            }
        }
    }

    // If no path was provided, default to credentials.json in the current directory
    let path = match path {
        Some(p) => match std::path::absolute(p) {
            Ok(p) => p,
            Err(e) => return Err(format!("Invalid path: {}", e)),
        },
        None => {
            // No path provided, try to identify a default in the current directory
            let pwd = env::current_dir();
            if pwd.is_err() {
                // For some reason the current directory
                return Err("Current directory is invalid or indeterminate, please provide an explicit output path".to_string());
            }

            // Default to credentials.json in the current directory
            pwd.unwrap().join("credentials.json")
        }
    };

    // If the *file* already exists, check if the force flag was provided
    if path.exists() && path.is_file() && !force.unwrap_or(false) {
        return Err(format!(
            "Output file already exists, use -f to overwrite ({})",
            path.display()
        ));
    }

    Ok(Arguments {
        force: force.unwrap_or(false),
        path,
    })
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    // Initialize the logger
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "info")
    }
    env_logger::builder().init();

    // Parse the arguments
    let args = match parse_arguments(&env::args().collect()) {
        Ok(a) => a,
        Err(e) => {
            error!("Error parsing arguments: {}", e);
            exit(1);
        }
    };

    info!("Credentials file: {}", &args.path.display());

    // TODO: If spotifyd is running, ask if shutdown is desired

    // Figure out the username
    let mut username = match env::consts::OS {
        "windows" => env::var("USERNAME"),
        _ => env::var("USER"),
    }
    // Trim whitespace from the username
    .map(|u| u.trim().to_string())
    .unwrap_or("unknown".to_string());

    // Default the username to 'unknown' if it doesn't fit the expected format
    if username != "unknown" {
        let valid_characters = r"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789-_";
        if match &username {
            u if u.is_empty() => {
                warn!("Cannot determine username, defaulting to 'unknown'");
                true
            }
            u if u.len() > 20 => {
                warn!("Username is too long, defaulting to 'unknown'");
                true
            }
            u if u.len() < 2 => {
                warn!("Username is too short, defaulting to 'unknown'");
                true
            }
            u if u.contains(|c| !valid_characters.contains(c)) => {
                warn!("Username contains invalid characters, defaulting to 'unknown'");
                true
            }
            _ => false,
        } {
            username = "unknown".to_string();
        }
    }

    // Create the device metadata
    let device_name = format!("spotify-quickauth-{}", username);
    let device_id = hex::encode(Sha1::digest(device_name.as_bytes()));
    let device_type = DeviceType::Computer;

    let mut server = Discovery::builder(device_id)
        .name(device_name.clone())
        .device_type(device_type)
        .launch()
        .unwrap();

    info!("Open Spotify and select output device: {}", device_name);

    while let Some(credentials) = server.next().await {
        // Check if file exists
        if args.path.exists() && !args.force {
            warn!("Output file already exists (appeared after startup), use -f to overwrite");
            exit(1);
        }

        // Write the credentials to the file
        let result = File::create(&args.path).and_then(|mut file| {
            let data = serde_json::to_string(&credentials)?;
            write!(file, "{data}")
        });

        // Check if the file was created successfully
        if let Err(e) = result {
            warn!("Cannot save credentials to cache: {}", e);
            exit(1);
        } else {
            info!("Credentials saved: {}", &args.path.display());
            exit(0);
        }
    }
}

mod tests {}
