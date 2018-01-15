extern crate colored;

use std::env::home_dir;
use std::path::PathBuf;
use std::fs::File;
use std::io::prelude::*;
use std::fs::create_dir_all;
use colored::*;

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

    let home_dir = match home_dir() { Some(p) => p, None => PathBuf::from("."), };

    let mut bashrc = home_dir.clone();
    bashrc.push(".bashrc");
    let _ = match File::open(&bashrc) {
        Ok(mut f) => {
            let mut contents = String::new();
            f.read_to_string(&mut contents).expect("File read failed");
            let mut contents_saved = contents.clone();

            let should_write: bool = contents.lines().fold(true, |acc, next| acc && (next != "\n#manpath updated by cli-setup") );

            if !should_write {
                contents_saved.push_str("\n#manpath updated by cli-setup\nexport MANPATH=~/.local/share:$MANPATH");
                let _ = match File::create(&bashrc) {
                    Ok(mut file) => { file.write(contents_saved.as_bytes()).expect("File write failed") ; },
                    _ => eprintln!("{}: failed to open file at {}, not installing manual pages", "Warning".yellow(), &bashrc.display()),
                };
            }}
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
                Err(_) => eprintln!("{}: failed to open file, not installing manual pages", "Warning".yellow()),
            }
        },
        Err(_) => eprintln!("{}: failed to open file, not installing manual pages", "Warning".yellow()),
    }
}
