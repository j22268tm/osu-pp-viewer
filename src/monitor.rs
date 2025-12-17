// use futures::{StreamExt, SinkExt};
use tokio::net::TcpStream;
use tokio_tungstenite::{connect_async, MaybeTlsStream, WebSocketStream};
use serde::Deserialize;
use std::error::Error;

#[derive(Debug, Deserialize)]
pub struct GosuMemoryState {
    pub menu: Menu,
    pub gameplay: Gameplay,
}

#[derive(Debug, Deserialize)]
pub struct Menu {
    pub bm: Beatmap,
    pub mods: Mods,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct Beatmap {
    pub id: i32,
    pub set: i32,
    pub path: BeatmapPath,
}

#[derive(Debug, Deserialize)]
pub struct BeatmapPath {
    pub full: String,
}

#[derive(Debug, Deserialize)]
pub struct Mods {
    pub str: String,
}

#[derive(Debug, Deserialize)]
pub struct Gameplay {
    pub combo: Combo,
    pub hits: Hits,
    pub accuracy: f64,
}

#[derive(Debug, Deserialize)]
pub struct Combo {
    pub current: i32,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct Hits {
    #[serde(rename = "0")]
    pub n0: i32,
    #[serde(rename = "50")]
    pub n50: i32,
    #[serde(rename = "100")]
    pub n100: i32,
    #[serde(rename = "300")]
    pub n300: i32,
    pub geki: i32,
    pub katu: i32,
}

pub async fn connect(url: &str) -> Result<WebSocketStream<MaybeTlsStream<TcpStream>>, Box<dyn Error>> {
    let (ws_stream, _) = connect_async(url).await?;
    println!("Connected to gosumemory at {}", url);
    Ok(ws_stream)
}
