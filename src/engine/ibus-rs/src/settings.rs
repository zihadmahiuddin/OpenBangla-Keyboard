use qt_core::{qs, QBox, QSettings, QVariant};

use crate::constants::{self, paths};

pub struct Settings {
    q_settings: QBox<QSettings>,
}

impl Settings {
    pub fn new(organization: &str, application: &str) -> Self {
        let q_settings = unsafe { QSettings::from_2_q_string(&qs(organization), &qs(application)) };
        Self { q_settings }
    }

    pub fn get_layout_path(&self) -> String {
        unsafe {
            self.q_settings.sync();
            self.q_settings
                .value_2a(
                    &qs("layout/path"),
                    &QVariant::from_q_string(&qs(paths::AVRO_PHONETIC_LAYOUT_PATH)),
                )
                .to_string()
                .to_std_string()
        }
    }

    pub fn get_show_prev_win_fixed(&self) -> bool {
        unsafe {
            self.q_settings.sync();
            self.q_settings
                .value_2a(
                    &qs("settings/FixedLayout/ShowPrevWin"),
                    &QVariant::from_bool(true),
                )
                .to_bool()
        }
    }

    pub fn get_auto_vowel_form_fixed(&self) -> bool {
        unsafe {
            self.q_settings.sync();
            self.q_settings
                .value_2a(
                    &qs("settings/FixedLayout/AutoVowelForm"),
                    &QVariant::from_bool(true),
                )
                .to_bool()
        }
    }

    pub fn get_auto_chandra_pos_fixed(&self) -> bool {
        unsafe {
            self.q_settings.sync();
            self.q_settings
                .value_2a(
                    &qs("settings/FixedLayout/AutoChandraPos"),
                    &QVariant::from_bool(true),
                )
                .to_bool()
        }
    }
    pub fn get_traditional_kar_fixed(&self) -> bool {
        unsafe {
            self.q_settings.sync();
            self.q_settings
                .value_2a(
                    &qs("settings/FixedLayout/TraditionalKar"),
                    &QVariant::from_bool(true),
                )
                .to_bool()
        }
    }

    pub fn get_number_pad_fixed(&self) -> bool {
        unsafe {
            self.q_settings.sync();
            self.q_settings
                .value_2a(
                    &qs("settings/FixedLayout/NumberPad"),
                    &QVariant::from_bool(true),
                )
                .to_bool()
        }
    }

    pub fn get_old_reph(&self) -> bool {
        unsafe {
            self.q_settings.sync();
            self.q_settings
                .value_2a(
                    &qs("settings/FixedLayout/OldReph"),
                    &QVariant::from_bool(true),
                )
                .to_bool()
        }
    }

    pub fn get_candidate_win_horizontal(&self) -> bool {
        unsafe {
            self.q_settings.sync();
            self.q_settings
                .value_2a(
                    &qs("settings/CandidateWin/Horizontal"),
                    &QVariant::from_bool(true),
                )
                .to_bool()
        }
    }

    pub fn get_show_cwphonetic(&self) -> bool {
        unsafe {
            self.q_settings.sync();
            self.q_settings
                .value_2a(
                    &qs("settings/CandidateWin/Phonetic"),
                    &QVariant::from_bool(true),
                )
                .to_bool()
        }
    }

    pub fn get_suggestion_include_english(&self) -> bool {
        unsafe {
            self.q_settings.sync();
            self.q_settings
                .value_2a(
                    &qs("settings/PreviewWin/IncludeEnglish"),
                    &QVariant::from_bool(true),
                )
                .to_bool()
        }
    }

    pub fn get_fixed_old_kar_order(&self) -> bool {
        unsafe {
            self.q_settings.sync();
            self.q_settings
                .value_2a(
                    &qs("settings/FixedLayout/OldKarOrder"),
                    &QVariant::from_bool(true),
                )
                .to_bool()
        }
    }

    pub fn get_ansi_encoding(&self) -> bool {
        unsafe {
            self.q_settings.sync();
            self.q_settings
                .value_2a(
                    &qs("settings/ANSI"),
                    &QVariant::from_bool(true),
                )
                .to_bool()
        }
    }

    pub fn get_smart_quoting(&self) -> bool {
        unsafe {
            self.q_settings.sync();
            self.q_settings
                .value_2a(
                    &qs("settings/SmartQuoting"),
                    &QVariant::from_bool(true),
                )
                .to_bool()
        }
    }
}

impl Default for Settings {
    fn default() -> Self {
        Self::new(
            constants::DEFAULT_SETTINGS_ORGANIZATION,
            constants::DEFAULT_SETTINGS_APPLICATION,
        )
    }
}

// SAFETY: we only use a single thread in this program so it should be fine
unsafe impl Sync for Settings {}
unsafe impl Send for Settings {}

impl Drop for Settings {
    fn drop(&mut self) {
        unsafe { self.q_settings.sync() }
    }
}
