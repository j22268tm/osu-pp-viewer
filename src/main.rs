mod context;
mod monitor;
mod calculator;

use dotenv::dotenv;
// use std::env;
use futures::StreamExt;
use crate::calculator::Calculator;

#[tokio::main]
async fn main() {
    dotenv().ok();
    println!("Starting osu! PP Viewer...");

    // Basic loop for now
    let ws_url = "ws://localhost:24050/ws";
    
    match monitor::connect(ws_url).await {
        Ok(mut ws_stream) => {
            let mut calculator = Calculator::new();
            println!("Connected. Waiting for gameplay data...");
            
            while let Some(msg) = ws_stream.next().await {
                match msg {
                    Ok(msg) => {
                        if msg.is_text() {
                            let text = msg.to_text().unwrap();
                            
                            if let Ok(state) = serde_json::from_str::<monitor::GosuMemoryState>(text) {
                                // Load map if changed
                                // Note: gosumemory path is usually URL encoded or needs decoding? 
                                // Actually it's usually a standard system path string.
                                if let Err(e) = calculator.load_map(&state.menu.bm.path.full) {
                                    eprintln!("Failed to load map: {}", e);
                                    continue;
                                }

                                // Parse mods
                                let mods = parse_mods(&state.menu.mods.str);

                                // Calculate PP
                                let pp = calculator.calculate_pp(
                                    0, // TODO: Mode detection (0 = osu!)
                                    mods,
                                    state.gameplay.combo.current as usize,
                                    state.gameplay.hits.n300 as usize,
                                    state.gameplay.hits.n100 as usize,
                                    state.gameplay.hits.n50 as usize,
                                    state.gameplay.hits.n0 as usize, // Misses
                                );

                                if let Some(pp) = pp {
                                    // Clear screen or just print line
                                    println!("PP: {:.2} | Combo: {} | Acc: {:.2}%", 
                                        pp, 
                                        state.gameplay.combo.current,
                                        state.gameplay.accuracy
                                    );
                                }
                            }
                        }
                    }
                    Err(e) => eprintln!("Error receiving message: {}", e),
                }
            }
        },
        Err(e) => {
            eprintln!("Failed to connect to gosumemory: {}", e);
            eprintln!("Make sure gosumemory is running on port 24050");
        }
    }
}

fn parse_mods(mods_str: &str) -> u32 {
    let mut mask = 0;
    // Basic mod parsing
    if mods_str.contains("NF") { mask |= 1 << 0; }
    if mods_str.contains("EZ") { mask |= 1 << 1; }
    if mods_str.contains("TD") { mask |= 1 << 2; }
    if mods_str.contains("HD") { mask |= 1 << 3; }
    if mods_str.contains("HR") { mask |= 1 << 4; }
    if mods_str.contains("SD") { mask |= 1 << 5; }
    if mods_str.contains("DT") { mask |= 1 << 6; }
    if mods_str.contains("RX") { mask |= 1 << 7; }
    if mods_str.contains("HT") { mask |= 1 << 8; }
    if mods_str.contains("NC") { mask |= 1 << 6; mask |= 1 << 9; } // NC implies DT
    if mods_str.contains("FL") { mask |= 1 << 10; }
    if mods_str.contains("SO") { mask |= 1 << 12; }
    if mods_str.contains("AP") { mask |= 1 << 13; }
    if mods_str.contains("PF") { mask |= 1 << 14; }
    // 4K, 5K etc for mania
    mask
}
