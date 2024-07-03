use crate::port::bevy::{
    InputMap, InputQueue, Position, ProcessTimer, Updates, UpdatesQueue, User,
};

use bevy_ecs::system::{Query, Res, ResMut};

pub fn update_input_map(input_queue: Res<InputQueue>, mut input_map: ResMut<InputMap>) {
    for input in input_queue.receiver.try_iter() {
        input_map.inputs.insert(
            input.user_id,
            Position {
                x: input.position.x,
                y: input.position.y,
            },
        );
    }
}

pub fn process_inputs(
    mut query: Query<(&User, &mut Position)>,
    mut input_map: ResMut<InputMap>,
    mut timer: ResMut<ProcessTimer>,
) {
    if timer.should_process() {
        for (user, mut position) in query.iter_mut() {
            if let Some(new_position) = input_map.inputs.get(&user.id) {
                if position.x != new_position.x || position.y != new_position.y {
                    println!("pos: {} {}", new_position.x, new_position.y);
                }
                position.x = new_position.x;
                position.y = new_position.y;
            }
        }
        input_map.inputs.clear();
    }
}

pub fn current_pos(query: Query<(&User, &mut Position)>) {
    for (_user, position) in query.iter() {
        println!("pos: {} {}", position.x, position.y);
    }
}

pub fn send_updates_to_clients(
    output_updates: Res<UpdatesQueue>,
    query: Query<(&User, &mut Position)>,
) {
    for (user, position) in query.iter() {
        match output_updates.sender.send(Updates {
            position: position.clone(),
        }) {
            Ok(_) => {
                println!("position of x is: {}", position.x)
            }
            Err(err) => {
                println!("error sending updates: {}", err)
            }
        }
    }
}
