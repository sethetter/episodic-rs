use yew::{html, Component, ComponentLink, Html, ShouldRender, Properties};
use serde::Deserialize;

use crate::login::Login;

pub struct App {
    link: ComponentLink<Self>,
    user: Option<User>,
}

#[derive(Clone, Properties, Deserialize)]
pub struct User {
    pub id: u32,
    pub name: String,
}

pub enum Msg {
    Login(User),
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        App { user: None, link }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Login(user) => {
                self.user = Some(user);
                true
            }
        }
    }

    fn view(&self) -> Html {
        let on_login = self.link.callback(Msg::Login);
        match &self.user {
            Some(user) => html! { <h1>{ format!("Hello, {}!", user.name) }</h1> },
            None => html! {
                <Login on_login=on_login />
            },
        }
    }
}
