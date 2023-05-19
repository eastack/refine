use arboard::Clipboard;
#[cfg(target_os = "linux")]
use arboard::SetExtLinux;
use notify_rust::Notification;
use regex::{Captures, Regex};
use std::{borrow::Cow, env, error::Error, process};

// An argument that can be passed into the program to signal that it should daemonize itself. This
// can be anything as long as it is unlikely to be passed in by the user by mistake.
const DAEMONIZE_ARG: &str = "__internal_daemonize";

fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let mut clipboard = Clipboard::new().unwrap();

    let comment = clipboard.get_text()?;
    let comment = remove_hyphen(&comment);

    let comment = comment
        .lines()
        .map(|str| str.trim())
        .map(clean_comment_flag)
        .map(|str| str.trim())
        .filter(|str| !str.is_empty())
        .fold(String::new(), |cmt, line| cmt + line);

    #[cfg(target_os = "linux")]
    if env::args().nth(1).as_deref() == Some(DAEMONIZE_ARG) {
        clipboard.set().wait().text(comment)?;
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
        Clipboard::new()?.set_text(comment)?;
    }

    Notification::new().summary("Comment refined").show()?;

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

fn remove_hyphen(comment: &str) -> Cow<str> {
    let hyphen = Regex::new(r"([a-z])-\n([a-z])").unwrap();
    hyphen.replace_all(comment, |caps: &Captures| {
        format!("{}{}", &caps[1], &caps[2])
    })
}
