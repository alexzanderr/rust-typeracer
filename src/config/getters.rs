use super::{
    TyperacerConfig,
    UIConfig,
};

impl TyperacerConfig {
    pub fn ui_ref(&self) -> &UIConfig {
        &self.ui
    }

    pub fn fps_ref(&self) -> &u8 {
        &self.fps
    }
}
