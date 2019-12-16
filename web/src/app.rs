use yew::{html, Component, ComponentLink, Html, ShouldRender};

use crate::login::Login;

pub struct App {
    user: Option<User>,
}

pub struct User {
    id: String,
}

pub enum Msg {}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        App { user: None }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn view(&self) -> Html<Self> {
        html! {
            <Login />
        }
    }
}
