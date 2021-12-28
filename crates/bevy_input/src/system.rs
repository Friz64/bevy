use crate::{
    keyboard::{KeyCode, KeyboardInput},
    ElementState,
};
use bevy_app::AppExit;
use bevy_ecs::prelude::{EventReader, EventWriter};

/// Sends the `AppExit` event whenever the "esc" key is pressed.
pub fn exit_on_esc_system(
    mut keyboard_input_events: EventReader<KeyboardInput>,
    mut app_exit_events: EventWriter<AppExit>,
) {
    for event in keyboard_input_events.iter() {
        if event.state == ElementState::Pressed && event.key_code == KeyCode::Escape {
            app_exit_events.send_default();
        }
    }
}
