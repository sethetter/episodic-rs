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
        let phone_block = move || -> Html<Login> {
            html! {
                <>
                    <input type="phone",
                        name="phone",
                        placeholder="Phone Number"
                        value=&self.phone />

                    <button onclick=|_| Msg::Submit>{ "Login" }</button>
                </>
            }
        };

        let verify_block = move || -> Html<Login> {
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
        };

        html! {
            <>
                { phone_block() }
                { verify_block() }
            </>
        }
    }
}
