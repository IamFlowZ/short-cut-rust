use std::env;
use std::fs::OpenOptions;
use std::io::Write;

use clap::Parser;

static HELP_TEXT: &str = r#"
         __               __                   __ 
   _____/ /_  ____  _____/ /_      _______  __/ /_
  / ___/ __ \/ __ \/ ___/ __/_____/ ___/ / / / __/
 (__  ) / / / /_/ / /  / /_/_____/ /__/ /_/ / /_  
/____/_/ /_/\____/_/   \__/      \___/\__,_/\__/  

Usage:
  short-cut [NAME]

  Quickly create an environment variable that equals the current working directory.

  Example:
    [user@machine:/home/user]$ short-cut test
      output: Please reload your bashrc file to access the new shortcut. (source ~/.bashrc)
    [user@machine:/home/user]$ source ~/.bashrc
    [user@machine:/home/user]$ echo $test
      output: /home/user"#;

#[derive(Parser, Debug)]
#[command(version, about = HELP_TEXT)]
struct Args {
    /// Name of the shortcut
    #[arg(short, long)]
    name: String,
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();

    let current_dir = match env::current_dir() {
        Ok(value) => value,
        _ => panic!("no cd"),
    };

    let home_dir = match std::env::var("HOME") {
        Ok(value) => value,
        _ => panic!("no shell set"),
    };

    let shell = match std::env::var("SHELL") {
        Ok(value) => value,
        _ => panic!("no shell set"),
    };

    println!("{:?}", shell);

    let shell_rc = if matches!(shell.as_str(), "/bin/zsh") {
        format!("{}/.zshrc", home_dir)
    } else {
        panic!("oh no");
    };

    println!("{shell_rc}");

    let file = OpenOptions::new().write(true).append(true).open(shell_rc);

    match writeln!(file?, "export {}={}", args.name, current_dir.display()) {
        Err(error) => panic!("failed to write file\n{}", error),
        _ => (),
    };
    Ok(())
}
