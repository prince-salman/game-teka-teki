use yew::prelude::*;

use crate::styles;

#[derive(Properties, PartialEq)]
pub struct NarrativeProps {
    pub paragraphs: Vec<String>,
    pub button_text: &'static str,
    pub on_continue: Callback<MouseEvent>,
}

#[function_component(Narrative)]
pub fn narrative(props: &NarrativeProps) -> Html {
    html! {
        <div style="animation: fadeIn 0.6s ease-out;">
            { for props.paragraphs.iter().enumerate().map(|(i, p)| {
                let delay = format!(
                    "{} animation-delay: {}s; animation-fill-mode: both;",
                    styles::NARRATIVE_PARAGRAPH_STYLE,
                    i as f32 * 0.15
                );
                html! {
                    <p style={delay} key={i}>
                        { p }
                    </p>
                }
            })}

            <div style="text-align: center; margin-top: 32px;">
                <button
                    style={styles::BUTTON_PRIMARY_STYLE}
                    onclick={props.on_continue.clone()}
                >
                    { props.button_text }
                </button>
            </div>
        </div>
    }
}
