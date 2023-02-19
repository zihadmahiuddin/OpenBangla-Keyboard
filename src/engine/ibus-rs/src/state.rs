use std::sync::{
    atomic::{AtomicU16, Ordering},
    Arc, Mutex,
};

use ibus::{
    Bus, BusExt, BusExtManual, Component, ComponentExt, Engine, EngineDesc, EngineExt, Factory,
    FactoryExt, LookupTable, LookupTableExt, LookupTableExtManual, Orientation, PreeditFocusMode,
    Text, TextExt,
};

use lazy_static::lazy_static;

use riti::{config::Config as RitiConfig, context::RitiContext, suggestion::Suggestion};

use crate::{
    constants::{metadata::*, paths},
    settings::Settings,
    signal_handler,
};

lazy_static! {
    static ref IBUS_OPEN_BANGLA: Arc<Mutex<State>> = Arc::new(Mutex::new({
        let mut args = std::env::args();
        start_setup(args.len() > 1 && args.nth(1) == Some("--ibus".to_string()))
    }));
    static ref ENGINE_COUNTER: AtomicU16 = AtomicU16::new(0);
}

pub struct State {
    pub(crate) riti_cfg: RitiConfig,
    pub(crate) riti_ctx: RitiContext,
    pub(crate) last_suggestion: Option<Suggestion>,
    pub(crate) lookup_table: LookupTable,
    pub(crate) settings: Settings,
    pub(crate) alt_gr: bool,
}

impl State {
    pub fn ctx(&self) -> &RitiContext {
        &self.riti_ctx
    }

    pub fn update_with_settings(&mut self) {
        update_riti_config(&self.settings, &mut self.riti_cfg);

        let orientation = if self.settings.get_candidate_win_horizontal() {
            Orientation::Horizontal
        } else {
            Orientation::Vertical
        };
        self.lookup_table.set_orientation(orientation);
    }

    pub fn update_lookup_table(&mut self, engine: &Engine) {
        match &self.last_suggestion {
            Some(last_suggestion) if !last_suggestion.is_lonely() => {
                let aux = last_suggestion.get_auxiliary_text();
                let aux = Text::from_string(aux);

                self.lookup_table.clear();
                engine.update_auxiliary_text(&aux, true);

                let suggestions = last_suggestion.get_suggestions();

                for suggestion in suggestions {
                    let text = Text::from_string(suggestion);
                    self.lookup_table.append_candidate(&text);
                }

                let prev_selection = last_suggestion.previously_selected_index();
                self.lookup_table.set_cursor_pos(
                    prev_selection
                        .try_into()
                        .expect("Lookup table cursor position too big."),
                );
            }
            _ => {}
        }
        self.update_preedit(engine);
    }

    pub fn update_preedit(&self, engine: &Engine) {
        let text = self.last_suggestion.as_ref().map(|last_suggestion| {
            if !last_suggestion.is_lonely() {
                engine.update_lookup_table_fast(&self.lookup_table, true);
                let index = self.lookup_table.cursor_pos();

                last_suggestion.get_pre_edit_text(index as usize)
            } else {
                last_suggestion.get_pre_edit_text(0)
            }
        });
        if let Some(text) = text {
            let text = Text::from_string(&text);
            engine.update_preedit_text_with_mode(
                &text,
                text.length(),
                true,
                PreeditFocusMode::Commit,
            );
        }
    }

    pub fn reset_engine(&self, engine: &Engine) {
        self.lookup_table.clear();
        engine.hide_preedit_text();
        engine.hide_auxiliary_text();
        engine.hide_lookup_table();
    }

    pub fn commit(&self, engine: &Engine) {
        let commit_result = match &self.last_suggestion {
            Some(last_suggestion) => {
                if !last_suggestion.is_lonely() {
                    let index = self.lookup_table.cursor_pos();
                    Some((index, last_suggestion.get_pre_edit_text(index as usize)))
                } else {
                    Some((0, last_suggestion.get_lonely_suggestion().to_owned()))
                }
            }
            None => None,
        };

        if let Some((index, text)) = commit_result {
            let text = Text::from_string(&text);
            self.commit_text(engine, &text);
            self.riti_ctx
                .candidate_committed(index.try_into().expect("Couldn't convert u32 to usize..."));
        }
    }

    pub fn commit_text(&self, engine: &Engine, text: &Text) {
        engine.commit_text(text);
        self.reset_engine(engine);
    }

    pub fn update_engine(&mut self) {
        self.riti_ctx.update_engine(&self.riti_cfg);
    }
}

fn start_setup(ibus: bool) -> State {
    ibus::init();

    let bus = Bus::new();

    bus.connect_connected(signal_handler::bus::connected);
    bus.connect_disconnected(signal_handler::bus::disconnected);

    let connection = bus.connection().expect("Failed to get ibus connection.");

    let factory = Factory::new(&connection);

    {
        factory.connect_create_engine(move |_factory, engine_name| {
            println!("Engine {engine_name} creating!");
            let path = format!(
                "/org/freedesktop/IBus/Engine/{}",
                ENGINE_COUNTER.fetch_add(1, Ordering::Relaxed)
            );
            let engine = Engine::new(NAME, &path, &connection);

            engine.connect_enable(signal_handler::engine::enable);
            engine.connect_disable(signal_handler::engine::disable);
            engine.connect_reset(signal_handler::engine::reset);
            engine.connect_focus_out(signal_handler::engine::focus_out);
            engine.connect_candidate_clicked(signal_handler::engine::candidate_clicked);
            engine.connect_process_key_event(signal_handler::engine::process_key_event);

            Some(engine)
        });
    }

    if ibus {
        bus.request_name(BUS_NAME, 0);
    } else {
        let component = Component::new(
            BUS_NAME, LONG_NAME, VERSION, LICENSE, AUTHOR, HOMEPAGE, CMDLINE, TEXTDOMAIN,
        );

        let engine_desc = EngineDesc::new(
            NAME,
            LONG_NAME,
            DESC,
            LANG,
            LICENSE,
            AUTHOR,
            ICON,
            DEFAULT_LAYOUT,
        );

        component.add_engine(&engine_desc);
        bus.register_component(&component);
        bus.set_global_engine_async("OpenBangla", None);
    }

    let settings = Settings::default();
    let mut riti_cfg = RitiConfig::default();
    update_riti_config(&settings, &mut riti_cfg);

    let riti_ctx = RitiContext::new_with_config(&riti_cfg);

    State {
        riti_cfg,
        riti_ctx,
        last_suggestion: None,
        lookup_table: LookupTable::new(10, 0, true, true),
        settings,
        alt_gr: false,
    }
}

pub fn update_riti_config(settings: &Settings, riti_cfg: &mut RitiConfig) {
    riti_cfg.set_database_dir(paths::DATABASE_PATH);
    riti_cfg.set_layout_file_path(&settings.get_layout_path());

    riti_cfg.set_phonetic_suggestion(settings.get_show_cwphonetic());
    riti_cfg.set_suggestion_include_english(settings.get_suggestion_include_english());
    riti_cfg.set_fixed_suggestion(settings.get_show_prev_win_fixed());
    riti_cfg.set_fixed_automatic_vowel(settings.get_auto_vowel_form_fixed());
    riti_cfg.set_fixed_automatic_chandra(settings.get_auto_chandra_pos_fixed());
    riti_cfg.set_fixed_traditional_kar(settings.get_traditional_kar_fixed());
    riti_cfg.set_fixed_old_kar_order(settings.get_fixed_old_kar_order());
    riti_cfg.set_fixed_old_reph(settings.get_old_reph());
    riti_cfg.set_fixed_numpad(settings.get_number_pad_fixed());
    riti_cfg.set_ansi_encoding(settings.get_ansi_encoding());
    riti_cfg.set_smart_quote(settings.get_smart_quoting());
}

pub fn get_state() -> Arc<Mutex<State>> {
    Arc::clone(&*IBUS_OPEN_BANGLA)
}

// SAFETY: we only use a single thread in this program so it should be fine
unsafe impl Sync for State {}
unsafe impl Send for State {}
