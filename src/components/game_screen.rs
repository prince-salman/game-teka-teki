use yew::prelude::*;

use crate::chapters::get_chapter;
use crate::components::chapter_header::ChapterHeader;
use crate::components::inventory::Inventory;
use crate::components::narrative::Narrative;
use crate::components::puzzle::Puzzle;
use crate::state::actions::{GameAction, GameContext};
use crate::state::game_state::ChapterPhase;
use crate::styles;

#[function_component(GameScreen)]
pub fn game_screen() -> Html {
    let state = use_context::<GameContext>().expect("GameContext not found");
    let chapter_data = get_chapter(state.current_chapter);

    let bg_img = if state.current_chapter <= 10 {
        "/assets/arc1.png"
    } else if state.current_chapter <= 20 {
        "/assets/arc2.png"
    } else {
        "/assets/arc3.png"
    };

    html! {
        <div class="game-layout">
            // Panel utama (kiri)
            <div style={styles::MAIN_PANEL_STYLE}>
                <ChapterHeader />

                <div style="width: 100%; height: 300px; margin-bottom: 24px; border-radius: 8px; overflow: hidden; border: 1px solid rgba(139, 69, 19, 0.3); display: flex; justify-content: center; align-items: center; background-color: #0d0806;">
                    <img src={bg_img} style="max-width: 100%; height: 100%; object-fit: contain; border-radius: 8px;" />
                </div>

                {
                    match state.chapter_phase {
                        ChapterPhase::Intro => {
                            html! {
                                <Narrative
                                    paragraphs={chapter_data.narrative_intro.iter().map(|s| s.to_string()).collect::<Vec<_>>()}
                                    button_text={"🔍 Lanjut ke Teka-Teki"}
                                    on_continue={
                                        let s = state.clone();
                                        Callback::from(move |_: MouseEvent| {
                                            s.dispatch(GameAction::SetChapterPhase(ChapterPhase::Puzzle));
                                        })
                                    }
                                />
                            }
                        }
                        ChapterPhase::Puzzle | ChapterPhase::Failure => {
                            html! { <Puzzle /> }
                        }
                        ChapterPhase::Success => {
                            html! {
                                <Narrative
                                    paragraphs={chapter_data.narrative_success.iter().map(|s| s.to_string()).collect::<Vec<_>>()}
                                    button_text={
                                        if state.current_chapter >= 30 {
                                            "🏆 Selesaikan Petualangan"
                                        } else {
                                            "📖 Lanjut ke Chapter Berikutnya"
                                        }
                                    }
                                    on_continue={
                                        let s = state.clone();
                                        Callback::from(move |_: MouseEvent| {
                                            s.dispatch(GameAction::NextChapter);
                                        })
                                    }
                                />
                            }
                        }
                        ChapterPhase::Transition => {
                            html! {
                                <div style="text-align: center; padding: 40px;">
                                    <p style={styles::NARRATIVE_PARAGRAPH_STYLE}>
                                        { "Memuat chapter berikutnya..." }
                                    </p>
                                </div>
                            }
                        }
                    }
                }
            </div>

            // Panel samping (kanan) — inventory & stats
            <div class="side-panel">
                <Inventory />
            </div>
        </div>
    }
}
