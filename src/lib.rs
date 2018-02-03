extern crate colored;

use colored::*;
use std::env::home_dir;
use std::fs::File;
use std::fs::create_dir_all;
use std::io::prelude::*;
use std::path::PathBuf;

/// Set up rules for [fuck](https://pypi.python.org/pypi/thefuck/) that work with `clap-rs`.
///
/// ```
/// use cli_setup::*;
///
/// setup_thefuck();
/// ```
pub fn setup_thefuck() {
    pub const THEFUCK_STRING: &'static str = include_str!("../py/clap-rs.py");
    let home_dir = match home_dir() {
        Some(p) => p,
        None => PathBuf::from("."),
    };

    let mut config_path = home_dir.clone();
    config_path.push(".config");
    config_path.push("thefuck");
    config_path.push("rules");
    let cfg_path_exists = config_path.exists() && config_path.is_dir();
    if cfg_path_exists {
        config_path.push("clap-rs");
        config_path.set_extension("py");
        let _ = match File::create(&config_path) {
            Ok(mut f) => {
                let mut thefuck_string = THEFUCK_STRING.to_string();
                let _ = match f.write(thefuck_string.as_bytes()) {
                    Ok(_) => (),
                    _ => eprintln!("{}: file write failed", "Warning".yellow()),
                };
            }
            _ => eprintln!(
                "{}: failed to open file at {}",
                "Warning".yellow(),
                &config_path.display()
            ),
        };
    }
}

/// Set up given the manpage contents and an executable name. This function is intended to be
/// called inside the project `build.rs`.
///
/// ```
/// use cli_setup::*;
///
/// pub const MANPAGE: &'static str = include_str!("man/executable.1");
/// setup_manpages(MANPAGE, "executable");
/// ```
pub fn setup_manpages(man: &str, exe_name: &str) {
    let home_dir = match home_dir() {
        Some(p) => p,
        None => PathBuf::from("."),
    };

    let mut bashrc = home_dir.clone();
    bashrc.push(".bashrc");
    let _ = match File::open(&bashrc) {
        Ok(mut f) => {
            let mut contents = String::new();
            match f.read_to_string(&mut contents) {
                Ok(_) => (),
                Err(_) => eprintln!(
                    "{}: failed to open file, not installing manual pages",
                    "Warning".yellow()
                ),
            }
            let mut contents_saved = contents.clone();

            let should_write: bool = contents.lines().fold(true, |acc, next| {
                acc && (next != "\n#manpath updated by cli-setup")
            });

            if !should_write {
                contents_saved.push_str(
                    "\n#manpath updated by cli-setup\nexport MANPATH=~/.local/share:$MANPATH",
                );
                match File::create(&bashrc) {
                    Ok(mut file) => match file.write(contents_saved.as_bytes()) {
                        Ok(_) => (),
                        Err(_) => eprintln!(
                            "{}: failed to open file, not installing manual pages",
                            "Warning".yellow()
                        ),
                    },
                    _ => eprintln!(
                        "{}: failed to open file at {}, not installing manual pages",
                        "Warning".yellow(),
                        &bashrc.display()
                    ),
                };
            };
        }
        _ => (),
    };

    let mut man_dir = home_dir.clone();
    man_dir.push(".local");
    man_dir.push("share");
    man_dir.push("man");
    man_dir.push("man1");
    let mut man_path = man_dir.clone();

    let _ = create_dir_all(man_dir);

    man_path.push(exe_name);
    man_path.set_extension("1");

    let pre_f = File::create(man_path);
    match pre_f {
        Ok(mut f) => {
            let res = f.write(man.as_bytes());
            match res {
                Ok(_) => (),
                Err(_) => eprintln!(
                    "{}: failed to open file, not installing manual pages",
                    "Warning".yellow()
                ),
            }
        }
        Err(_) => eprintln!(
            "{}: failed to open file, not installing manual pages",
            "Warning".yellow()
        ),
    }
}
