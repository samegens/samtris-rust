use crate::input::{InputEvent, Key};

/// Translate SDL2 events into platform-independent input events.
pub fn translate_sdl_event(event: sdl2::event::Event) -> Option<InputEvent> {
    match event {
        sdl2::event::Event::Quit { .. } => Some(InputEvent::Quit),
        sdl2::event::Event::KeyDown {
            keycode: Some(keycode),
            ..
        } => {
            let key = match keycode {
                sdl2::keyboard::Keycode::Up => Key::Up,
                sdl2::keyboard::Keycode::Down => Key::Down,
                sdl2::keyboard::Keycode::Left => Key::Left,
                sdl2::keyboard::Keycode::Right => Key::Right,
                sdl2::keyboard::Keycode::Space => Key::Space,
                sdl2::keyboard::Keycode::Return => Key::Enter,
                sdl2::keyboard::Keycode::Escape => Key::Escape,
                sdl2::keyboard::Keycode::X => Key::X,
                sdl2::keyboard::Keycode::Z => Key::Z,
                _ => return None,
            };
            Some(InputEvent::KeyPressed(key))
        }
        sdl2::event::Event::KeyUp {
            keycode: Some(keycode),
            ..
        } => {
            let key = match keycode {
                sdl2::keyboard::Keycode::Up => Key::Up,
                sdl2::keyboard::Keycode::Down => Key::Down,
                sdl2::keyboard::Keycode::Left => Key::Left,
                sdl2::keyboard::Keycode::Right => Key::Right,
                sdl2::keyboard::Keycode::Space => Key::Space,
                sdl2::keyboard::Keycode::Return => Key::Enter,
                sdl2::keyboard::Keycode::Escape => Key::Escape,
                sdl2::keyboard::Keycode::X => Key::X,
                sdl2::keyboard::Keycode::Z => Key::Z,
                _ => return None,
            };
            Some(InputEvent::KeyReleased(key))
        }
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(sdl2::keyboard::Keycode::Up, Key::Up)]
    #[case(sdl2::keyboard::Keycode::Down, Key::Down)]
    #[case(sdl2::keyboard::Keycode::Left, Key::Left)]
    #[case(sdl2::keyboard::Keycode::Right, Key::Right)]
    #[case(sdl2::keyboard::Keycode::Space, Key::Space)]
    #[case(sdl2::keyboard::Keycode::Return, Key::Enter)]
    #[case(sdl2::keyboard::Keycode::Escape, Key::Escape)]
    #[case(sdl2::keyboard::Keycode::X, Key::X)]
    #[case(sdl2::keyboard::Keycode::Z, Key::Z)]
    fn translate_key_down_returns_correct_key_pressed(
        #[case] sdl_keycode: sdl2::keyboard::Keycode,
        #[case] expected_key: Key,
    ) {
        // Arrange
        let sdl_event = sdl2::event::Event::KeyDown {
            timestamp: 0,
            window_id: 0,
            keycode: Some(sdl_keycode),
            scancode: None,
            keymod: sdl2::keyboard::Mod::empty(),
            repeat: false,
        };

        // Act
        let result = translate_sdl_event(sdl_event);

        // Assert
        assert_eq!(result, Some(InputEvent::KeyPressed(expected_key)));
    }

    #[test]
    fn translate_quit_event_returns_quit() {
        // Arrange
        let sdl_event = sdl2::event::Event::Quit { timestamp: 0 };

        // Act
        let result = translate_sdl_event(sdl_event);

        // Assert
        assert_eq!(result, Some(InputEvent::Quit));
    }

    #[test]
    fn translate_unknown_key_returns_none() {
        // Arrange
        let sdl_event = sdl2::event::Event::KeyDown {
            timestamp: 0,
            window_id: 0,
            keycode: Some(sdl2::keyboard::Keycode::A), // Not in our key mapping
            scancode: None,
            keymod: sdl2::keyboard::Mod::empty(),
            repeat: false,
        };

        // Act
        let result = translate_sdl_event(sdl_event);

        // Assert
        assert_eq!(result, None);
    }
}
