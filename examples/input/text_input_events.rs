use bevy::{prelude::*, window::ReceivedTextInput};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_system(print_text_input_event_system)
        .run();
}

/// This system prints out all text input events as they come in
fn print_text_input_event_system(mut text_input_events: EventReader<ReceivedTextInput>) {
    for event in text_input_events.iter() {
        info!("{:?}: '{}'", event, event.text);
    }
}
