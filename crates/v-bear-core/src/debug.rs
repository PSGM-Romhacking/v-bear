//! Debug switches read once from the environment (`V_BEAR_DEBUG=snd,t65`).

use std::sync::OnceLock;

pub fn on(name: &str) -> bool {
    static FLAGS: OnceLock<Vec<String>> = OnceLock::new();
    FLAGS
        .get_or_init(|| {
            std::env::var("V_BEAR_DEBUG")
                .map(|v| v.split(',').map(str::to_owned).collect())
                .unwrap_or_default()
        })
        .iter()
        .any(|f| f == name)
}
