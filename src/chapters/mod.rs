pub mod arc_sd;
pub mod arc_smp;
pub mod arc_sma;

use crate::state::game_state::{GameState, MathLevel};

/// Data lengkap satu chapter — narasi, puzzle, validasi.
pub struct ChapterData {
    pub number: usize,
    pub title: &'static str,
    pub math_level: MathLevel,
    /// Paragraf narasi pembuka — ditampilkan sebelum puzzle.
    pub narrative_intro: &'static [&'static str],
    /// Konteks cerita untuk puzzle.
    pub puzzle_context: &'static str,
    /// Pertanyaan matematika.
    pub puzzle_question: &'static str,
    /// Hint — ditampilkan setelah 2x salah.
    pub puzzle_hint: &'static str,
    /// Jawaban yang diterima (bisa lebih dari satu format).
    pub correct_answers: &'static [&'static str],
    /// Narasi setelah berhasil menjawab.
    pub narrative_success: &'static [&'static str],
    /// Pesan saat jawaban salah.
    pub narrative_failure: &'static str,
    /// Kunci variabel untuk disimpan (lintas chapter).
    pub variable_key: Option<&'static str>,
    /// Nilai yang disimpan (None = pakai jawaban pemain).
    pub variable_value: Option<&'static str>,
    /// Item yang didapat setelah berhasil.
    pub items_gained: &'static [&'static str],
    /// Damage kalau salah jawab.
    pub health_penalty: u32,
}

/// Ambil data chapter berdasarkan nomor (1-30).
pub fn get_chapter(n: usize) -> ChapterData {
    match n {
        1 => arc_sd::chapter_01(),
        2 => arc_sd::chapter_02(),
        3 => arc_sd::chapter_03(),
        4 => arc_sd::chapter_04(),
        5 => arc_sd::chapter_05(),
        6 => arc_sd::chapter_06(),
        7 => arc_sd::chapter_07(),
        8 => arc_sd::chapter_08(),
        9 => arc_sd::chapter_09(),
        10 => arc_sd::chapter_10(),
        11 => arc_smp::chapter_11(),
        12 => arc_smp::chapter_12(),
        13 => arc_smp::chapter_13(),
        14 => arc_smp::chapter_14(),
        15 => arc_smp::chapter_15(),
        16 => arc_smp::chapter_16(),
        17 => arc_smp::chapter_17(),
        18 => arc_smp::chapter_18(),
        19 => arc_smp::chapter_19(),
        20 => arc_smp::chapter_20(),
        21 => arc_sma::chapter_21(),
        22 => arc_sma::chapter_22(),
        23 => arc_sma::chapter_23(),
        24 => arc_sma::chapter_24(),
        25 => arc_sma::chapter_25(),
        26 => arc_sma::chapter_26(),
        27 => arc_sma::chapter_27(),
        28 => arc_sma::chapter_28(),
        29 => arc_sma::chapter_29(),
        30 => arc_sma::chapter_30(),
        _ => arc_sd::chapter_01(), // fallback
    }
}

/// Validasi jawaban pemain.
pub fn validate_answer(chapter: usize, answer: &str, _state: &GameState) -> bool {
    let data = get_chapter(chapter);
    let normalized = answer
        .trim()
        .to_lowercase()
        .replace(' ', "")
        .replace(',', ".");

    data.correct_answers
        .iter()
        .any(|&a| normalized == a.to_lowercase().replace(' ', "").replace(',', "."))
}

/// Total jumlah chapter dalam game.
pub const TOTAL_CHAPTERS: usize = 30;
