use anathema::{component::Component, state::State};

pub(crate) struct ItemComponent;

impl ItemComponent {}

impl Component for ItemComponent {
    type State = ();
    type Message = ();
}
