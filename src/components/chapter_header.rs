use yew::prelude::*;

use crate::chapters::{get_chapter, TOTAL_CHAPTERS};
use crate::state::actions::GameContext;
use crate::styles;

#[function_component(ChapterHeader)]
pub fn chapter_header() -> Html {
    let state = use_context::<GameContext>().expect("GameContext not found");
    let chapter = get_chapter(state.current_chapter);
    let progress_pct = (state.current_chapter as f64 / TOTAL_CHAPTERS as f64) * 100.0;

    html! {
        <div style={styles::CHAPTER_HEADER_STYLE}>
            <div style="display: flex; justify-content: space-between; align-items: flex-start;">
                <div>
                    <p style={styles::CHAPTER_NUMBER_STYLE}>
                        { format!("Chapter {}", chapter.number) }
                    </p>
                    <h2 style={styles::CHAPTER_TITLE_STYLE}>
                        { chapter.title }
                    </h2>
                    <span style={format!("{} {}", styles::MATH_LEVEL_BADGE, chapter.math_level.badge_color())}>
                        { chapter.math_level.label() }
                    </span>
                </div>
                <div style="text-align: right;">
                    <p style={styles::STAT_LABEL_STYLE}>{ "Skor" }</p>
                    <p style={styles::STAT_VALUE_STYLE}>{ format!("{}/{}", state.score, TOTAL_CHAPTERS) }</p>
                </div>
            </div>

            // Progress bar
            <div style={styles::PROGRESS_BAR_CONTAINER}>
                <div style={format!("{} width: {:.1}%;", styles::PROGRESS_BAR_FILL, progress_pct)}></div>
            </div>
        </div>
    }
}
