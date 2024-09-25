use clap::Parser;
use cli::Cli;
use futures::{stream, StreamExt};

use wbm_api::get_calendar_captures;
use wbm_chrono::{PartialDateRoundingMode, YearMonthDay, YearMonthDayHourMinuteSecond};

use output_directory::OutputDirectory;

use crate::wbm_api::get_capture;

mod cli;
mod error;
mod output_directory;
mod wbm_api;
mod wbm_chrono;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    let output_dir = OutputDirectory::new(&cli.url, cli.output_directory)?;

    let from = YearMonthDay::from_str(&cli.from_date, PartialDateRoundingMode::Floor)?;
    let until = YearMonthDay::from_str(&cli.until_date, PartialDateRoundingMode::Ceiling)?;

    // WBM API lets you search on a prefix time. We'll go with months cos they usually
    // have a good number of captures (not too many, not too few)
    let months_to_check = from
        .to_year_month()
        .months_between_inclusive(&until.to_year_month())?;

    eprintln!("Fetching captures for {} months", months_to_check.len());

    let mut all_capture_ymdhms: Vec<YearMonthDayHourMinuteSecond> = vec![];

    // We only allow to the day precision on the inputs, but we check a whole month
    // for captures so we have to manually filter out captures before the day we request
    let from_min_filter_ymdhs =
        from.to_year_month_day_hour_minute_second(PartialDateRoundingMode::Floor);
    let until_max_filter_ymdhs =
        until.to_year_month_day_hour_minute_second(PartialDateRoundingMode::Ceiling);

    for year_month in months_to_check {
        eprintln!("Fetching captures for {}", year_month);

        let captures = get_calendar_captures(&cli.url, &year_month).await?;

        captures
            .items
            .unwrap_or_default()
            .into_iter()
            .filter(|item| item.status == 200)
            .map(|ok_row| ok_row.ymdhms)
            .filter(|ymdhms| *ymdhms > from_min_filter_ymdhs && *ymdhms < until_max_filter_ymdhs)
            .for_each(|capture_ymdhms| all_capture_ymdhms.push(capture_ymdhms));
    }

    eprintln!("Gathered {} captures", all_capture_ymdhms.len());

    stream::iter(all_capture_ymdhms)
        .map(|ymdhms| {
            let url = &cli.url;
            let output_dir = &output_dir;

            async move {
                if !output_dir.check_if_capture_exists(&ymdhms) {
                    eprintln!("Downloading {}", ymdhms.as_wbm_datetime_str());

                    match get_capture(&ymdhms, url).await {
                        Ok(html) => {
                            if let Err(e) = output_dir.save_html(&ymdhms, &html) {
                                eprintln!("Failed to save capture: {}", e);
                            }
                        }
                        Err(e) => {
                            eprintln!("Failed to get capture: {}", e);
                        }
                    }
                } else {
                    eprintln!("Already got {}", ymdhms.as_wbm_datetime_str());
                }
            }
        })
        .buffer_unordered(16)
        .collect::<()>()
        .await;

    Ok(())
}
