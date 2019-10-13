/// [rsget](https://github.com/otavio/rsget) is an excellent command written by Rust which
/// download file from a url with progress bars. But it's not a *crate*, I couldn't import in the
/// Cargo.toml. So, I copied most codes in the download_progressbar function and add some features
/// and comments.

use std::{
  fs,
  io::{self, copy, Read},
  path::Path,
};

use failure::{self, Error};
use indicatif::{ProgressBar, ProgressStyle};
use reqwest::{
  Client,
  header::{self, HeaderMap, HeaderValue},
};

/// DownloadProgress struct
struct DownloadProgress<R> {
  inner: R,
  progress_bar: ProgressBar,
}

impl<R> Read for DownloadProgress<R>
  where
    R: Read,
{
  fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
    self.inner.read(buf).map(|n| {
      self.progress_bar.inc(n as u64);
      n
    })
  }
}

/// download_progressbar is download function with progressbar used in the console.
/// The function get the total size of file with a HEAD request, then use a GET request
/// to download.
/// TODO 1. Build two HeaderMap in HEAD and GET request, it's should be improved. 2. Add export
/// TODO format support.
pub fn download_progressbar(url: &str, path: &str, token: &str, invalid_certs: bool) -> Result<u64, Error> {
  let client = Client::builder()
    .danger_accept_invalid_certs(invalid_certs)
    .build()
    .unwrap();

  let mut header = HeaderMap::new();
  if token.is_empty() == false {
    header.append("Authorization",
                  HeaderValue::from_str(
                    format!("Bearer {}", token).as_str()).unwrap());
  }

  let total_size = {
    let resp = client
      .head(url)
      .headers(header)
      .send().unwrap();
    if resp.status().is_success() {
      resp.headers()
        .get(header::CONTENT_LENGTH)
        .and_then(|ct_len| ct_len.to_str().ok())
        .and_then(|ct_len| ct_len.parse().ok())
        .unwrap_or(0)
    } else {
      return Err(failure::err_msg(format!(
        "Couldn't HEAD URL: {}. Error: {:?}", url, resp.status(),
      )).into());
    }
  };

  let mut header = HeaderMap::new();
  if token.is_empty() == false {
    header.append("Authorization",
                  HeaderValue::from_str(
                    format!("Bearer {}", token).as_str()).unwrap());
  }
  let mut request = client.get(url).headers(header);
  let pb = ProgressBar::new(total_size);
  pb.set_style(ProgressStyle::default_bar()
    .template("\t{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta})")
    .progress_chars("#>-"));

  let file = Path::new(path);

  if file.exists() == true {
    let size = file.metadata()?.len().checked_sub(1).unwrap_or(0);
    request = request.header(header::RANGE, format!("bytes={}-", size));
    pb.inc(size);
  }

  let mut source = DownloadProgress {
    progress_bar: pb,
    inner: request.send()?,
  };

  let mut dest = fs::OpenOptions::new()
    .create(true)
    .append(true)
    .open(&file)?;

  let result = copy(&mut source, &mut dest);

  match result {
    Ok(u) => { Ok(u) }
    Err(_) => {
      Err(failure::err_msg(format!(
        "Couldn't download URL: {}. Error: {:?}", url, source.inner.status(),
      )).into())
    }
  }
}