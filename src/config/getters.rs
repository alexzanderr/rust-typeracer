use super::{
    TyperacerConfig,
    UIConfig,
};

impl TyperacerConfig {
    #[inline(always)]
    pub fn ui_ref(&self) -> &UIConfig {
        &self.ui
    }

    #[inline(always)]
    pub fn sleep_ms_ref(&self) -> &u16 {
        &self.sleep_ms
    }

    #[inline(always)]
    pub fn sleep_ms(&self) -> u16 {
        self.sleep_ms
    }
}
