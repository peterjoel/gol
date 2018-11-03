use crate::{AppAction, GameState};
use editor::EditAction;

pub fn map_key_to_global_action(game_state: GameState, key: char) -> Option<AppAction> {
    match key {
        'q' => Some(AppAction::Quit),
        '\r' if game_state == GameState::Editing => Some(AppAction::EditDone),
        '\r' => Some(AppAction::TogglePause),
        'e' if game_state != GameState::Editing => Some(AppAction::EditMode),
        _ => None,
    }
}

pub fn map_key_to_edit_action(key: char) -> Option<EditAction> {
    match key {
        'c' => Some(EditAction::Clear),
        'i' => Some(EditAction::MoveCursorBy { x: 0, y: -1 }),
        'k' => Some(EditAction::MoveCursorBy { x: 0, y: 1 }),
        'j' => Some(EditAction::MoveCursorBy { x: -1, y: 0 }),
        'l' => Some(EditAction::MoveCursorBy { x: 1, y: 0 }),
        ' ' => Some(EditAction::ToggleCell),
        n if n.is_digit(10) => Some(EditAction::AddPreset {
            index: n.to_string().parse().unwrap(),
        }),
        _ => None,
    }
}
