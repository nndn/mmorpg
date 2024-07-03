use std::io::Error;

use axum::Router;
use crossbeam_channel::{Receiver, Sender};
use socketioxide::{
    extract::{Data, SocketRef},
    layer::SocketIoLayer,
    SocketIo,
};
use tokio::net::TcpListener;
use tower::ServiceBuilder;
use tower_http::{cors::CorsLayer, services::ServeDir};

use crate::{domain::common::ID, port::bevy::Position};

use super::bevy::{InputMessage, Updates};

#[derive(Clone)]
pub struct SocketIoClientInputReceiver {
    client_input_sender: Sender<InputMessage>,
    update_receiver: Receiver<Updates>,
    io: SocketIo,
    layer: SocketIoLayer,
}

impl SocketIoClientInputReceiver {
    pub fn new(sender: Sender<InputMessage>, update_receiver: Receiver<Updates>) -> Self {
        let (layer, io) = SocketIo::builder().build_layer();
        return SocketIoClientInputReceiver {
            client_input_sender: sender,
            update_receiver: update_receiver,
            io: io,
            layer: layer,
        };
    }

    pub fn build_router(self) -> Router {
        let (layer, io) = (self.layer, self.io);

        io.ns("/", |s: SocketRef| {
            s.on("movement", |_s: SocketRef, Data::<String>(direction)| {
                println!("direction: {}", direction);
            });

            s.on_disconnect(|_s: SocketRef| println!("disconnected"));
        });

        io.ns("/api", |s: SocketRef| {
            s.on("join-room", |s: SocketRef, Data::<String>(token)| {
                match s.join("test-room") {
                    Ok(_) => {
                        println!("joined test-room!! with token {}", token)
                    }
                    Err(err) => {
                        println!("error joining room: {}", err)
                    }
                };
            });

            s.on("movement", move |_s: SocketRef, Data::<Position>(pos)| {
                self.client_input_sender
                    .send(InputMessage {
                        user_id: ID("1".to_string()),
                        position: pos,
                    })
                    .expect("Failed to send");
            });

            s.on_disconnect(|_s: SocketRef| println!("disconnected server"));
        });

        return axum::Router::new()
            .nest_service("/", ServeDir::new("dist"))
            .layer(
                ServiceBuilder::new()
                    .layer(CorsLayer::permissive()) // Enable CORS policy
                    .layer(layer),
            );
    }

    pub async fn start(self: Self, listener: TcpListener) -> Result<(), Error> {
        let io = self.io.clone();
        let router = self.clone().build_router();
        tokio::spawn(async {
            let _ = axum::serve(listener, router).await;
        });
        loop {
            for message in self.update_receiver.try_iter() {
                io.of("/api")
                    .unwrap()
                    .within("test-room")
                    .emit("updates", message.clone())
                    .unwrap();
                println!("x: {}", message.position.x)
            }
            tokio::time::sleep(std::time::Duration::from_millis(1000)).await;
        }
    }
}
