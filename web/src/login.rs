use {
    crate::app,
    serde::Deserialize,
    serde_json::json,
    yew::{
        format::json::Json,
        html,
        prelude::*,
        services::fetch::{FetchService, FetchTask, Request, Response},
        Component, ComponentLink, Html, InputData, Properties, ShouldRender,
    },
};

#[derive(Clone, Properties)]
pub struct LoginProps {
    #[props(required)]
    pub on_login: Callback<app::User>,
}

pub struct Login {
    link: ComponentLink<Self>,
    props: LoginProps,

    phone: String,
    verify: String,
    user: Option<app::User>,

    loading: bool,
    show_verify: bool,

    web: FetchService,
    fetch_task: Option<FetchTask>,
}

pub enum Msg {
    PhoneInput(String),
    VerifyInput(String),

    Login,
    Verify,

    LoginRespOk(LoginResponse),
    LoginRespErr,

    VerifyRespOk,
    VerifyRespErr,
}

#[derive(Deserialize)]
pub struct LoginResponse {
    user: app::User,
}

impl Component for Login {
    type Message = Msg;
    type Properties = LoginProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Login {
            props,
            link,

            phone: "".to_string(),
            verify: "".to_string(),
            user: None,
            loading: false,
            show_verify: false,
            web: FetchService::new(),
            fetch_task: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::PhoneInput(v) => {
                self.phone = v;
                true
            }
            Msg::VerifyInput(v) => {
                self.verify = v;
                true
            }
            Msg::Login => {
                // POST self.phone to /login
                // Process response with Msg::LoginResponse
                self.loading = true;

                let body = &json!({ "phone": self.phone });
                let req = Request::post("/api/login")
                    .header("content-type", "application/json")
                    .body(Json(body))
                    .expect("Failed to build request");

                let task = self.web.fetch(
                    req,
                    self.link.callback(
                        move |resp: Response<Json<Result<LoginResponse, failure::Error>>>| {
                            if let (meta, Json(Ok(body))) = resp.into_parts() {
                                if meta.status.is_success() {
                                    return Msg::LoginRespOk(body);
                                }
                            }
                            Msg::LoginRespErr
                        },
                    ),
                );
                self.fetch_task = Some(task);
                true
            }
            Msg::Verify => {
                self.loading = true;

                let body = &json!({
                    "user_id": self.user.clone().unwrap().id,
                    "verify_code": self.verify
                });
                let req = Request::post("/api/login/verify")
                    .header("content-type", "application/json")
                    .body(Json(body))
                    .expect("Failed to build request");

                let task = self.web.fetch(
                    req,
                    self.link.callback(
                        move |resp: Response<Json<Result<LoginResponse, failure::Error>>>| {
                            if resp.status().is_success() {
                                return Msg::VerifyRespOk;
                            }
                            Msg::LoginRespErr
                        },
                    ),
                );
                self.fetch_task = Some(task);
                true
            }
            Msg::LoginRespOk(resp) => {
                self.loading = false;
                self.user = Some(resp.user);
                self.show_verify = true;
                true
            }
            Msg::LoginRespErr => {
                self.loading = false;
                // self.error_msg = "An error occurred!";
                true
            }
            Msg::VerifyRespOk => {
                // Send msg to parent component to set state to logged in
                // and store user / session token in local storage.
                self.loading = false;
                self.props.on_login.emit(self.user.clone().unwrap());
                true
            }
            Msg::VerifyRespErr => {
                self.loading = false;
                // self.error_msg = "An error occurred!";
                true
            }
        }
    }

    fn view(&self) -> Html {
        html! {
            <>
                { self.view_phone_input() }
                { self.view_verify_input() }
            </>
        }
    }
}

impl Login {
    fn view_phone_input(&self) -> Html {
        html! {
            <>
                <input type="phone"
                    name="phone"
                    placeholder="Phone Number"
                    oninput=self.link.callback(|e: InputData| {
                        Msg::PhoneInput(e.value)
                    }) />
                <button onclick=self.link.callback(|_| Msg::Login)>
                    { "Login" }
                </button>
            </>
        }
    }

    fn view_verify_input(&self) -> Html {
        if self.show_verify {
            html! {
                <>
                    <input type="number"
                        name="verify"
                        oninput=self.link.callback(|e: InputData| {
                            Msg::VerifyInput(e.value)
                        }) />
                    <button onclick=self.link.callback(|_| Msg::Verify)>
                        { "Verify" }
                    </button>
                </>
            }
        } else {
            html! {}
        }
    }
}
