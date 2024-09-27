use anathema::{
    component::{Component, KeyCode},
    state::{State, Value},
};

pub(crate) struct NextItemComponent;

impl NextItemComponent {}

impl Component for NextItemComponent {
    type State = NextItemComponentState;
    type Message = ();

    fn on_key(
        &mut self,
        key: anathema::component::KeyEvent,
        state: &mut Self::State,
        mut _elements: anathema::widgets::Elements<'_, '_>,
        mut _context: anathema::prelude::Context<'_, Self::State>,
    ) {
        if let KeyCode::Enter = key.code {
            let now = state.display.to_bool();
            *state.display.to_mut() = !now;
        }
    }
}

#[derive(State)]
pub(crate) struct NextItemComponentState {
    display: Value<bool>,
}

impl NextItemComponentState {
    pub(crate) fn new() -> Self {
        Self {
            display: Value::new(false),
        }
    }
}
