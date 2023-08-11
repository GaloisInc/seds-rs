//! Code Formatting
use anyhow::Context;
use std::io::Write;
use std::process::{Command, Output, Stdio};

use quote::ToTokens;

/// run the rust formatter rustfmt on a token stream
pub fn rustfmt(tokens: impl ToTokens) -> Result<String, anyhow::Error> {
    let tokens = tokens.into_token_stream();

    // note that to string will fail
    let s = format!("{}", tokens);

    let mut child = Command::new("rustfmt")
        .arg("--edition")
        .arg("2021")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .context("Failed to spawn rustfmt")?;

    let mut stdin = child.stdin.take().unwrap();
    write!(stdin, "{}", s)?;
    stdin.flush()?;
    drop(stdin);

    let Output {
        status,
        stdout,
        stderr,
    } = child.wait_with_output()?;
    let stdout = String::from_utf8_lossy(&stdout);
    let stderr = String::from_utf8_lossy(&stderr);

    if !status.success() {
        eprintln!("---- Stdout ----");
        eprintln!("{}", stdout);
        eprintln!("---- Stderr ----");
        eprintln!("{}", stderr);
        let code = status.code();
        match code {
            Some(code) => anyhow::bail!("The `rustfmt` command failed with return code {}", code),
            None => anyhow::bail!("The `rustfmt` command failed"),
        }
    }

    Ok(stdout.into())
}
