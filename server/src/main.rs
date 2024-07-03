use crossbeam_channel::unbounded;
use port::{
    bevy::{BevyGameEngine, GameEngine},
    socketio::SocketIoClientInputReceiver,
};

pub mod domain;
pub mod port;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!(
        "Starting game server [socket IO] on {}",
        listener.local_addr().unwrap()
    );
    let (client_input_sender, client_input_receiver) = unbounded();
    let (update_sender, update_receiver) = unbounded();

    tokio::spawn(async move {
        SocketIoClientInputReceiver::new(client_input_sender, update_receiver)
            .start(listener)
            .await
            .unwrap()
    });

    BevyGameEngine::new(client_input_receiver, update_sender)
        .start()
        .unwrap();
    Ok(())
}
