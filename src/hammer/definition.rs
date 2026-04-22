use bevy::prelude::*;
use bevy_rapier2d::prelude::*;


pub const HAMMER_HANDLE_OFFSET: Vec2 = Vec2 { x: -40.0, y: 0.0 };
pub const HAMMER_ACTION_KEY_CODE: KeyCode = KeyCode::Space;
pub const CHANGE_DIRECTION_KEY_CODE_LL: KeyCode = KeyCode::ArrowLeft;
pub const CHANGE_DIRECTION_KEY_CODE_RR: KeyCode = KeyCode::ArrowRight;
pub const CHANGE_DIRECTION_KEY_CODE_LR: KeyCode = KeyCode::ArrowDown;
pub const CHANGE_DIRECTION_KEY_CODE_RL: KeyCode = KeyCode::ArrowUp;
pub const HAMMER_SPIN: (f32, f32) = (100.0, 0.1);