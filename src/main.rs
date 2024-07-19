use bevy::prelude::Schedule;
fn main() {
    let mut schedule = Schedule::default();
    schedule.add_systems(noop);
}

fn noop() {}

