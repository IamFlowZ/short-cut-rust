use std::env;
use std::path::Path;
use std::fs::OpenOptions;
use std::io::Write;

use clap::Parser;
use regex::Regex;

const BANNER: &str = r#"
         __               __                   __ 
   _____/ /_  ____  _____/ /_      _______  __/ /_
  / ___/ __ \/ __ \/ ___/ __/_____/ ___/ / / / __/
 (__  ) / / / /_/ / /  / /_/_____/ /__/ /_/ / /_  
/____/_/ /_/\____/_/   \__/      \___/\__,_/\__/  
"#;

const USAGE: &str = "
Usage:
  short-cut [NAME]

  Quickly create an environment variable that equals the current working directory.

  Example:
    [user@machine:/home/user]$ short-cut test
      output: Please reload your bashrc file to access the new shortcut. (source ~/.bashrc)
    [user@machine:/home/user]$ source ~/.bashrc
    [user@machine:/home/user]$ echo $test
      output: /home/user";

#[derive(Parser, Debug)]
#[command(version, before_help = BANNER, long_about = USAGE)]
struct Args {
    /// Name of the shortcut
    name: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let current_dir = match env::current_dir()?.into_string() {
      Ok(value) => value,
      Err(path_buf) => panic!("Can't create shortcut for path: {}", path_buf.display())
    };

    let dir_regex = Regex::new(r"[A-Za-z_][A-Za-z0-9_]*").unwrap();

    if !dir_regex.is_match(&current_dir) {
      panic!("Directory failed check: [A-Za-z_][A-Za-z0-9_]*");
    }

    let home_dir = match env::var("HOME") {
        Ok(value) => value,
        _ => panic!("no home set"),
    };

    let shell = match env::var("SHELL") {
        Ok(value) => value,
        _ => panic!("no shell set"),
    };

    if cfg!(debug_assertions) {
      println!("{:?}", shell);
    }

    let rc = match Path::new(&shell).file_name().and_then(|s| s.to_str()) {
        Some("zsh") => ".zshrc",
        Some("bash") => ".bashrc",
        other => return Err(format!("unsupported shell: {other:?}").into()),
    };

    let shell_rc = Path::new(&home_dir).join(rc);   // PathBuf: /home/you/.zshrc
    if cfg!(debug_assertions) {
      println!("{}", shell_rc.display());
    }

    let mut file = OpenOptions::new().append(true).open(shell_rc)?;

    writeln!(file, "export {}={}", args.name, current_dir)?;

    println!("export {}={}", args.name, current_dir);

    Ok(())
}
