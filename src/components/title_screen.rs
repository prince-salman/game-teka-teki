use yew::prelude::*;

use crate::state::actions::{GameAction, GameContext};
use crate::state::persistence;
use crate::styles;

#[function_component(TitleScreen)]
pub fn title_screen() -> Html {
    let state = use_context::<GameContext>().expect("GameContext not found");

    let on_new_game = {
        let state = state.clone();
        Callback::from(move |_: MouseEvent| {
            state.dispatch(GameAction::StartNewGame);
        })
    };

    let on_continue = {
        let state = state.clone();
        Callback::from(move |_: MouseEvent| {
            state.dispatch(GameAction::LoadGame);
        })
    };

    let has_save = persistence::has_save();

    html! {
        <div style={styles::TITLE_SCREEN_STYLE}>
            // Ornamen atas
            <div style="font-size: 2rem; margin-bottom: 20px; opacity: 0.4; letter-spacing: 10px;">
                { "⚙ ✦ ⚙" }
            </div>

            // Judul utama
            <h1 style={styles::GAME_TITLE_STYLE}>
                { "Cetak Biru" }
                <br />
                { "Berdarah" }
            </h1>

            // Subtitle
            <p style={styles::SUBTITLE_STYLE}>
                { "The Crimson Blueprint" }
            </p>

            // Divider
            <div style={styles::DIVIDER_STYLE}></div>

            // Tagline
            <p style="\
                font-family: 'Crimson Text', Georgia, serif; \
                font-size: 1.05rem; \
                color: #8a7560; \
                max-width: 500px; \
                margin-bottom: 40px; \
                font-style: italic; \
                line-height: 1.6; \
                animation: fadeIn 1.5s ease-out;\
            ">
                { "Setiap angka punya cerita. Setiap jawaban membuka pintu. " }
                { "Tapi hati-hati — di balik cetak biru ini, tersembunyi darah yang belum kering." }
            </p>

            // Buttons
            <div style="display: flex; flex-direction: column; align-items: center; gap: 12px;">
                <button
                    style={styles::BUTTON_PRIMARY_STYLE}
                    onclick={on_new_game}
                >
                    { "⚔ Mulai Petualangan Baru" }
                </button>

                if has_save {
                    <button
                        style={styles::BUTTON_SECONDARY_STYLE}
                        onclick={on_continue}
                    >
                        { "📜 Lanjutkan Petualangan" }
                    </button>
                }
            </div>

            // Footer info
            <div style="\
                margin-top: 60px; \
                font-family: 'Courier Prime', monospace; \
                font-size: 0.75rem; \
                color: #5a4535; \
                letter-spacing: 2px;\
            ">
                { "30 CHAPTER · TEKA-TEKI MATEMATIKA · PETUALANGAN ANALOG" }
            </div>

            // Ornamen bawah
            <div style="font-size: 1.5rem; margin-top: 20px; opacity: 0.3; letter-spacing: 10px;">
                { "✦ ⚙ ✦" }
            </div>
        </div>
    }
}
