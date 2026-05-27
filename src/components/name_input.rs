use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;

use crate::state::actions::{GameAction, GameContext};
use crate::styles;

#[function_component(NameInput)]
pub fn name_input() -> Html {
    let state = use_context::<GameContext>().expect("GameContext not found");
    let input_ref = use_node_ref();

    let on_submit = {
        let state = state.clone();
        let input_ref = input_ref.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            if let Some(input) = input_ref.cast::<HtmlInputElement>() {
                let name = input.value();
                if !name.trim().is_empty() {
                    state.dispatch(GameAction::SetPlayerName(name));
                }
            }
        })
    };

    html! {
        <div style={styles::NAME_INPUT_CONTAINER}>
            // Ornamen
            <div style="font-size: 1.5rem; margin-bottom: 30px; opacity: 0.5; letter-spacing: 8px;">
                { "📜 ✦ 📜" }
            </div>

            <h2 style={styles::NAME_PROMPT_STYLE}>
                { "Siapa namamu, Detektif?" }
            </h2>

            <p style="\
                font-family: 'Crimson Text', Georgia, serif; \
                font-size: 1rem; \
                color: #8a7560; \
                font-style: italic; \
                margin-bottom: 30px; \
                max-width: 450px;\
            ">
                { "Tuliskan namamu di buku catatan ini. " }
                { "Mulai sekarang, kamu yang akan mengungkap kebenaran di balik cetak biru berdarah ini." }
            </p>

            <form onsubmit={on_submit} style={styles::FORM_STYLE}>
                <input
                    ref={input_ref}
                    type="text"
                    placeholder="Ketik namamu..."
                    style={styles::INPUT_STYLE}
                    maxlength="30"
                    autofocus=true
                />
                <button type="submit" style={styles::BUTTON_PRIMARY_STYLE}>
                    { "🔍 Mulai Investigasi" }
                </button>
            </form>

            <div style={styles::DIVIDER_STYLE}></div>

            <p style="\
                font-family: 'Courier Prime', monospace; \
                font-size: 0.75rem; \
                color: #5a4535; \
                letter-spacing: 1px;\
            ">
                { "KASUS #1987-CBB · PRIORITAS TINGGI · RAHASIA" }
            </p>
        </div>
    }
}
