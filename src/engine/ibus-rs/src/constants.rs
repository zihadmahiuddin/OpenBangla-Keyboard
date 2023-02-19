pub mod metadata {
    pub const BUS_NAME: &str = "org.freedesktop.IBus.OpenBangla";
    pub const NAME: &str = "OpenBangla";
    pub const LONG_NAME: &str = "OpenBangla Keyboard";
    pub const DESC: &str = "OpenBangla Keyboard IME for iBus";
    pub const LANG: &str = "bn";
    pub const LICENSE: &str = "GPL 3";
    pub const AUTHOR: &str = "See About Dialog";
    pub const VERSION: &str = "2.0.0";
    pub const HOMEPAGE: &str = "http://openbangla.github.io/";
    pub const CMDLINE: &str = "/home/zihad/Documents/Projects/GitHub/OpenBangla/OpenBangla-Keyboard/src/engine/ibus-rs/target/debug/ibus-rs --ibus";
    pub const TEXTDOMAIN: &str = "openbangla-keyboard";
    pub const ICON: &str = "/usr/share/icons/OpenBangla-Keyboard.png";
    pub const DEFAULT_LAYOUT: &str = "default";
}

pub mod paths {
    pub const AVRO_PHONETIC_LAYOUT_PATH: &str =
        "/usr/share/openbangla-keyboard/layouts/avrophonetic.json";
    pub const DATABASE_PATH: &str = "/usr/share/openbangla-keyboard/data";
}

pub const DEFAULT_SETTINGS_ORGANIZATION: &str = "OpenBangla";
pub const DEFAULT_SETTINGS_APPLICATION: &str = "Keyboard";
