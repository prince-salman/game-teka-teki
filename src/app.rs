use yew::prelude::*;

use crate::components::end_screen::EndScreen;
use crate::components::game_screen::GameScreen;
use crate::components::name_input::NameInput;
use crate::components::title_screen::TitleScreen;
use crate::state::actions::GameContext;
use crate::state::game_state::*;
use crate::styles;

#[function_component(App)]
pub fn app() -> Html {
    let state = use_reducer(GameState::default);

    html! {
        <ContextProvider<GameContext> context={state.clone()}>
            <style>{ styles::GLOBAL_STYLES }</style>
            <div style="min-height: 100vh;">
                {
                    match state.phase {
                        GamePhase::TitleScreen => html! { <TitleScreen /> },
                        GamePhase::NameInput => html! { <NameInput /> },
                        GamePhase::Playing => html! { <GameScreen /> },
                        GamePhase::GameOver => html! { <EndScreen game_over={true} /> },
                        GamePhase::Victory => html! { <EndScreen game_over={false} /> },
                    }
                }
            </div>
        </ContextProvider<GameContext>>
    }
}
