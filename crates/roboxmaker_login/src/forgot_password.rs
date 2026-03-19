use log::*;
use regex::Regex;
use yew::{prelude::*, web_sys};
use wasm_bindgen_futures::spawn_local;
use yew::{html, Component, ComponentLink, Html, ShouldRender};

use roboxmaker_main::lang;
use roboxmaker_models::registration::{self, credential_reset};

use crate::login::PageMode;

pub struct ForgotPage {
    link: ComponentLink<Self>,
    props: Properties,
    page_mode: PageModeForgotPassword,
    reset_crendential_with: bool,
    validate_username: bool,
    validate_email: bool,
    email: String,
    username: String,
    password: String,
    recover_password_status: i32,
    class_forgot: Vec<(bool, String)>,
    node_options: NodeRef,
    forgot_action: bool, 
}

#[derive(Properties, Clone)]
pub struct Properties {
    pub on_page_mode: Callback<PageMode>,
}

#[derive(Debug)]
pub enum PageModeForgotPassword {
    ForgotPassword,
    ResendForgotPassword,
}

#[derive(Debug)]
pub enum Message {
    ChangePageMode(PageMode),
    PageMode(PageModeForgotPassword),
    ResetActionWith(bool),
    UpdateEmail(String),
    UpdateUsername(String),
    ResetCredentialAction,
    ResetCredential(Option<registration::credentials_reset_action::ResponseData>),
    OptionChange,
}

impl Component for ForgotPage {
    type Message = Message;
    type Properties = Properties;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        ForgotPage {
            link,
            props,
            page_mode: PageModeForgotPassword::ForgotPassword,
            reset_crendential_with: false,
            validate_username: false,
            validate_email: false,
            email: "".to_string(),
            username: "".to_string(),
            password: "".to_string(),
            recover_password_status: 0,
            class_forgot: vec! [
                (true, "Verificar Email".to_string()),
                (false, "Actualizar Contraseña".to_string()),
            ],
            node_options: NodeRef::default(),
            forgot_action: true,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        info!("{:?}", msg);
        let mut should_render = false;
        match msg {
            Message::ChangePageMode(page_mode) => {
                self.props.on_page_mode.emit(page_mode)
            }
            Message::PageMode(mode) => {
                self.page_mode = mode;
                self.recover_password_status = 0;
                match &self.page_mode {
                    PageModeForgotPassword::ForgotPassword => {
                        self.email = "".to_string();
                        self.password = "".to_string();
                    }
                    PageModeForgotPassword::ResendForgotPassword => {
                        self.email = "".to_string();
                        self.password = "".to_string();
                    }
                }
                should_render = true;
            }
            Message::UpdateEmail(email) => {
                let email_regex = Regex::new(r"^([a-z0-9_+]([a-z0-9_+.]*[a-z0-9_+])?)@([a-z0-9]+([\-\.]{1}[a-z0-9]+)*\.[a-z]{2,6})").unwrap();
                if email_regex.is_match(&email) {
                    self.validate_email = true;
                } else {
                    self.validate_email = false;
                }
                self.email = email;
                should_render = true;
            }
            Message::UpdateUsername(username) => {
                let username_regex = Regex::new(r"^[A-z0-9_-]{8,20}(\.?[a-z0-9])*$").unwrap();
                if username_regex.is_match(&username) {
                    self.validate_username = true;
                } else {
                    self.validate_username = false;
                }
                self.username = username;
                should_render = true;
            }
            Message::ResetActionWith(value) => {
                self.reset_crendential_with = value;
                if value {
                    self.username = "".to_string();
                } else {
                    self.email = "".to_string();
                }
                should_render = true;
            }
            Message::ResetCredentialAction => {
                let action = if self.forgot_action {
                    "VERIFY_EMAIL".to_string()
                } else {
                    "UPDATE_PASSWORD".to_string()
                };
                self.recover_password_status = 102;

                let vars = registration::credentials_reset_action::Variables {
                    email: Some(self.email.clone()),
                    username: Some(self.username.clone()),
                    reset_action: action,
                };
                let link = self.link.clone();
                spawn_local(async move {
                    let response = credential_reset(vars).await;
                    match response {
                        Ok(data) => {
                            link.send_message(Message::ResetCredential(data))
                        },
                        Err(_e) => {},
                    }
                });
                should_render = true;
            }
            Message::ResetCredential(_response) => {
                self.page_mode = PageModeForgotPassword::ResendForgotPassword;
                if _response.is_some() {
                    self.recover_password_status = 201;
                } else {
                    self.recover_password_status = 404;
                }
                should_render = true;
            }
            Message::OptionChange => {
                let forgot = self
                    .node_options
                    .cast::<web_sys::HtmlSelectElement>()
                    .unwrap();
                self.forgot_action = serde_json::from_str(&forgot.value()).unwrap_or(true);
            }
        }
        should_render
    }
    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }
    fn view(&self) -> Html {
        let on_change_forgot_option = self.link.callback(|_: ChangeData| Message::OptionChange);
        let on_update_email_recover = self.link.callback(|e: InputData| Message::UpdateEmail(e.value.trim().to_string()));
        let on_update_username_recover = self.link.callback(|e: InputData| Message::UpdateUsername(e.value.trim().to_string()));
        let on_login_page_mode = self.link.callback(move |_| Message::ChangePageMode(PageMode::Login));
        let on_change_forgot = self.link.callback(|_: MouseEvent| Message::ResetCredentialAction);
        let on_recovery_password_user_name = self.link.callback(|_: ChangeData| Message::ResetActionWith(false));
        let on_recovery_password_email = self.link.callback(|_: ChangeData| Message::ResetActionWith(true));
        let forgot_list_options = self
            .class_forgot
            .iter()
            .map(|option| {
                let option_id = format!("{:?}", option.0);
                html! {
                        <option value=option_id>{&option.1}</option>
                }
            })
            .collect::<Html>();

        let notifications = match self.page_mode {
            PageModeForgotPassword::ForgotPassword => {
                html! {
                    if self.recover_password_status == 404 && !self.reset_crendential_with {
                        html! {
                            <span class="text-danger bg-white w-100 rounded-3 p-3">
                                {lang::dict("Username Incorrect")}
                            </span>
                        }
                    } else if self.recover_password_status == 404 && self.reset_crendential_with {
                        html! {
                            <span class="text-danger bg-white w-100 rounded-3 p-3">
                                {lang::dict("Email Incorrect")}
                            </span>
                        }
                    } else if self.recover_password_status == 201 {
                        html! {
                            <span class="text-warning bg-white w-100 rounded-3 p-3">
                                {lang::dict("Action Sent")}
                                <br/>
                                {lang::dict("Email to request action sent")}
                            </span>
                        }
                    } else if self.recover_password_status == 102 {
                        html! {
                            <span class="text-warning bg-white w-100 rounded-3 p-3">
                                {lang::dict("Processing...")}
                            </span>
                        }
                    } else {
                        html! {}
                    }
                }
            }
            PageModeForgotPassword::ResendForgotPassword => {
                html! {
                    if self.recover_password_status == 201 {
                        html! {
                            <span class="text-warning bg-white w-100 rounded-3 p-3">
                                {lang::dict("Action Sent")}
                                <br/>
                                {lang::dict("Email to request action sent")}
                            </span>
                        }
                    } else if self.recover_password_status == 102 {
                        html! {
                            <span class="text-warning bg-white w-100 rounded-3 p-3">
                                {lang::dict("Processing...")}
                            </span>
                        }
                    } else {
                        html! {}
                    }
                }
            }
        };

        let select_option_forgot = html! {
            <div class="mt-2">
                <label class="text-white noir-bold is-size-16 lh-20 my-2">{lang::dict("What do you want to do?")}</label>
                <div class="control has-icons-left d-flex align-items-center">
                    <div class="select" style="width: 100%; height: 50px;">
                        <select ref=self.node_options.clone() onchange=on_change_forgot_option class="input-style-l is-fullwidth" style="width: 100%; border-color: #DBDBFF; color: #BCBDFD; top: auto;">
                            {forgot_list_options}
                        </select>
                    </div>
                </div>
            </div>
        };
        let recover_password_field = if !self.reset_crendential_with {
            html! {
                <div class="mt-3">
                    <label class="form-label text-white noir-bold is-size-16 lh-20">{lang::dict("Fill in the following field")}</label>
                    <div class="input-group">
                        <span class="input-group-text">
                            <i class="far fa-user"></i>
                        </span>
                        <input type="text" class="form-control input-style-l" placeholder="alexahrdz01" oninput=&on_update_username_recover required=true />
                    </div>
                </div>
            }
        } else {
            html! {
                <div class="mt-3">
                    <label class="form-label text-white noir-bold is-size-16 lh-20">{lang::dict("Fill in the following field")}</label>
                    <div class="input-group">
                        <span class="input-group-text">
                            <i class="far fa-envelope"></i>
                        </span>
                        <input type="email" class="form-control input-style-l" placeholder="alexa@example.com" oninput=&on_update_email_recover required=true />
                    </div>
                </div>
            }
        };
        let page_mode = match self.page_mode {
            PageModeForgotPassword::ForgotPassword => {
                html! {
                    <div class="card bg-dark text-white h-100 w-100">
                        <img src="/static/new-arte.png" class="card-img" alt="..." style="height: 100vh; width: 100vw; object-fit: cover;" />
                        <div class="card-img-overlay">
                            <div class="col-sm-12 col-md-12 col-lg-6 col-xl-5 col-xxl-4 bg-section-login pd-sm-4 p-md-7 p-sm-6 p-4 d-flex flex-column justify-content-between h-85 h-sm-100">
                                <div class="d-flex justify-content-between">
                                    <div class="col-4">
                                        <img src="/static/logo-robox-maker.png" class="img-fluid" />
                                    </div>
                                    <a class="text-white noir-bold is-size-6 lh-19" onclick=&on_login_page_mode>
                                        <i class="fas fa-arrow-left mx-2"></i>
                                        <span>{lang::dict("Behind")}</span>
                                    </a>
                                </div>
                                <h1 class="text-white noir-bold is-size-48 lh-58">{lang::dict("Forgetting Information")}</h1>
                                <span class="text-white noir-light text-center is-size-18 lh-22">{lang::dict("An email will be sent to the email registered on the platform")}</span>
                                <div class="d-flex flex-column">
                                    <label class="form-label text-white noir-bold is-size-16 lh-20">{lang::dict("What do you remember about his account?")}</label>
                                    <div class="d-flex justify-content-between">
                                        <label class="radio">
                                            <input type="radio" id="radio-username" name="answer" value="username"
                                                checked=!self.reset_crendential_with
                                                onchange=&on_recovery_password_user_name />
                                            <span class="text-white noir-light is-size-18 lh-22 ps-3 d-flex align-items-center p-0 mb-0">{lang::dict("Username")}</span>
                                        </label>
                                        <label class="radio">
                                            <input type="radio" id="radio-email" name="answer" value="email"
                                                checked=self.reset_crendential_with
                                                onchange=&on_recovery_password_email />
                                            <span class="text-white noir-light is-size-18 lh-22 ps-3 d-flex align-items-center p-0 mb-0">{lang::dict("Email")}</span>
                                        </label>
                                    </div>
                                    {select_option_forgot}
                                    {recover_password_field}
                                </div>
                                {notifications}
                                <button onclick=&on_change_forgot class="btn button-login w-100">
                                    <span class="text-white">{lang::dict("Continue")}</span>
                                    <i class="fas fa-arrow-right mx-2"></i>
                                </button>
                            </div>
                        </div>
                    </div>
                }
            }
            PageModeForgotPassword::ResendForgotPassword => {
                html! {
                    <div class="card bg-dark text-white h-100 w-100">
                        <img src="/static/new-arte.png" class="card-img" alt="..." style="height: 100vh; width: 100vw; object-fit: cover;" />
                        <div class="card-img-overlay">
                            <div class="col-sm-12 col-md-12 col-lg-6 col-xl-5 col-xxl-4 bg-section-login pd-sm-4 p-md-7 p-sm-6 p-4 d-flex flex-column justify-content-between h-85 h-sm-100">
                                <div class="d-flex justify-content-between">
                                    <div class="col-4">
                                        <img src="/static/logo-robox-maker.png" class="img-fluid" />
                                    </div>
                                    <a class="text-white noir-bold is-size-6 lh-19" onclick=&on_login_page_mode>
                                        <i class="fas fa-arrow-left mx-2"></i>
                                        <span>{lang::dict("Behind")}</span>
                                    </a>
                                </div>
                                <h1 class="text-white noir-bold text-center is-size-48 lh-58">{lang::dict("Email sent")}</h1>
                                <div class="d-flex flex-column">
                                    <span class="text-white noir-light text-center is-size-18 lh-22">{lang::dict("Please check your email.")}</span>
                                    <span class="text-white noir-light text-center is-size-18 lh-22">{lang::dict("If you can't find the email, check your SPAM folder.")}</span>
                                </div>
                                <div class="resend-forgot-image d-flex justify-content-center mx-3">
                                    <img src="/static/resend-forgot.png" />
                                </div>
                                {notifications}
                                <button onclick=&on_change_forgot class="btn button-login w-100">
                                    <span class="text-white">{lang::dict("Forward Email")}</span>
                                    <i class="fas fa-arrow-right mx-2"></i>
                                </button>
                            </div>
                        </div>
                    </div>
                }
            }
        };
        html! {
            <div class="notranslate w-100 h-100">{page_mode}</div>
        }
    }
}