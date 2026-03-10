use clap::Parser;
use console::style;
use indicatif::{ProgressBar, ProgressStyle};
use reqwest::blocking::Client;
use std::io::Read;

#[derive(Parser)]
#[command(name = "Rget", version = "0.1.0", about = "wget clone written in Rust")]
struct Cli {
    /// URL to download
    url: String,
}

fn main() {
    let args = Cli::parse();
    if let Err(e) = download(&args.url, false) {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

fn create_progress_bar(quiet_mode: bool, msg: &str, length: Option<u64>) -> ProgressBar {
    let bar = match quiet_mode {
        true => ProgressBar::hidden(),
        false => match length {
            Some(len) => ProgressBar::new(len),
            None => ProgressBar::new_spinner(),
        },
    };

    bar.set_message(msg.to_string());

    match length.is_some() {
        true => bar.set_style(
            ProgressStyle::default_bar()
                .template(
                    "{msg} {spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] \
                     {bytes}/{total_bytes} eta: {eta}",
                )
                .unwrap()
                .progress_chars("=> "),
        ),
        false => bar.set_style(ProgressStyle::default_spinner()),
    };

    bar
}

fn parse_url(target: &str) -> Result<String, Box<dyn std::error::Error>> {
    if target.starts_with("http://") || target.starts_with("https://") {
        Ok(target.to_string())
    } else {
        Ok(format!("https://{}", target))
    }
}

fn save_to_file(buf: &[u8], fname: &str) -> Result<(), Box<dyn std::error::Error>> {
    std::fs::write(fname, buf)?;
    Ok(())
}

fn download(target: &str, quiet_mode: bool) -> Result<(), Box<dyn std::error::Error>> {
    let url = parse_url(target)?;
    let client = Client::new();
    let mut resp = client.get(&url).send()?;

    println!(
        "HTTP request sent... {}",
        style(format!("{}", resp.status())).green()
    );

    if resp.status().is_success() {
        let ct_len = resp.content_length();
        let ct_type = resp
            .headers()
            .get(reqwest::header::CONTENT_TYPE)
            .and_then(|v| v.to_str().ok())
            .unwrap_or("unknown")
            .to_string();

        match ct_len {
            Some(len) => println!(
                "Length: {} ({})",
                style(len).green(),
                style(format!("{} bytes", len)).red()
            ),
            None => println!("Length: {}", style("unknown").red()),
        }

        println!("Type: {}", style(&ct_type).green());

        let fname = target.split('/').next_back().unwrap_or("downloaded_file");
        println!("Saving to: {}", style(fname).green());

        let chunk_size = match ct_len {
            Some(x) => (x as usize) / 99,
            None => 1024,
        };

        let mut buf = Vec::new();
        let bar = create_progress_bar(quiet_mode, fname, ct_len);

        loop {
            let mut buffer = vec![0; chunk_size];
            let bcount = resp.read(&mut buffer[..])?;
            buffer.truncate(bcount);

            if !buffer.is_empty() {
                buf.extend_from_slice(&buffer);
                bar.inc(bcount as u64);
            } else {
                break;
            }
        }

        bar.finish();
        save_to_file(&buf, fname)?;
    }

    Ok(())
}
