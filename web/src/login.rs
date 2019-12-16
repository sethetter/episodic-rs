use yew::{html, Component, ComponentLink, Html, ShouldRender};

pub struct Login {
    phone: String,
    verify_code: String,

    show_verify: bool,
}

pub enum Msg {
    Submit,
    Verify
}

impl Component for Login {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Login {
            phone: "".to_string(),
            verify_code: "".to_string(),
            show_verify: false,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Submit => {
                self.show_verify = true;
                true // Indicate that the Component should re-render
            },
            Msg::Verify => {
                true // Indicate that the Component should re-render
            },
        }
    }

    fn view(&self) -> Html<Self> {
        html! {
            <>
                { self.view_phone_input() }
                { self.view_verify_input() }
            </>
        }
    }
}

impl Login {
    fn view_phone_input(&self) -> Html<Self> {
        html! {
            <>
                <input type="phone",
                    name="phone",
                    placeholder="Phone Number"
                    value=&self.phone />

                <button onclick=|_| Msg::Submit>{ "Login" }</button>
            </>
        }
    }

    fn view_verify_input(&self) -> Html<Self> {
        if self.show_verify {
            html! {
                <>
                    <input type="number"
                        name="verify_code"
                        value=&self.verify_code />
                    <button onclick=|_| Msg::Verify>{ "Verify" }</button>
                </>
            }
        } else {
            html! {}
        }
    }
}
