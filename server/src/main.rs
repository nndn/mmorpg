use axum::{
    extract::{
        ws::{Message, WebSocket},
        WebSocketUpgrade,
    },
    response::Response,
    routing::get,
    Router,
};
use futures::{Sink, SinkExt, Stream, StreamExt};

#[tokio::main]
async fn main() {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("Starting game server on {}", listener.local_addr().unwrap());
    axum::serve(listener, app()).await.unwrap();
}

fn app() -> Router {
    Router::new()
        .route("/integration-testable", get(integration_testable_handler))
        .route("/unit-testable", get(unit_testable_handler))
}

// A WebSocket handler that echos any message it receives.
//
// This one we'll be integration testing so it can be written in the regular way.
async fn integration_testable_handler(ws: WebSocketUpgrade) -> Response {
    ws.on_upgrade(integration_testable_handle_socket)
}

async fn integration_testable_handle_socket(mut socket: WebSocket) {
    while let Some(Ok(msg)) = socket.recv().await {
        if let Message::Text(msg) = msg {
            if socket
                .send(Message::Text(format!("You said: {msg}")))
                .await
                .is_err()
            {
                break;
            }
        }
    }
}

// The unit testable version requires some changes.
//
// By splitting the socket into an `impl Sink` and `impl Stream` we can test without providing a
// real socket and instead using channels, which also implement `Sink` and `Stream`.
async fn unit_testable_handler(ws: WebSocketUpgrade) -> Response {
    ws.on_upgrade(|socket| {
        let (write, read) = socket.split();
        unit_testable_handle_socket(write, read)
    })
}

// The implementation is largely the same as `integration_testable_handle_socket` expect we call
// methods from `SinkExt` and `StreamExt`.
async fn unit_testable_handle_socket<W, R>(mut write: W, mut read: R)
where
    W: Sink<Message> + Unpin,
    R: Stream<Item = Result<Message, axum::Error>> + Unpin,
{
    while let Some(Ok(msg)) = read.next().await {
        if let Message::Text(msg) = msg {
            if write
                .send(Message::Text(format!("You said: {msg}")))
                .await
                .is_err()
            {
                break;
            }
        }
    }
}
