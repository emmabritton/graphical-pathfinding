use ggez::event::KeyCode;

pub fn is_keycode_a_number(keycode: KeyCode) -> bool {
    match keycode {
        KeyCode::Key0 | KeyCode::Key1 |
        KeyCode::Key2 | KeyCode::Key3 |
        KeyCode::Key4 | KeyCode::Key5 |
        KeyCode::Key6 | KeyCode::Key7 |
        KeyCode::Key8 | KeyCode::Key9 => return true,
        _ => return false
    }
}

pub fn keycode_to_num(keycode: KeyCode) -> usize {
    match keycode {
        KeyCode::Key0 => return 0,
        KeyCode::Key1 => return 1,
        KeyCode::Key2 => return 2,
        KeyCode::Key3 => return 3,
        KeyCode::Key4 => return 4,
        KeyCode::Key5 => return 5,
        KeyCode::Key6 => return 6,
        KeyCode::Key7 => return 7,
        KeyCode::Key8 => return 8,
        KeyCode::Key9 => return 9,
        _ => panic!("Can not convert {:?} to number", keycode),
    }
}