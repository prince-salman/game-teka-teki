use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Fase utama game — menentukan screen mana yang aktif.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum GamePhase {
    TitleScreen,
    NameInput,
    Playing,
    GameOver,
    Victory,
}

/// Fase dalam satu chapter — menentukan konten apa yang ditampilkan.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ChapterPhase {
    Intro,
    Puzzle,
    Success,
    Failure,
    Transition,
}

/// Level kognitif puzzle matematika.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum MathLevel {
    LOTS,
    MOTS,
    HOTS,
}

impl MathLevel {
    pub fn label(&self) -> &'static str {
        match self {
            MathLevel::LOTS => "LOTS",
            MathLevel::MOTS => "MOTS",
            MathLevel::HOTS => "HOTS",
        }
    }

    pub fn badge_color(&self) -> &'static str {
        match self {
            MathLevel::LOTS => "background: rgba(109, 170, 109, 0.2); color: #6daa6d; border: 1px solid rgba(109, 170, 109, 0.3);",
            MathLevel::MOTS => "background: rgba(198, 134, 66, 0.2); color: #c68642; border: 1px solid rgba(198, 134, 66, 0.3);",
            MathLevel::HOTS => "background: rgba(139, 0, 0, 0.2); color: #cc4444; border: 1px solid rgba(139, 0, 0, 0.3);",
        }
    }
}

/// State utama game — semua progress pemain ada di sini.
/// Fully serializable untuk save/load.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GameState {
    pub phase: GamePhase,
    pub current_chapter: usize,
    pub chapter_phase: ChapterPhase,

    /// Variabel lintas chapter — jawaban dari chapter awal jadi kunci chapter selanjutnya.
    pub variables: HashMap<String, String>,

    /// Inventaris pemain — item yang dikumpulkan sepanjang petualangan.
    pub inventory: Vec<String>,

    /// Flag boolean — puzzle solved, door unlocked, dsb.
    pub flags: HashMap<String, bool>,

    /// Nama pemain — diinput di awal game.
    pub player_name: String,

    /// Nyawa pemain — berkurang kalau salah jawab (analog trap damage).
    pub health: u32,

    /// Skor — berapa puzzle yang dijawab benar di percobaan pertama.
    pub score: u32,

    /// Berapa kali salah jawab di chapter ini.
    pub wrong_attempts: u32,

    /// Apakah hint sedang ditampilkan.
    pub showing_hint: bool,

    /// Jawaban terakhir yang salah (untuk feedback).
    pub last_wrong_answer: String,

    /// Sisa penggunaan hint (maks 3 per game).
    pub hints_remaining: u32,
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            phase: GamePhase::TitleScreen,
            current_chapter: 1,
            chapter_phase: ChapterPhase::Intro,
            variables: HashMap::new(),
            inventory: Vec::new(),
            flags: HashMap::new(),
            player_name: String::new(),
            health: 100,
            score: 0,
            wrong_attempts: 0,
            showing_hint: false,
            last_wrong_answer: String::new(),
            hints_remaining: 3,
        }
    }
}

impl GameState {
    pub fn new_game(name: String) -> Self {
        Self {
            phase: GamePhase::Playing,
            player_name: name,
            ..Self::default()
        }
    }

    pub fn has_item(&self, item: &str) -> bool {
        self.inventory.iter().any(|i| i == item)
    }

    pub fn get_variable(&self, key: &str) -> Option<&String> {
        self.variables.get(key)
    }

    pub fn is_flag_set(&self, flag: &str) -> bool {
        self.flags.get(flag).copied().unwrap_or(false)
    }

    pub fn health_percentage(&self) -> u32 {
        self.health.min(100)
    }

    pub fn health_color(&self) -> &'static str {
        if self.health > 60 {
            "background: linear-gradient(90deg, #6daa6d, #4a8a4a);"
        } else if self.health > 30 {
            "background: linear-gradient(90deg, #c68642, #a06830);"
        } else {
            "background: linear-gradient(90deg, #cc4444, #8b0000);"
        }
    }

    /// Ending ditentukan berdasarkan skor dan health.
    pub fn ending_type(&self) -> EndingType {
        if self.health == 0 {
            EndingType::Death
        } else if self.score >= 25 {
            EndingType::Perfect
        } else if self.score >= 15 {
            EndingType::Good
        } else {
            EndingType::Bad
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum EndingType {
    Death,
    Bad,
    Good,
    Perfect,
}
