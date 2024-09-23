use clap::Parser;
use futures::{stream, StreamExt};
use serde_json::Value;

use wbm_api::get_calendar_captures;
use wbm_date::{get_months_between_dates, to_wbm_date, RangePart};

use output_directory::OutputDirectory;

use crate::wbm_api::get_capture;

mod error;
mod output_directory;
mod wbm_api;
mod wbm_date;

#[derive(Debug, Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
struct Cli {
    /// The URL prefix to query WBM
    url: String,
    /// Match captures after this date
    from_date: String,
    /// Match captures before this date
    until_date: String,
    /// Set the output directory for the captures
    #[clap(short, long, default_value = "./")]
    output_directory: String,
    /// Verbose output logs if files are already downloaded
    #[clap(short, long)]
    verbose: bool,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    let output_dir = OutputDirectory::new(&cli.url, cli.output_directory)?;

    let wbm_from = to_wbm_date(RangePart::From, cli.from_date)?;
    let wbm_until = to_wbm_date(RangePart::Until, cli.until_date)?;

    let mut all_capture_ymdhms: Vec<String> = vec![];

    let mut months_to_check: Vec<String> = get_months_between_dates(&wbm_from, &wbm_until)
        .into_iter()
        .collect();
    months_to_check.sort();

    println!("Fetching captures for {} months", months_to_check.len());

    for year_month in months_to_check {
        println!("Fetching captures for {}", year_month);

        let captures = get_calendar_captures(&cli.url, &year_month)
            .await
            .expect("Failed to get captures");

        captures
            .items
            .unwrap_or_default()
            .iter()
            .filter(|capture_row| {
                matches!(capture_row, [_, Value::Number(num), _] if num.as_i64().unwrap() == 200)
            })
            .map(|ok_row| {
                let dhms = ok_row[0].to_string();

                // The WBM returns integers so leading zeros are cut off
                if dhms.len() == 7 {
                    format!("{}0{}", &year_month, dhms)
                } else {
                    format!("{}{}", &year_month, dhms)
                }
            })
            .for_each(|capture_ymdhms| all_capture_ymdhms.push(capture_ymdhms));
    }

    println!("Gathered {} captures", all_capture_ymdhms.len());

    stream::iter(all_capture_ymdhms)
        .map(|ymdhms| {
            let url = &cli.url;
            let output_dir = &output_dir;

            async move {
                if !output_dir.check_if_capture_exists(&ymdhms) {
                    println!("Downloading {}", ymdhms);
                    let html = get_capture(&ymdhms, url).await?;
                    output_dir.save_html(&ymdhms, &html)
                } else {
                    if cli.verbose {
                        println!("Already got {}", ymdhms);
                    }
                    Ok(())
                }
            }
        })
        .buffer_unordered(16)
        .for_each(|r| async {
            match r {
                Err(e) => eprintln!("Failed to save capture: {}", e),
                _ => {}
            }
        })
        .await;

    Ok(())
}
