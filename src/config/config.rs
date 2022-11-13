use std::path::Path;
use std::fs;

use toml;
use dirs;
use lazy_static::lazy_static;

use crate::utils::__exit;
use super::{ConfigResult, ConfigErrors};

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

#[derive(Debug, serde::Serialize, serde::Deserialize, PartialEq)]
pub struct TyperacerConfig {
    fps: u8,
    music: bool,
    ui: UIConfig,
}

impl Default for TyperacerConfig {
    fn default() -> Self {
        Self {
            fps: 60,
            music: true,
            ui: UIConfig::default(),
        }
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize, PartialEq, Clone)]
pub enum Border {
    // this is to write
    // border = "round" inside the toml file
    #[serde(rename = "round")]
    // otherwise, it wouldnt work
    Round,
    #[serde(rename = "square")]
    Square,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, PartialEq)]
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

        if 1 > _self.fps || _self.fps > 100 {
            return Err(ConfigErrors::FPSError(_self.fps));
        }
        Ok(_self)
    }

    fn _load_from_path<P>(path: P) -> ConfigResult<Self>
        where
            P: AsRef<Path>
    {
        let config_contents = fs::read_to_string(&path)?;
        let config: Self = toml::from_str(&config_contents)?;
        let config = Self::check_values(config)?;
        Ok(config)
    }

    /// load from default path doesnt mean that the values are default
    pub fn load_default_path() -> ConfigResult<Self>
    {
        Self::_load_from_path(&*DEFAULT_CONFIG_PATH)
    }

    /// load from custom path
    pub fn load_from_toml<P>(path: P) -> ConfigResult<Self>
        where
            P: AsRef<Path>
    {
        Self::_load_from_path(path)
    }
}

#[cfg(test)]
mod config {
    use super::{
        ConfigResult,
        TyperacerConfig,
        ConfigErrors,
        DEFAULT_CONFIG_PATH,
    };

    use assert2::assert;
    use rstest::rstest;
    use std::path::Path;

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
