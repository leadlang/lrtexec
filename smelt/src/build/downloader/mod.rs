use std::{sync::LazyLock, time::Duration};

use indicatif::{ProgressBar, ProgressStyle};
use reqwest::{Client, ClientBuilder};
use serde::{Deserialize, Serialize};
use tokio::{fs::File, io::AsyncWriteExt};

use crate::config::{Dependency, Source};

#[derive(Debug, Serialize, Deserialize)]
pub struct Releases {
  pub tag_name: String,
  pub assets: Vec<Asset>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Asset {
  pub browser_download_url: String,
  pub digest: String,
  pub name: String,
}

pub static CLIENT: LazyLock<Client> = LazyLock::new(|| {
  let builder = ClientBuilder::new().user_agent("AHQ Softwares");

  let builder = builder.default_headers({
    let mut map = reqwest::header::HeaderMap::new();

    if let Some(x) = option_env!("GITHUB_TOKEN") {
      let token = format!("Bearer {}", x);
      map.insert(
        "Authorization",
        reqwest::header::HeaderValue::from_name(
          reqwest::header::HeaderName::from_bytes(token.as_bytes()).unwrap(),
        ),
      );
    }

    map
  });

  builder.build().unwrap()
});

pub async fn download(name: &str, dep: &Dependency, prog: ProgressBar) {
  match dep.source {
    Source::GitHub => {
      prog.set_message("Resolving...");
      prog.enable_steady_tick(Duration::from_millis(10));

      let file = format!("lrt_pkg_{}.zip", env!("TARGET"));
      let download_url = format!(
        "https://github.com/{}/releases/download/{}/{}",
        name, dep.tag_name, file
      );

      let release = CLIENT.get(download_url).send().await.unwrap();

      let Ok(mut release) = release.error_for_status() else {
        panic!("Failed to download {}", dep.tag_name);
      };

      prog.disable_steady_tick();

      prog.set_style(ProgressStyle::default_bar());
      prog.set_length(release.content_length().unwrap_or(3000));

      let pkg_name = format!("./lrt_cache/github_{}_{}.zip", name, dep.tag_name);
      let mut file = File::create(&pkg_name).await.unwrap();

      while let Some(x) = release.chunk().await.unwrap() {
        prog.inc(x.len() as u64);
        file.write_all(&x).await.unwrap();
      }

      _ = file.flush().await;

      drop(file);
    }
  }
}
