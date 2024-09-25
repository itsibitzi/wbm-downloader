use async_recursion::async_recursion;
use reqwest::Response;
use serde::Deserialize;
use serde_json::Value;
use tokio::time::{sleep, Duration};

use crate::wbm_chrono::{YearMonth, YearMonthDayHourMinuteSecond};

pub struct Capture {
    /// Represents the day, hour, minutes and seconds of the datetime that
    /// this capture was taken. Max value is 31,245,959
    pub ymdhms: YearMonthDayHourMinuteSecond,
    /// The status of the page when the capture was taken
    pub status: u16,
}

pub struct CalendarCaptures {
    pub items: Option<Vec<Capture>>,
}

#[derive(Debug, Deserialize)]
struct RawCalendarCapture {
    // The API uses dynamic typing so we have to use a JSON Value
    pub items: Option<Vec<[Value; 3]>>,
}

pub async fn get_calendar_captures(
    url: &str,
    year_month: &YearMonth,
) -> anyhow::Result<CalendarCaptures> {
    let year_month_string = year_month.to_wbm_date_string();

    let api_url = format!(
        "https://web.archive.org/__wb/calendarcaptures/2?url={}&date={}",
        url, year_month_string
    );

    let captures = fetch_with_retries(&api_url, 3)
        .await?
        .json::<RawCalendarCapture>()
        .await?;

    let captures = CalendarCaptures {
        items: captures.items.map(|items| {
            items
                .into_iter()
                .flat_map(|[dhms, status, _]| {
                    let dhms = dhms
                        .as_u64()
                        .ok_or(anyhow::anyhow!("failed to convert dhms to u64"))?
                        .try_into()
                        .inspect_err(|e| eprintln!("Failed to parse dhms {}", e))?;

                    let status = status
                        .as_u64()
                        .ok_or(anyhow::anyhow!("failed to convert status to u64"))?
                        .try_into()
                        .inspect_err(|e| {
                            eprintln!("Failed to parse status '{}', expected u16: {}", status, e)
                        })?;

                    anyhow::Ok(Capture {
                        ymdhms: YearMonthDayHourMinuteSecond::from_ym_and_dhms_as_u32(
                            year_month, dhms,
                        ),
                        status,
                    })
                })
                .collect()
        }),
    };

    Ok(captures)
}

#[async_recursion]
async fn fetch_with_retries(page_url: &str, retries: i32) -> anyhow::Result<Response> {
    match reqwest::get(page_url).await {
        Ok(resp) => Ok(resp),
        Err(_) if retries > 0 => {
            sleep(Duration::from_secs_f32((1.0 / retries as f32) * 5.0)).await;
            fetch_with_retries(page_url, retries - 1).await
        }
        Err(e) => Err(e.into()),
    }
}

pub async fn get_capture(
    ymdhms: &YearMonthDayHourMinuteSecond,
    url: &str,
) -> anyhow::Result<String> {
    let page_url = format!(
        "https://web.archive.org/web/{}/{}",
        ymdhms.as_wbm_datetime_str(),
        url
    );

    Ok(fetch_with_retries(&page_url, 3).await?.text().await?)
}
