use my_penrose_config::{layouts::layouts, bar, bindings::raw_key_bindings};
use penrose::{
    core::{
        bindings::{parse_keybindings_with_xmodmap},
        Config, WindowManager,
    },
    x11rb::RustConn,
    extensions::hooks::add_ewmh_hooks,
};
use std::collections::HashMap;
use tracing_subscriber::{self, prelude::*};

fn main() -> anyhow::Result<()> {
    let tracing_builder = tracing_subscriber::fmt()
        .json() // JSON logs
        .flatten_event(true)
        .with_env_filter("info")
        .with_filter_reloading();

    let reload_handle = tracing_builder.reload_handle();
    tracing_builder.finish().init();

    let config = add_ewmh_hooks(Config {
        default_layouts: layouts(),
        ..Config::default()
    });

    let conn = RustConn::new()?;
    let raw_bindings = raw_key_bindings(reload_handle);
    let key_bindings = parse_keybindings_with_xmodmap(raw_bindings)?;
    let wm = WindowManager::new(config, key_bindings, HashMap::new(), conn)?;

    let bar = bar::status_bar()?;
    let wm = bar.add_to(wm);

    wm.run()?;

    Ok(())
}
