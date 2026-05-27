use yew::prelude::*;

use crate::state::actions::{GameAction, GameContext};
use crate::state::game_state::EndingType;
use crate::styles;

#[derive(Properties, PartialEq)]
pub struct EndScreenProps {
    pub game_over: bool,
}

#[function_component(EndScreen)]
pub fn end_screen(props: &EndScreenProps) -> Html {
    let state = use_context::<GameContext>().expect("GameContext not found");

    let on_restart = {
        let state = state.clone();
        Callback::from(move |_: MouseEvent| {
            state.dispatch(GameAction::GoToTitle);
        })
    };

    let (title, subtitle, story, color) = if props.game_over {
        (
            "☠ GAME OVER",
            "Nyawamu Habis",
            vec![
                format!("Detektif {}... kamu sudah berjuang keras.", state.player_name),
                "Tapi jebakan-jebakan analog itu terlalu kejam. Tubuhmu tak sanggup lagi.".to_string(),
                "Kasus Cetak Biru Berdarah ini... akan terkubur bersama rahasia Profesor Hartono.".to_string(),
                format!("Kamu berhasil sampai Chapter {} dari 30.", state.current_chapter),
                format!("Skor akhir: {}/30 puzzle terjawab benar.", state.score),
            ],
            "#cc4444",
        )
    } else {
        match state.ending_type() {
            EndingType::Perfect => (
                "🏆 ENDING SEMPURNA",
                "Sang Detektif Legendaris",
                vec![
                    format!("Detektif {}... kamu luar biasa.", state.player_name),
                    "Komisaris Surya sudah ditangkap. Lingkaran Merah dibubarkan. Semua bukti tersegel.".to_string(),
                    "Tapi yang paling mencengangkan — kamu memecahkan hampir semua teka-teki tanpa bantuan.".to_string(),
                    "Kota ini berhutang besar padamu. Nama kamu akan diukir di dinding Balai Kota Semarang.".to_string(),
                    format!("Skor sempurna: {}/30. Kamu pantas disebut detektif terhebat.", state.score),
                ],
                "#c68642",
            ),
            EndingType::Good => (
                "✨ ENDING BAIK",
                "Kebenaran Terungkap",
                vec![
                    format!("Kerja bagus, Detektif {}.", state.player_name),
                    "Komisaris Surya berhasil ditangkap dan Lingkaran Merah dibongkar.".to_string(),
                    "Meski beberapa jebakan berhasil melukaimu, kamu tetap berdiri tegak di akhir.".to_string(),
                    "Profesor Hartono... akhirnya bisa beristirahat dengan tenang.".to_string(),
                    format!("Skor: {}/30. Cukup mengesankan untuk seorang detektif muda.", state.score),
                ],
                "#6daa6d",
            ),
            EndingType::Bad => (
                "💀 ENDING PAHIT",
                "Setengah Kebenaran",
                vec![
                    format!("Detektif {}... kamu selamat, tapi dengan harga yang mahal.", state.player_name),
                    "Komisaris Surya berhasil menghancurkan sebagian besar bukti sebelum tertangkap.".to_string(),
                    "Dia hanya dihukum ringan. Lingkaran Merah bubar, tapi orang-orangnya tersebar ke mana-mana.".to_string(),
                    "Kasus ini... belum benar-benar selesai.".to_string(),
                    format!("Skor: {}/30. Masih banyak yang harus kamu pelajari.", state.score),
                ],
                "#8a7560",
            ),
            EndingType::Death => (
                "☠ GAME OVER",
                "Terlambat",
                vec!["Kamu tidak seharusnya sampai di sini dengan ending ini.".to_string()],
                "#cc4444",
            ),
        }
    };

    html! {
        <div style={styles::END_SCREEN_STYLE}>
            <div style={format!(
                "font-family: 'Playfair Display', Georgia, serif; \
                font-size: clamp(2rem, 5vw, 3.5rem); \
                font-weight: 900; \
                color: {}; \
                text-shadow: 0 0 30px {}40; \
                margin-bottom: 10px; \
                animation: flickerGlow 3s ease-in-out infinite;",
                color, color
            )}>
                { title }
            </div>

            <p style="\
                font-family: 'Crimson Text', Georgia, serif; \
                font-size: 1.3rem; \
                color: #8b4513; \
                font-style: italic; \
                margin-bottom: 40px;\
            ">
                { subtitle }
            </p>

            <div style={styles::DIVIDER_STYLE}></div>

            <div style="max-width: 600px; margin: 0 auto;">
                { for story.iter().map(|p| {
                    html! {
                        <p style={format!("{} text-align: center;", styles::NARRATIVE_PARAGRAPH_STYLE)}>
                            { p }
                        </p>
                    }
                })}
            </div>

            <div style={format!("{} margin-top: 20px;", styles::DIVIDER_STYLE)}></div>

            <button
                style={styles::BUTTON_PRIMARY_STYLE}
                onclick={on_restart}
            >
                { "🔄 Main Lagi dari Awal" }
            </button>
        </div>
    }
}
