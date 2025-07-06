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
                sdl2::keyboard::Keycode::Backspace => Key::Backspace,
                sdl2::keyboard::Keycode::A => Key::Alphanumeric('A'),
                sdl2::keyboard::Keycode::B => Key::Alphanumeric('B'),
                sdl2::keyboard::Keycode::C => Key::Alphanumeric('C'),
                sdl2::keyboard::Keycode::D => Key::Alphanumeric('D'),
                sdl2::keyboard::Keycode::E => Key::Alphanumeric('E'),
                sdl2::keyboard::Keycode::F => Key::Alphanumeric('F'),
                sdl2::keyboard::Keycode::G => Key::Alphanumeric('G'),
                sdl2::keyboard::Keycode::H => Key::Alphanumeric('H'),
                sdl2::keyboard::Keycode::I => Key::Alphanumeric('I'),
                sdl2::keyboard::Keycode::J => Key::Alphanumeric('J'),
                sdl2::keyboard::Keycode::K => Key::Alphanumeric('K'),
                sdl2::keyboard::Keycode::L => Key::Alphanumeric('L'),
                sdl2::keyboard::Keycode::M => Key::Alphanumeric('M'),
                sdl2::keyboard::Keycode::N => Key::Alphanumeric('N'),
                sdl2::keyboard::Keycode::O => Key::Alphanumeric('O'),
                sdl2::keyboard::Keycode::P => Key::Alphanumeric('P'),
                sdl2::keyboard::Keycode::Q => Key::Alphanumeric('Q'),
                sdl2::keyboard::Keycode::R => Key::Alphanumeric('R'),
                sdl2::keyboard::Keycode::S => Key::Alphanumeric('S'),
                sdl2::keyboard::Keycode::T => Key::Alphanumeric('T'),
                sdl2::keyboard::Keycode::U => Key::Alphanumeric('U'),
                sdl2::keyboard::Keycode::V => Key::Alphanumeric('V'),
                sdl2::keyboard::Keycode::W => Key::Alphanumeric('W'),
                sdl2::keyboard::Keycode::X => Key::Alphanumeric('X'),
                sdl2::keyboard::Keycode::Y => Key::Alphanumeric('Y'),
                sdl2::keyboard::Keycode::Z => Key::Alphanumeric('Z'),
                sdl2::keyboard::Keycode::Num0 => Key::Alphanumeric('0'),
                sdl2::keyboard::Keycode::Num1 => Key::Alphanumeric('1'),
                sdl2::keyboard::Keycode::Num2 => Key::Alphanumeric('2'),
                sdl2::keyboard::Keycode::Num3 => Key::Alphanumeric('3'),
                sdl2::keyboard::Keycode::Num4 => Key::Alphanumeric('4'),
                sdl2::keyboard::Keycode::Num5 => Key::Alphanumeric('5'),
                sdl2::keyboard::Keycode::Num6 => Key::Alphanumeric('6'),
                sdl2::keyboard::Keycode::Num7 => Key::Alphanumeric('7'),
                sdl2::keyboard::Keycode::Num8 => Key::Alphanumeric('8'),
                sdl2::keyboard::Keycode::Num9 => Key::Alphanumeric('9'),
                _ => return None,
            };
            Some(InputEvent::KeyPressed(key))
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
    #[case(sdl2::keyboard::Keycode::X, Key::Alphanumeric('X'))]
    #[case(sdl2::keyboard::Keycode::Z, Key::Alphanumeric('Z'))]
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
    fn translate_unknown_key_down_returns_none() {
        // Arrange
        let sdl_event = sdl2::event::Event::KeyDown {
            timestamp: 0,
            window_id: 0,
            keycode: Some(sdl2::keyboard::Keycode::BACKQUOTE), // Not in our key mapping
            scancode: None,
            keymod: sdl2::keyboard::Mod::empty(),
            repeat: false,
        };

        // Act
        let result = translate_sdl_event(sdl_event);

        // Assert
        assert_eq!(result, None);
    }

    #[test]
    fn translate_unknown_key_up_returns_none() {
        // Arrange
        let sdl_event = sdl2::event::Event::KeyUp {
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

    #[test]
    fn translate_unknown_sdl_event_returns_none() {
        // Arrange
        let sdl_event = sdl2::event::Event::MouseButtonDown {
            timestamp: 0,
            window_id: 0,
            which: 0,
            mouse_btn: sdl2::mouse::MouseButton::Left,
            clicks: 1,
            x: 0,
            y: 0,
        };

        // Act
        let result = translate_sdl_event(sdl_event);

        // Assert
        assert_eq!(result, None);
    }
}
