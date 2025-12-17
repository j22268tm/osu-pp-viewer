use rosu_v2::Osu;
use std::sync::Arc;
// use tokio::sync::Mutex;

#[allow(dead_code)]
#[derive(Clone)]
pub struct Context {
    pub osu: Arc<Osu>,
    // We might add more shared state here later, like current map cache
}

#[allow(dead_code)]
impl Context {
    pub fn new(osu: Osu) -> Self {
        Self {
            osu: Arc::new(osu),
        }
    }
}
