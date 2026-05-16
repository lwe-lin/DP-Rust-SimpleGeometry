//! Wrapper for hot-reloadable plugin.
use dp_rust_simplegeometry::{fyrox::plugin::Plugin, Game};

#[no_mangle]
pub fn fyrox_plugin() -> Box<dyn Plugin> {
    Box::new(Game::default())
}
