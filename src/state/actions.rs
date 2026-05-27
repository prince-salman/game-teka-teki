use std::rc::Rc;
use yew::prelude::*;

use crate::chapters::{get_chapter, validate_answer};
use crate::state::game_state::*;
use crate::state::persistence;

/// Semua aksi yang bisa terjadi dalam game.
pub enum GameAction {
    /// Mulai game baru dari title screen.
    StartNewGame,
    /// Set nama pemain dari input.
    SetPlayerName(String),
    /// Pindah fase chapter (Intro → Puzzle → Success → Transition).
    SetChapterPhase(ChapterPhase),
    /// Kirim jawaban puzzle.
    SubmitAnswer(String),
    /// Lanjut ke chapter berikutnya.
    NextChapter,
    /// Tambah item ke inventaris.
    AddItem(String),
    /// Set variabel lintas chapter.
    SetVariable(String, String),
    /// Set flag boolean.
    SetFlag(String, bool),
    /// Kurangi nyawa (jebakan analog).
    TakeDamage(u32),
    /// Tampilkan hint.
    ShowHint,
    /// Simpan game ke localStorage.
    SaveGame,
    /// Muat game dari localStorage.
    LoadGame,
    /// Ulangi chapter ini.
    RestartChapter,
    /// Kembali ke title screen.
    GoToTitle,
}

/// Type alias untuk context — dipakai semua komponen.
pub type GameContext = UseReducerHandle<GameState>;

impl Reducible for GameState {
    type Action = GameAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        let mut next = (*self).clone();

        match action {
            GameAction::StartNewGame => {
                next.phase = GamePhase::NameInput;
            }

            GameAction::SetPlayerName(name) => {
                if !name.trim().is_empty() {
                    next = GameState::new_game(name.trim().to_string());
                }
            }

            GameAction::SetChapterPhase(phase) => {
                next.chapter_phase = phase;
            }

            GameAction::SubmitAnswer(answer) => {
                let chapter_data = get_chapter(next.current_chapter);

                if validate_answer(next.current_chapter, &answer, &next) {
                    // Jawaban benar!
                    next.chapter_phase = ChapterPhase::Success;
                    if next.wrong_attempts == 0 {
                        next.score += 1; // Bonus skor kalau benar di percobaan pertama
                    }

                    // Simpan variabel lintas chapter
                    if let Some(key) = chapter_data.variable_key {
                        let val = match chapter_data.variable_value {
                            Some(v) => v.to_string(),
                            None => answer.trim().to_string(),
                        };
                        next.variables.insert(key.to_string(), val);
                    }

                    // Dapat item baru
                    for item in chapter_data.items_gained {
                        if !next.inventory.contains(&item.to_string()) {
                            next.inventory.push(item.to_string());
                        }
                    }

                    // Mark chapter sebagai solved
                    next.flags.insert(
                        format!("chapter_{}_solved", next.current_chapter),
                        true,
                    );

                    // Reset wrong attempts
                    next.wrong_attempts = 0;
                    next.showing_hint = false;
                    next.last_wrong_answer.clear();
                } else {
                    // Jawaban salah
                    next.wrong_attempts += 1;
                    next.last_wrong_answer = answer.trim().to_string();
                    next.chapter_phase = ChapterPhase::Failure;

                    // Kurangi nyawa
                    next.health = next.health.saturating_sub(chapter_data.health_penalty);

                    // Cek game over
                    if next.health == 0 {
                        next.phase = GamePhase::GameOver;
                    }
                }
            }

            GameAction::NextChapter => {
                // Auto-save sebelum lanjut
                persistence::save_game(&next);

                if next.current_chapter >= 30 {
                    // Game selesai!
                    next.phase = GamePhase::Victory;
                } else {
                    next.current_chapter += 1;
                    next.chapter_phase = ChapterPhase::Intro;
                    next.wrong_attempts = 0;
                    next.showing_hint = false;
                    next.last_wrong_answer.clear();
                }
            }

            GameAction::AddItem(item) => {
                if !next.inventory.contains(&item) {
                    next.inventory.push(item);
                }
            }

            GameAction::SetVariable(key, value) => {
                next.variables.insert(key, value);
            }

            GameAction::SetFlag(flag, value) => {
                next.flags.insert(flag, value);
            }

            GameAction::TakeDamage(damage) => {
                next.health = next.health.saturating_sub(damage);
                if next.health == 0 {
                    next.phase = GamePhase::GameOver;
                }
            }

            GameAction::ShowHint => {
                if next.hints_remaining > 0 && !next.showing_hint {
                    next.hints_remaining -= 1;
                    next.showing_hint = true;
                }
            }

            GameAction::SaveGame => {
                persistence::save_game(&next);
            }

            GameAction::LoadGame => {
                if let Some(saved) = persistence::load_game() {
                    next = saved;
                }
            }

            GameAction::RestartChapter => {
                next.chapter_phase = ChapterPhase::Intro;
                next.wrong_attempts = 0;
                next.showing_hint = false;
                next.last_wrong_answer.clear();
            }

            GameAction::GoToTitle => {
                next = GameState::default();
            }
        }

        Rc::new(next)
    }
}
