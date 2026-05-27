use gloo_storage::{LocalStorage, Storage};

use crate::state::game_state::GameState;

const SAVE_KEY: &str = "cetak_biru_berdarah_save";

/// Simpan game state ke localStorage.
pub fn save_game(state: &GameState) {
    if let Err(e) = LocalStorage::set(SAVE_KEY, state) {
        log::error!("Gagal menyimpan game: {:?}", e);
    } else {
        log::info!("Game tersimpan. Chapter {}", state.current_chapter);
    }
}

/// Muat game state dari localStorage.
pub fn load_game() -> Option<GameState> {
    match LocalStorage::get::<GameState>(SAVE_KEY) {
        Ok(state) => {
            log::info!("Game dimuat. Chapter {}", state.current_chapter);
            Some(state)
        }
        Err(_) => {
            log::info!("Tidak ada save data ditemukan.");
            None
        }
    }
}

/// Cek apakah ada save data.
pub fn has_save() -> bool {
    load_game().is_some()
}

/// Hapus save data.
pub fn delete_save() {
    LocalStorage::delete(SAVE_KEY);
    log::info!("Save data dihapus.");
}
