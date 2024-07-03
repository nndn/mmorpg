use bevy_ecs::prelude::*;
use client_sync::{current_pos, process_inputs, send_updates_to_clients, update_input_map};
use crossbeam_channel::{Receiver, Sender};
use domain::common::ID;
use map::{load_map, update_map};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::Error;
use std::time::{Duration, Instant};

use crate::domain;

use super::systems::*;

pub struct BevyGameEngine {
    pub world: World,
    pub init_schedule: Schedule,
    pub update_schedule: Schedule,
}

#[derive(Resource)]
pub struct SocketResource {}

#[derive(Debug, Serialize, Deserialize)]
pub struct InputMessage {
    pub user_id: ID,
    pub position: Position,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct Updates {
    pub position: Position,
}

#[derive(Component, Debug, Serialize, Deserialize, Clone, Copy)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

#[derive(Component, Debug)]
pub struct User {
    pub id: ID,
}

#[derive(Resource)]
pub struct InputQueue {
    pub receiver: Receiver<InputMessage>,
}

#[derive(Resource, Default)]
pub struct InputMap {
    pub inputs: HashMap<ID, Position>,
}

#[derive(Resource)]
pub struct ProcessTimer {
    pub last_processed: Instant,
    pub interval: Duration,
}

impl ProcessTimer {
    fn new(interval: Duration) -> Self {
        Self {
            last_processed: Instant::now(),
            interval,
        }
    }

    pub fn should_process(&mut self) -> bool {
        let now = Instant::now();
        if now.duration_since(self.last_processed) >= self.interval {
            self.last_processed = now;
            true
        } else {
            false
        }
    }
}

#[derive(Resource)]
pub struct UpdatesQueue {
    pub sender: Sender<Updates>,
}

impl BevyGameEngine {
    pub fn new(input_receiver: Receiver<InputMessage>, updates_sender: Sender<Updates>) -> Self {
        let mut world = World::new();

        // add resources to the world
        world.insert_resource(InputQueue {
            receiver: input_receiver,
        });
        world.insert_resource(UpdatesQueue {
            sender: updates_sender,
        });
        world.insert_resource(InputMap::default());
        world.insert_resource(ProcessTimer::new(Duration::from_secs(1)));

        // add components
        // Create entities with User and Position components
        world
            .spawn(User {
                id: ID("1".to_string()),
            })
            .insert(Position { x: 0, y: 0 });
        // world
        //     .spawn(User {
        //         id: ID("2".to_string()),
        //     })
        //     .insert(Position { x: 0, y: 0 });

        // init systems
        let mut init_schedule = Schedule::default();
        init_schedule.add_systems(load_map);

        // update systems
        let mut update_schedule = Schedule::default();
        update_schedule.add_systems(update_map);
        update_schedule.add_systems(update_input_map);
        update_schedule.add_systems(process_inputs);
        update_schedule.add_systems(send_updates_to_clients);

        BevyGameEngine {
            world: world,
            init_schedule: init_schedule,
            update_schedule: update_schedule,
        }
    }
}

pub trait GameEngine {
    fn start(&mut self) -> Option<Error>;
}

impl GameEngine for BevyGameEngine {
    fn start(&mut self) -> Option<Error> {
        println!("starting bevy engine!!");

        // Init world
        self.init_schedule.run(&mut self.world);

        // Main loop
        loop {
            self.update_schedule.run(&mut self.world);
            std::thread::sleep(std::time::Duration::from_millis(200));
        }
    }
}
