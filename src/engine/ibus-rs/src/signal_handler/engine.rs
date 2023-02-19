use ibus::keys::{
    ALT_L, ALT_R, BACKSPACE, CONTROL_L, CONTROL_R, DOWN, ISO_LEVEL3_SHIFT, KP_ENTER, LEFT, META_L,
    META_R, RETURN, RIGHT, SHIFT_L, SHIFT_R, SPACE, TAB, UP,
};
use ibus::{Engine, LookupTableExt, ModifierType};

use crate::{keycode, state::get_state};

pub fn enable(_engine: &Engine) {
    println!("Engine enabled.");

    let ibus_open_bangla = get_state();
    {
        let mut ibus_open_bangla = ibus_open_bangla.lock().unwrap();
        ibus_open_bangla.update_engine();
    }
}

pub fn disable(_engine: &Engine) {
    println!("Engine disabled.");
}

pub fn reset(engine: &Engine) {
    println!("Engine reset.");

    let ibus_open_bangla = get_state();
    {
        let ibus_open_bangla = ibus_open_bangla.lock().unwrap();
        if ibus_open_bangla.riti_ctx.ongoing_input_session() {
            ibus_open_bangla.riti_ctx.finish_input_session();
            ibus_open_bangla.reset_engine(engine);
        }
    }
}

pub fn focus_out(engine: &Engine) {
    let ibus_open_bangla = get_state();
    {
        let ibus_open_bangla = ibus_open_bangla.lock().unwrap();
        if ibus_open_bangla.riti_ctx.ongoing_input_session() {
            ibus_open_bangla.riti_ctx.finish_input_session();
            ibus_open_bangla.reset_engine(engine);
        }
    }
    println!("Engine focus out.");
}

pub fn candidate_clicked(engine: &Engine, index: u32, button: u32, state: u32) {
    let ibus_open_bangla = get_state();
    {
        let ibus_open_bangla = ibus_open_bangla.lock().unwrap();
        if let Some(text) = ibus_open_bangla.lookup_table.candidate(index) {
            ibus_open_bangla.commit_text(engine, &text);
            ibus_open_bangla
                .riti_ctx
                .candidate_committed(index.try_into().expect("Couldn't convert u32 to usize..."));
        }
    }

    println!("Engine candidate clicked {}, {}, {}", index, button, state);
}

pub fn process_key_event(engine: &Engine, keyval: u32, _keycode: u32, state: u32) -> bool {
    let mut modifier: u8 = 0;
    let mut ctrl_key = false;
    let mut alt_key = false;
    let keyval = keyval as i32;

    let state = ModifierType::from_bits(state).unwrap_or_else(|| panic!("Invalid state {}", state));

    let ibus_open_bangla = get_state();

    // Don't accept Key Release event
    if state.contains(ModifierType::RELEASE_MASK) {
        if keyval == ALT_R || keyval == ISO_LEVEL3_SHIFT {
            let mut ibus_open_bangla = ibus_open_bangla.lock().unwrap();
            ibus_open_bangla.alt_gr = false;
        }
        return false;
    }

    {
        let mut ibus_open_bangla = ibus_open_bangla.lock().unwrap();
        if !ibus_open_bangla.riti_ctx.ongoing_input_session() {
            ibus_open_bangla.update_with_settings();
            ibus_open_bangla.update_engine();
        }
    }

    // At first, handle the special keys.
    match keyval {
        BACKSPACE => {
            let mut ibus_open_bangla = ibus_open_bangla.lock().unwrap();
            if ibus_open_bangla.riti_ctx.ongoing_input_session() {
                let ctrl_mod = state.contains(ModifierType::CONTROL_MASK);
                let suggestion = ibus_open_bangla.riti_ctx.backspace_event(ctrl_mod);

                if !suggestion.is_empty() {
                    ibus_open_bangla.update_lookup_table(engine);
                    ibus_open_bangla.last_suggestion = Some(suggestion);
                } else {
                    ibus_open_bangla.reset_engine(engine);
                }

                return true;
            } else {
                return false;
            }
        }
        RETURN => {
            {
                let ibus_open_bangla = ibus_open_bangla.lock().unwrap();
                if ibus_open_bangla.riti_ctx.ongoing_input_session() {
                    ibus_open_bangla.commit(engine);
                    // return (gboolean) gSettings->getEnterKeyClosesPrevWin();
                    return true;
                } else {
                    return false;
                }
            }
        }
        SPACE | KP_ENTER => {
            let ibus_open_bangla = ibus_open_bangla.lock().unwrap();
            if ibus_open_bangla.riti_ctx.ongoing_input_session() {
                ibus_open_bangla.commit(engine);
            }
            return false;
        }
        // Arrow and Tab keys.
        // We use the arrow keys and the tab key to move the selection
        // in the preview window. So we have to ensure the preview
        // window is shown by checking if the current suggestion is
        // not a lonely one. Otherwise we don't handle it.
        RIGHT | LEFT | UP | DOWN | TAB => {
            let ibus_open_bangla = ibus_open_bangla.lock().unwrap();
            if ibus_open_bangla.riti_ctx.ongoing_input_session() {
                let candidate_win_horizontal =
                    ibus_open_bangla.settings.get_candidate_win_horizontal();
                let is_lonely = ibus_open_bangla
                    .last_suggestion
                    .as_ref()
                    .map(|s| s.is_lonely())
                    .unwrap_or(true);

                if is_lonely {
                    ibus_open_bangla.commit(engine);
                } else {
                    let should_go_down = (candidate_win_horizontal && keyval == RIGHT)
                        || (!candidate_win_horizontal && keyval == DOWN)
                        || keyval == TAB;
                    let should_go_up = (candidate_win_horizontal && keyval == LEFT)
                        || (!candidate_win_horizontal && keyval == UP);

                    if should_go_down {
                        ibus_open_bangla.lookup_table.cursor_down();
                    } else if should_go_up {
                        ibus_open_bangla.lookup_table.cursor_up();
                    }

                    if should_go_up || should_go_down {
                        ibus_open_bangla.update_preedit(engine);
                        return true;
                    }
                }
            } else {
                return false;
            }
        }
        // Modifier keys
        ALT_R | ISO_LEVEL3_SHIFT | SHIFT_L | SHIFT_R | CONTROL_L | CONTROL_R | ALT_L | META_L
        | META_R => {
            let mut ibus_open_bangla = ibus_open_bangla.lock().unwrap();

            if keyval == ALT_R || keyval == ISO_LEVEL3_SHIFT {
                // Keep track of the right Alt key (also known as the AltGr key)
                ibus_open_bangla.alt_gr = true;
            }

            return ibus_open_bangla.riti_ctx.ongoing_input_session();
        }
        _ => {}
    }

    // Set modifiers
    if state.contains(ModifierType::SHIFT_MASK) {
        modifier |= riti::context::MODIFIER_SHIFT;
    }

    if state.contains(ModifierType::CONTROL_MASK) {
        ctrl_key = true;
    }

    if state.contains(ModifierType::MOD1_MASK) {
        alt_key = true;
    }

    // Convert the key value into riti's key value.
    let riti_key = keycode::ibus_to_riti_keycode(keyval);

    {
        let ibus_open_bangla = ibus_open_bangla.lock().unwrap();
        // Reject the key which has only Ctrl or Alt (not the right one) combination and riti doesn't handle.
        if (ctrl_key && !alt_key)
            || (!ctrl_key && alt_key && !ibus_open_bangla.alt_gr)
            || riti_key.is_none()
        {
            if ibus_open_bangla.riti_ctx.ongoing_input_session() {
                ibus_open_bangla.commit(engine);
            }
            return false;
        }
    }
    {
        // If we have Ctrl and Alt combination or the right Alt, set it as the AltGr modifier.
        let ibus_open_bangla = ibus_open_bangla.lock().unwrap();
        if (ctrl_key && alt_key) || ibus_open_bangla.alt_gr {
            modifier |= riti::context::MODIFIER_ALT_GR;
        }
    }

    let mut ibus_open_bangla = ibus_open_bangla.lock().unwrap();
    let suggestion = ibus_open_bangla.riti_ctx.get_suggestion_for_key(
        riti_key.unwrap_or_else(|| panic!("Invalid ibus keycode {keyval}")),
        modifier,
    );

    if !suggestion.is_empty() {
        ibus_open_bangla.last_suggestion = Some(suggestion);
        ibus_open_bangla.update_lookup_table(engine);
    } else {
        // Corner case: When old style kar typing is enabled, a lonely suggestion and an empty
        // suggestion is not distinguishable. So we accept the key event if a input session
        // is ongoing.
        return ibus_open_bangla.riti_ctx.ongoing_input_session();
    }

    true
}
