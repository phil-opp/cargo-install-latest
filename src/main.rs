use std::env;
use std::io::{stderr, Write};

use cargo_install_latest::*;

fn main() {
    match run() {
        Ok(()) => {}
        Err(err) => {
            writeln!(stderr(), "Error: {}", err).expect("failed to write to stderr");
        }
    };
}

fn run() -> Result<(), String> {
    use std::collections::HashMap;

    let mut args = env::args();
    assert!(args.next().is_some());
    assert_eq!(args.next(), Some("install-latest".into()));

    let mut required_crates = HashMap::new();
    for crate_name in args {
        let required_crate = Crate {
            name: crate_name.clone(),
            version: "*".into(),
            kind: CrateKind::CratesIo,
        };
        required_crates.insert(crate_name, required_crate);
    }

    let latest_versions = get_latest_versions(&required_crates)?;

    let installed_crates = installed_crates()?;

    let mut updates = Vec::new();
    for crate_name in required_crates.keys() {
        let installed_version = installed_crates.get(crate_name).map(|c| c.version.clone());
        let latest_version = latest_versions
            .get(crate_name)
            .ok_or(format!("Crate `{}` not found", crate_name))?;
        if installed_version.as_ref() == Some(latest_version) {
            println!("Up to date: {} {}", crate_name, latest_version);
        } else {
            updates.push((crate_name, installed_version, latest_version));
        }
    }

    if updates.len() > 1 {
        println!("\nThe following crates will be installed or updated:");
        for (crate_name, installed_version, latest_version) in &updates {
            if let Some(installed_version) = installed_version {
                println!(
                    "    Update {} from {} to {}",
                    crate_name, installed_version, latest_version
                );
            } else {
                println!("    Install {} {}", crate_name, latest_version);
            }
        }
    }

    for (crate_name, installed_version, latest_version) in &updates {
        if let Some(installed_version) = installed_version {
            println!(
                "\nUpdating {} from {} to {}",
                crate_name, installed_version, latest_version
            );
        } else {
            println!("\nInstalling {} {}", crate_name, latest_version);
        }
        if !install_update(&crate_name, latest_version)?.success() {
            return Err("Error: `cargo install` failed".into());
        }
    }

    println!("\nAll crates installed and up to date.");

    Ok(())
}
