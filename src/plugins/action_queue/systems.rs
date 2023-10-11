use bevy::prelude::*;

use super::components::ActionQueue;

pub fn update(
    time: Res<Time>,
    mut query: Query<&mut ActionQueue>,
) {
    for mut action_queue in query.iter_mut() {
        if !action_queue.ready {
            action_queue.timer.tick(time.delta());
        }

        if action_queue.timer.finished() {
            action_queue.timer.reset();
            action_queue.ready = true;
        }
    }
}