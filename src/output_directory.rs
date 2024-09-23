use std::path::{Path, PathBuf};

use urlencoding::encode;

use crate::error::Error;

pub struct OutputDirectory {
    path: PathBuf,
}

impl OutputDirectory {
    pub fn new<P: AsRef<Path>>(url: &str, base_path: P) -> anyhow::Result<OutputDirectory> {
        let mut path: PathBuf = base_path.as_ref().into();

        if path.exists() && path.is_dir() {
            let encoded_url = encode(url);
            path.push(&*encoded_url);

            if !path.exists() {
                std::fs::create_dir(path.clone())?;
            }

            Ok(OutputDirectory {
                path,
            })
        } else {
            Err(Error::OutputDirectoryDoesNotExist.into())
        }
    }

    pub fn check_if_capture_exists(&self, ymdhms: &str) -> bool {
        let mut path = self.path.clone();
        path.push(ymdhms);
        path.set_extension("html");

        path.exists()
    }

    pub fn save_html(&self, ymdhms: &str, html: &str) -> anyhow::Result<()> {
        let mut path = self.path.clone();
        path.push(ymdhms);
        path.set_extension("html");

        std::fs::write(path, html)?;

        Ok(())
    }
}
