use clap::{Arg, App};
use std::error::Error;
use std::process;

fn clap<'a>() -> App<'a, 'a> {
    App::new("url")
        .arg(Arg::with_name("title")
                .help("Title for the provided URL")
                .short("t")
                .long("title")
                .takes_value(false)
                .required(false))
        .arg(Arg::with_name("markdown")
                .help("Markdown output")
                .short("m")
                .long("markdown")
                .takes_value(false)
                .required(false))
        .arg(Arg::with_name("url")
                .required(true)
                .index(1))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let matches = clap().get_matches();

    if !matches.is_present("title") {
        println!("Nothing to do; exiting.");
        process::exit(1);
    }

    let url = matches.value_of("url").unwrap();
    let title = url::get_title(url).await?;

    if matches.is_present("markdown") {
        println!("[{}]({})", title, url);
    } else {
        println!("{}", title);
    }

    Ok(())
}
