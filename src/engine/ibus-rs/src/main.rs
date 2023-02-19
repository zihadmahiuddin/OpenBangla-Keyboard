mod constants;
mod keycode;
mod settings;
mod signal_handler;
pub mod state;

fn main() {
    {
        // Make sure the local state is up-to-date with the settings
        let ibus_open_bangla = state::get_state();
        let mut ibus_open_bangla = ibus_open_bangla.lock().unwrap();
        ibus_open_bangla.update_with_settings();
    }

    ibus::main();
}
