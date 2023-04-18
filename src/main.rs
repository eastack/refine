use arboard::Clipboard;
#[cfg(target_os = "linux")]
use arboard::SetExtLinux;
use std::{env, error::Error, process};

// An argument that can be passed into the program to signal that it should daemonize itself. This
// can be anything as long as it is unlikely to be passed in by the user by mistake.
const DAEMONIZE_ARG: &str = "__internal_daemonize";

fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let mut clipboard = Clipboard::new().unwrap();
    let content = clipboard.get_text()?;
    let content = content
        .lines()
        .map(|str| str.trim())
        .map(clean_comment_flag)
        .map(|str| str.trim())
        .filter(|str| !str.is_empty())
        .collect::<Vec<_>>()
        .join(" ");

    #[cfg(target_os = "linux")]
    if env::args().nth(1).as_deref() == Some(DAEMONIZE_ARG) {
        clipboard.set().wait().text(content)?;
        return Ok(());
    }

    if cfg!(target_os = "linux") {
        process::Command::new(env::current_exe()?)
            .arg(DAEMONIZE_ARG)
            .stdin(process::Stdio::null())
            .stdout(process::Stdio::null())
            .stderr(process::Stdio::null())
            .current_dir("/")
            .spawn()?;
    } else {
        Clipboard::new()?.set_text("Hello, world!")?;
    }

    Ok(())
}

fn clean_comment_flag(str: &str) -> &str {
    if str.starts_with("///") {
        return &str[3..];
    }
    if str.starts_with("//")
        || str.starts_with("/*")
        || str.starts_with("* ")
        || str.starts_with("# ")
        || str.starts_with("--")
    {
        return &str[2..];
    }

    str
}