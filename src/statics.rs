use phf::{
    phf_map as static_hash_map,
    Map as StaticHashMap
};

pub const TERMINAL_CURSOR: &str = "▏";
pub const PROMPT_ARROW: &str = "❱";
pub const GREEN: &'static str = "\x1b[0;32m";
pub const RED: &'static str = "\x1b[0;31m";
pub const ENDC: &'static str = "\u{1b}[0m";
// pub const path: &'static [u8] =
// include_bytes!("static/audio/undertale-megalovania-soundtrack.mp3");
// pub const path: &'static [u8] =
// include_bytes!("static/audio/davai_hardbass.wav");
pub const SKELER_TELATIV_SONG: &'static [u8] =
    include_bytes!("../static/audio/skeler-telaviv.mp3");
pub const PLAY_CS16_SOUND: &'static [u8] =
    include_bytes!("../static/audio/play_cs16.wav");
pub const UNSTOPPABLE_CS16_SOUND: &'static [u8] =
    include_bytes!("../static/audio/unstoppable.wav");

pub const PROGRESS_BAR_LINE: &'static str = "━";
pub const PROGRESS_BAR: &'static str = "█";

#[cfg(feature = "embedded-music")]
mod embedded_music {
    pub const SKELER_TELATIV_SONG1: &'static [u8] =
        include_bytes!("../static/audio/skeler-telaviv.mp3");
    pub const PLAY_CS16_SOUND1: &'static [u8] =
        include_bytes!("../static/audio/play_cs16.wav");
}

#[cfg(feature = "embedded-music")]
pub use embedded_music::*;

/// capacity for the typed keys container
pub const TYPED_KEYS_CAPACITY: usize = 8;
pub static KEYS_REPR: StaticHashMap<&'static str, &'static str> = static_hash_map! {
    "enter" => "⏎",
    "tab" => "↹",
    "space" => "␣",
    "backspace" => "⌫",
    "up" => "↑",
    "left" => "←",
    "right" => "→",
    "down" => "↓",
    "shift" => "⇧",
    // "progress-bar-line" => PROGRESS_BAR_LINE,
    // "progress-bar" => PROGRESS_BAR,
};

// let cursor = "▏".red();
// let cursor = "│".red();
// let cursor = "|".red();
// let cursor = "｜".red();
// let cursor = "⏐".yellow();
// let cursor = "❘".yellow();
// let cursor = "|".yellow();
// let cursor = "𑗅".yellow();
