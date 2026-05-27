use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;

use crate::chapters::get_chapter;
use crate::state::actions::{GameAction, GameContext};
use crate::state::game_state::ChapterPhase;
use crate::styles;

#[function_component(Puzzle)]
pub fn puzzle() -> Html {
    let state = use_context::<GameContext>().expect("GameContext not found");
    let chapter = get_chapter(state.current_chapter);
    let input_ref = use_node_ref();

    let on_submit = {
        let state = state.clone();
        let input_ref = input_ref.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            if let Some(input) = input_ref.cast::<HtmlInputElement>() {
                let answer = input.value();
                if !answer.trim().is_empty() {
                    state.dispatch(GameAction::SubmitAnswer(answer));
                    input.set_value("");
                }
            }
        })
    };

    let on_retry = {
        let state = state.clone();
        Callback::from(move |_: MouseEvent| {
            state.dispatch(GameAction::SetChapterPhase(ChapterPhase::Puzzle));
        })
    };

    let on_hint = {
        let state = state.clone();
        Callback::from(move |_: MouseEvent| {
            state.dispatch(GameAction::ShowHint);
        })
    };

    let is_failure = state.chapter_phase == ChapterPhase::Failure;

    html! {
        <div style="animation: fadeIn 0.5s ease-out;">
            // Konteks cerita puzzle
            <p style={styles::PUZZLE_CONTEXT_STYLE}>
                { chapter.puzzle_context }
            </p>

            // Container puzzle
            <div style={styles::PUZZLE_CONTAINER_STYLE}>
                // Pertanyaan
                <p style={styles::PUZZLE_QUESTION_STYLE}>
                    { "📐 " }{ chapter.puzzle_question }
                </p>

                // Feedback salah
                if is_failure {
                    <div style={styles::FAILURE_STYLE}>
                        { "❌ " }{ chapter.narrative_failure }
                        if !state.last_wrong_answer.is_empty() {
                            <span style="opacity: 0.7; margin-left: 8px;">
                                { format!("(Jawabanmu: {})", state.last_wrong_answer) }
                            </span>
                        }
                    </div>
                }

                // Hint
                if state.showing_hint {
                    <div style={styles::HINT_STYLE}>
                        { "💡 Petunjuk: " }{ chapter.puzzle_hint }
                    </div>
                }

                // Info percobaan
                if state.wrong_attempts > 0 {
                    <p style="\
                        font-family: 'Courier Prime', monospace; \
                        font-size: 0.8rem; \
                        color: #8a7560; \
                        margin-top: 12px;\
                    ">
                        { format!("Percobaan salah: {} kali", state.wrong_attempts) }
                    </p>
                }

                // Form input jawaban
                <form onsubmit={on_submit} style="\
                    display: flex; \
                    flex-direction: column; \
                    align-items: center; \
                    gap: 16px; \
                    margin-top: 24px;\
                ">
                    <input
                        ref={input_ref}
                        type="text"
                        placeholder="Masukkan jawabanmu..."
                        style={styles::INPUT_STYLE}
                        autofocus=true
                    />

                    <div style="display: flex; gap: 12px; flex-wrap: wrap; justify-content: center;">
                        <button type="submit" style={styles::BUTTON_PRIMARY_STYLE}>
                            { "✓ Kirim Jawaban" }
                        </button>

                        if is_failure {
                            <button
                                type="button"
                                style={styles::BUTTON_SECONDARY_STYLE}
                                onclick={on_retry}
                            >
                                { "🔄 Coba Lagi" }
                            </button>
                        }

                        if !state.showing_hint && state.hints_remaining > 0 {
                            <button
                                type="button"
                                style={styles::BUTTON_SECONDARY_STYLE}
                                onclick={on_hint}
                            >
                                { format!("💡 Gunakan Hint (Sisa: {})", state.hints_remaining) }
                            </button>
                        }
                    </div>
                </form>
            </div>
        </div>
    }
}
