use rosu_pp::{Beatmap, Performance};
use std::error::Error;

pub struct Calculator {
    current_map: Option<Beatmap>,
    current_map_path: String,
}

impl Calculator {
    pub fn new() -> Self {
        Self {
            current_map: None,
            current_map_path: String::new(),
        }
    }

    pub fn load_map(&mut self, path: &str) -> Result<(), Box<dyn Error>> {
        if self.current_map_path != path {
            self.current_map = Some(Beatmap::from_path(path)?);
            self.current_map_path = path.to_string();
            println!("Loaded map: {}", path);
        }
        Ok(())
    }

    pub fn calculate_pp(&self, _mode: u8, mods: u32, combo: usize, n300: usize, n100: usize, n50: usize, n_miss: usize) -> Option<f64> {
        let map = self.current_map.as_ref()?;

        // rosu-pp 1.0.0 usage
        let pp_result = Performance::new(map)
            .mods(mods)
            .combo(combo as u32)
            .n300(n300 as u32)
            .n100(n100 as u32)
            .n50(n50 as u32)
            .misses(n_miss as u32)
            .calculate();

        Some(pp_result.pp())
    }
}
