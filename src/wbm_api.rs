use async_recursion::async_recursion;
use serde::Deserialize;
use serde_json::Value;
use tokio::time::{sleep, Duration};
use urlencoding::encode;

#[derive(Deserialize)]
pub struct CalendarCaptures {
    // They annoyingly return non-specific type so we have to use a JSON Value
    pub items: Option<Vec<[Value; 3]>>,
}

pub async fn get_calendar_captures(
    url: &str,
    year_month: &str,
) -> anyhow::Result<CalendarCaptures> {
    let encoded_url = encode(url);

    let api_url = format!(
        "https://web.archive.org/__wb/calendarcaptures/2?url={}&date={}",
        encoded_url, year_month
    );

    Ok(reqwest::get(api_url)
        .await?
        .json::<CalendarCaptures>()
        .await?)
}

#[async_recursion]
async fn fetch_with_retries(page_url: &str, retries: i32) -> anyhow::Result<String> {
    match reqwest::get(page_url).await {
        Ok(resp) => Ok(resp.text().await?),
        Err(_) if retries > 0 => {
            sleep(Duration::from_secs_f32((1.0 / retries as f32) * 5.0)).await;
            fetch_with_retries(page_url, retries - 1).await
        },
        Err(e) => Err(e.into())
    }
}

pub async fn get_capture(ymdhms: &str, url: &str) -> anyhow::Result<String> {
    let page_url = format!("https://web.archive.org/web/{}/{}", ymdhms, url);

    fetch_with_retries(&page_url, 3).await
}
