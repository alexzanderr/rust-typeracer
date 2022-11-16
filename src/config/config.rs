use std::path::Path;
use std::fs;

use lazy_static::lazy_static;

use crate::utils::__exit;
use super::{
    ConfigErrors,
    ConfigResult,
};

#[cfg(feature = "config-load-from-bin")]
pub const DEFAULT_CONFIG_DATA: &'static str =
    include_str!(concat!(env!("", "")));

lazy_static! {
    // NOTE: syntax highlight doesnt work inside lazy_static blocks inside intellij idea with rust plugin
    pub static ref DEFAULT_CONFIG_PATH: String = {
        let conf = dirs::config_dir();
        match conf {
            Some(config_path) => {
                format!("{}/tty-racer/config.toml", config_path.display())
            },
            None => {
                eprintln!("config dir doesnt exist on your platform, what?");
                eprintln!("if on linux, check: `$HOME/.config/");
                // NOTE: now the rust complier knows to not tell me
                // that is getting () instead of String
                // if i were using __exit(1) the compiler would say: error!!
                std::process::exit(1);
            }
        }
    };
}

#[derive(Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq)]
pub struct TyperacerConfig {
    pub(super) sleep_ms: u16,
    music: bool,
    pub(super) ui: UIConfig,
}

impl Default for TyperacerConfig {
    fn default() -> Self {
        Self {
            // 1 second
            sleep_ms: 1000,
            music: true,
            ui: UIConfig::default(),
        }
    }
}

#[derive(
Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone,
)]
pub enum Border {
    // this is to write
    // border = "round" inside the toml file
    #[serde(rename = "round")]
    // otherwise, it wouldnt work
    Round,
    #[serde(rename = "square")]
    Square
}

#[derive(Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq)]
pub struct UIConfig {
    wpm: bool,
    invisibles: bool,
    border: Border,
}

impl Default for UIConfig {
    fn default() -> Self {
        Self {
            wpm: true,
            invisibles: true,
            border: Border::Square,
        }
    }
}

impl TyperacerConfig {
    // fn _handle_load() {
    //     // first of all check if the file exists
    //
    //     if file_exists {
    //         // ask the user to create it
    //         // how do i ask the user ? using pretty prompt
    //         // when generating the config file generate with defaults
    //         // or let the use select on the fly
    //     } else {
    //         // just load the contents
    //     }
    // }

    fn check_values(_self: Self) -> ConfigResult<Self> {
        // check FPS
        let sleep_ms = _self.sleep_ms;
        if sleep_ms < 10 || sleep_ms > 1000 {
            return Err(ConfigErrors::SleepMSError(sleep_ms));
        }

        Ok(_self)
    }

    fn _load_from_str<S: AsRef<str>>(config_contents: S) -> ConfigResult<Self> {
        let config: Self = toml::from_str(config_contents.as_ref())?;
        Self::check_values(config)
    }

    fn _load_from_path<P>(path: P) -> ConfigResult<Self>
        where
            P: AsRef<Path>
    {
        let config_contents = fs::read_to_string(&path)?;
        let config = Self::_load_from_str(config_contents)?;
        Ok(config)
    }

    /// load from default path doesnt mean that the values are default
    pub fn load_default_path() -> ConfigResult<Self> {
        Self::_load_from_path(&*DEFAULT_CONFIG_PATH)
    }

    /// load from custom path
    pub fn load_from_toml<P>(path: P) -> ConfigResult<Self>
        where
            P: AsRef<Path>
    {
        Self::_load_from_path(path)
    }

    pub fn load_from_str<S: AsRef<str>>(file_contents: S) -> ConfigResult<Self> {
        Self::_load_from_str(file_contents)
    }

    #[cfg(feature = "config-load-from-bin")]
    pub fn load_from_binary() -> ConfigResult<Self> {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use assert2::assert;
    use rstest::rstest;

    use super::{
        ConfigErrors,
        ConfigResult,
        TyperacerConfig,
        DEFAULT_CONFIG_PATH,
    };

    #[test]
    /// load from default path doesnt mean that the values are default
    fn load_from_default_path() -> ConfigResult<()> {
        let loaded_from_path = TyperacerConfig::load_default_path()?;
        dbg!(&loaded_from_path);

        // let default_values = TyperacerConfig::default();

        // assert!(default_values == loaded_from_path);

        Ok(())
    }

    #[rstest]
    #[case(& * DEFAULT_CONFIG_PATH)]
    #[case("config.toml")]
    fn load_from_custom_path<P: AsRef<Path>>(
        #[case] path: P
    ) -> ConfigResult<()> {
        let res = TyperacerConfig::load_from_toml(path);
        match res {
            Ok(config) => {
                dbg!(&config);
            },
            Err(error) => {
                eprintln!("\n{}\n", error);
                return Err(error);
            }
        }
        Ok(())
    }
}
