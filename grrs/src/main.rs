#![allow(unused)]

use clap::Parser;
/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    #[clap(parse(from_os_str))]
    path: std::path::PathBuf,
}

fn do_hard_work() {
    let ten_millis = std::time::Duration::from_millis(20);
    let now = std::time::Instant::now();
    std::thread::sleep(ten_millis);
}

fn random_progress_bar() {
    log::warn!("Gonna waste a lot of your time with this progress bar");
    let pb = indicatif::ProgressBar::new(100);
    for i in 0..100 {
        do_hard_work();
        pb.inc(1);
    }
    pb.finish_with_message("done");
}

fn main() {
    env_logger::init();
    let args = Cli::parse();
    let content = std::fs::read_to_string(&args.path).expect("could not read file");
    // Tips to handle panic: https://rust-cli.github.io/book/tutorial/errors.html#no-need-to-panic

    grrs::find_matches(&content, &args.pattern, &mut std::io::stdout());

    random_progress_bar();
}


