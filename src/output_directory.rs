use std::path::{Path, PathBuf};

use urlencoding::encode;

use crate::{error::Error, wbm_chrono::YearMonthDayHourMinuteSecond};

pub struct OutputDirectory {
    path: PathBuf,
}

impl OutputDirectory {
    pub fn new(url: &str, base_path: impl AsRef<Path>) -> anyhow::Result<OutputDirectory> {
        let mut path: PathBuf = base_path.as_ref().into();

        if path.exists() && path.is_dir() {
            let encoded_url = encode(url);
            path.push(&*encoded_url);

            if !path.exists() {
                eprintln!("Creating path {}", path.display());
                std::fs::create_dir(path.clone())?;
            }

            Ok(OutputDirectory { path })
        } else {
            Err(Error::OutputDirectoryDoesNotExist.into())
        }
    }

    pub fn check_if_capture_exists(&self, ymdhms: &YearMonthDayHourMinuteSecond) -> bool {
        let mut path = self.path.clone();
        path.push(&ymdhms.as_wbm_datetime_str());
        path.set_extension("html");

        path.exists()
    }

    pub fn save_html(
        &self,
        ymdhms: &YearMonthDayHourMinuteSecond,
        html: &str,
    ) -> anyhow::Result<()> {
        let mut path = self.path.clone();
        path.push(&ymdhms.as_wbm_datetime_str());
        path.set_extension("html");

        std::fs::write(path, html)?;

        Ok(())
    }
}
