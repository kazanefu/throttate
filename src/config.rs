use bevy::prelude::*;

#[derive(Clone, Copy)]
pub struct PlayerSetting {}

impl Default for PlayerSetting {
    fn default() -> Self {
        Self {}
    }
}

#[derive(Clone, Copy)]
pub struct CourseSetting {}

impl Default for CourseSetting {
    fn default() -> Self {
        Self {}
    }
}

#[derive(Clone, Copy)]
pub struct InputSetting {
    pub respawn: KeyCode,
    pub throw: KeyCode,
    pub ll_spin: KeyCode,
    pub lr_spin: KeyCode,
    pub rl_spin: KeyCode,
    pub rr_spin: KeyCode,
    pub next: KeyCode,
    pub exit: KeyCode,
}

impl Default for InputSetting {
    fn default() -> Self {
        Self {
            respawn: KeyCode::KeyR,
            throw: KeyCode::Space,
            ll_spin: KeyCode::ArrowLeft,
            lr_spin: KeyCode::ArrowDown,
            rl_spin: KeyCode::ArrowUp,
            rr_spin: KeyCode::ArrowRight,
            next: KeyCode::Enter,
            exit: KeyCode::Escape,
        }
    }
}

#[derive(Resource, Default, Clone)]
pub struct GameConfig {
    pub player_setting: PlayerSetting,
    pub course_setting: CourseSetting,
    pub input_setting: InputSetting,
}
