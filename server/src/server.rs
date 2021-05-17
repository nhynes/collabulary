use std::sync::Arc;

use anyhow::Result;
use futures_util::{stream::SplitStream, SinkExt as _, StreamExt, TryStreamExt as _};
use serde::Serialize;
use tracing::debug;
use warp::{
    filters::ws::Message,
    http::{header, Method},
    ws::WebSocket,
    Filter, Rejection, Reply,
};

use crate::game::*;

pub fn filters(
    game: Game<WebSocket>,
) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    let game = Arc::new(game);

    let cors = warp::cors()
        .allow_any_origin()
        .allow_methods(&[
            Method::DELETE,
            Method::GET,
            Method::HEAD,
            Method::OPTIONS,
            Method::PATCH,
            Method::POST,
            Method::PUT,
        ])
        .allow_headers(&[
            header::ACCEPT,
            header::AUTHORIZATION,
            header::CONTENT_LENGTH,
            header::CONTENT_TYPE,
            "x-requested-with".parse().unwrap(),
        ])
        .max_age(60 * 60);

    let ws = warp::get()
        .map(move || Arc::clone(&game))
        .and(warp::path::param())
        .and(warp::ws())
        .map(handle_ws_connect)
        .with(cors);
    let static_site = warp::get().and(warp::fs::dir("../app/public"));
    static_site.or(ws)
}

fn handle_ws_connect(game: Arc<Game<WebSocket>>, locale: Locale, ws: warp::ws::Ws) -> impl Reply {
    ws.on_upgrade(move |ws| async move {
        debug!(participant=?locale, "joining");
        let (sink, mut stream) = ws.split();

        if let Err((e, mut sink)) = game.set_participant(locale, sink).await {
            debug!(error=?e, "closing {:?} connection", locale);
            sink.send(Message::text(
                serde_json::to_string(&ErrorResponse {
                    error: e.to_string(),
                })
                .unwrap(),
            ))
            .await
            .ok();
            return;
        }

        debug!(participant=?locale, "joined");
        let result = handle_ws_connection(&mut stream, Arc::clone(&game), locale).await;
        let mut sink = match game.take_participant(locale).await {
            Some(sink) => sink,
            None => return,
        };
        debug!(participant=?locale, "left");

        if let Err(e) = result {
            debug!(error=?e, "closing {:?} connection", locale);
            sink.send(Message::text(
                serde_json::to_string(&ErrorResponse {
                    error: e.to_string(),
                })
                .unwrap(),
            ))
            .await
            .ok();
        }
    })
}

async fn handle_ws_connection(
    ws: &mut SplitStream<WebSocket>,
    game: Arc<Game<WebSocket>>,
    locale: Locale,
) -> Result<()> {
    // let messages = ws.and_then(|msg| async { msg.to_str().into() });
    while let Some(msg) = ws.try_next().await? {
        if msg.is_close() {
            break;
        }
        let req_json = match msg.to_str() {
            Ok(txt) if !txt.is_empty() => txt,
            _ => continue,
        };
        // if req_json.is_empty() {
        //     continue;
        // }
        let req: Request = serde_json::from_str(req_json)
            .map_err(|e| anyhow::Error::from(e).context(req_json.to_string()))?;
        debug!(locale=?locale, req=?req, "processing request");

        match req {
            Request::Score { score } => game.set_score(locale, score).await?,
            Request::Action { action } => match action {
                Action::AdvanceRound => game.set_ready(locale).await?,
            },
        }
    }

    Ok(())
}

#[derive(Serialize)]
pub struct ErrorResponse {
    error: String,
}
