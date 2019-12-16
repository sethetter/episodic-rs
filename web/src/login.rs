use yew::{html, Component, ComponentLink, Html, ShouldRender};

pub struct Login {
    phone: String,
}

pub enum Msg {
    Submit,
}

impl Component for Login {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Login { phone: "".to_string() }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Submit => {
                true // Indicate that the Component should re-render
            }
        }
    }

    fn view(&self) -> Html<Self> {
        html! {
            <div>
                <input type="phone",
                    name="phone",
                    placeholder="Phone Number"
                    value=&self.phone />

                <button onclick=|_| Msg::Submit>{ "Login" }</button>
            </div>
        }
    }
}
