#![feature(try_blocks)]

mod torrent;

use crate::torrent::Torrent;
use axum::extract::Multipart;
use axum::response::{Html, IntoResponse};
use axum::routing::{get, post};
use axum::{Json, Router};
use serde::Serialize;
use std::io::Write;
use std::net::SocketAddr;

#[derive(Serialize)]
#[serde(rename_all = "lowercase", tag = "type", content = "data")]
enum TorrentResponse {
    Success(Torrent),
    Fail(String),
}

async fn index() -> Html<&'static str> {
    Html(include_str!("../index.html"))
}

async fn torrent(mut body: Multipart) -> impl IntoResponse {
    let torrent: Option<Torrent> = try {
        loop {
            if let Some(field) = body.next_field().await.ok()? {
                let name = field.name()?;
                if name == "file" {
                    let data_raw = field.bytes().await.ok()?;
                    let torrent: Torrent = serde_bencode::from_bytes(data_raw.as_ref()).ok()?;

                    // save file on success
                    let ref torrent_name = torrent.info.name;
                    let name = torrent_name.replace("/", "／");
                    let mut file = std::fs::File::create(format!("/tmp/{name}.torrent")).ok()?;
                    file.write_all(data_raw.as_ref()).ok()?;
                    drop(file);

                    break Some(torrent);
                }
            } else {
                break None;
            }
        }?
    };
    match torrent {
        Some(torrent) => Json(TorrentResponse::Success(torrent)),
        None => Json(TorrentResponse::Fail("Failed to parse torrent".to_string())),
    }
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(index))
        .route("/torrent", post(torrent));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
