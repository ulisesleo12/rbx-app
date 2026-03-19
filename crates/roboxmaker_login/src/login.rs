use log::*;
use regex::Regex;
use crate::signup::SignUp;
use yew::{prelude::*, web_sys};
use crate::forgot_password::ForgotPage;
use yew::{html, Component, ComponentLink, Html, ShouldRender};

use roboxmaker_main::lang;

pub struct LoginPage {
    link: ComponentLink<Self>,
    props: Properties,
    show_password: bool,
    page_mode: PageMode,
    verify_password: bool,
    validate_username: bool,
    username: String,
    password: String,
    confirmation_password: String,
    user_created_status: i32,
    change_eyes: IconLogin,
}

#[derive(Properties, Clone)]
pub struct Properties {
    pub on_app_login: Callback<(String, String)>,
    pub login_status: i32,
}

#[derive(Debug)]
pub enum PageMode {
    Login,
    SignUp,
    ForgotPassword,
}

#[derive(Debug, PartialEq)]
pub enum IconLogin {
    Eyes,
    EyeSlash,
}

#[derive(Debug)]
pub enum Message {
    AppLogIn,
    PageMode(PageMode),
    ShowPassword,
    UpdateUsername(String),
    UpdatePassword(String),
    Nothing,
    ChangeIconEye,
    UserCreateStatus(i32),
}

impl Component for LoginPage {
    type Message = Message;
    type Properties = Properties;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        LoginPage {
            link,
            props,
            page_mode: PageMode::Login,
            show_password: false,
            verify_password: false,
            validate_username: false,
            username: "".to_string(),
            password: "".to_string(),
            confirmation_password: "".to_string(),
            user_created_status: 0,
            change_eyes: IconLogin::EyeSlash,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        info!("{:?}", msg);
        let mut should_render = false;
        match msg {
            Message::AppLogIn => {
                self.props
                    .on_app_login
                    .emit((self.username.clone(), self.password.clone()));
                
            }
            Message::PageMode(mode) => {
                self.page_mode = mode;
                match &self.page_mode {
                    PageMode::Login => {
                        self.show_password = false;
                        self.verify_password = false;
                        self.validate_username = false;
                        self.props.login_status = 0;
                    }
                    PageMode::SignUp => {}
                    PageMode::ForgotPassword => {}
                }
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
            Message::UpdatePassword(password) => {
                self.password = password.clone();
                if password == self.confirmation_password && self.confirmation_password.len() > 5 {
                    self.verify_password = true;
                } else {
                    self.verify_password = false;
                }
                should_render = true;
            }
            Message::ShowPassword => {
                self.show_password = !self.show_password;
                should_render = true;
            }
            Message::Nothing => {}
            Message::ChangeIconEye => {
                if let Some(element) = gloo_utils::document().get_element_by_id("show-icon-login") {
                    if self.change_eyes == IconLogin::Eyes {
                        self.change_eyes = IconLogin::EyeSlash;
                        let _ = element.set_attribute("class", "far fa-eye-slash");
                    } else {
                        self.change_eyes = IconLogin::Eyes;
                        let _ = element.set_attribute("class", "far fa-eye");
                    }
                }
                self.link.send_message(Message::ShowPassword);
                should_render = true;
            }
            Message::UserCreateStatus(status) => {
                self.user_created_status = status
            }
        }
        should_render
    }
    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }
    fn view(&self) -> Html {
        let on_app_login = self.link.callback(|_| Message::AppLogIn);
        let on_enter_login = self.link.callback(|evt: web_sys::KeyboardEvent| {
            if evt.key_code() == 13 {
                Message::AppLogIn
            } else {
                Message::Nothing
            }
        });

        let on_update_username_login = self.link.callback(|e: InputData| Message::UpdateUsername(e.value.trim().to_string()));
        let on_update_password = self.link.callback(|e: InputData| Message::UpdatePassword(e.value.trim().to_string()));
        let on_signup = self.link.callback(move |_| Message::PageMode(PageMode::SignUp));
        let on_forgot_password_page_mode = self.link.callback(move |_| Message::PageMode(PageMode::ForgotPassword));

        let notifications = match self.page_mode {
            PageMode::Login => {
                if self.user_created_status == 201 {
                    html! {
                        <span class="text-success bg-white w-100 rounded-3 p-3">
                            {lang::dict("You are almost registered")}
                            // <br/>
                            // {lang::dict("Check and verify your email, then sign in")}
                        </span> 
                    }
                } else if self.props.login_status == 204 {
                    html! {
                        <span class="text-danger bg-white w-100 rounded-3 p-3">
                            {lang::dict("Login failed")}
                            <br/>
                            {lang::dict("Username or password incorrect")}
                        </span>                      
                    }
                } else {
                    html! {}
                }
            }
            PageMode::SignUp => html! {},
            PageMode::ForgotPassword => html! {}
        };

        let input_pasword_type = if self.show_password {
            "text"
        } else {
            "password"
        };

        let page_mode = match self.page_mode {
            PageMode::Login => {
                let on_show_icon = self.link.callback(move |_| Message::ChangeIconEye);
                html! {
                    <div class="card bg-dark text-white h-100 w-100">
                        <img src="/static/new-arte.png" class="card-img" alt="..." style="height: 100vh; width: 100vw; object-fit: cover;" />
                        <div class="card-img-overlay">
                            <div class="col-sm-12 col-md-12 col-lg-6 col-xl-5 col-xxl-4 bg-section-login pd-sm-4 p-md-7 p-sm-6 p-4 d-flex flex-column justify-content-between h-85 h-sm-100">
                                <div class="col-4">
                                    <img src="/static/logo-robox-maker.png" class="img-fluid" />
                                </div>
                                <div>
                                    <h1 class="text-white noir-bold is-size-48 lh-58">{lang::dict("LOGIN")}</h1>
                                    <div class="d-flex flex-row mt-3">
                                        <span class="text-white noir-light is-size-18 lh-22">{lang::dict("You do not have an account?")}</span>
                                        <a onclick=&on_signup>
                                            <span class="text-cyan-sky noir-bold is-size-18 lh-22 mx-2">{lang::dict("Sign Up")}</span>
                                        </a>
                                    </div>
                                </div>
                                <div>
                                    <label class="form-label text-white noir-bold is-size-16 lh-20">{lang::dict("Username o Email")}</label>
                                    <div class="input-group">
                                        <span class="input-group-text">
                                            <i class="far fa-user"></i>
                                        </span>
                                        <input type="text" class="form-control input-style-l" placeholder="alexa@example.com" oninput=&on_update_username_login autocomplete="on" required=true />
                                    </div>
                                </div>
                                <div>
                                    <label class="form-label text-white noir-bold is-size-16 lh-20">{lang::dict("Password")}</label>
                                    <div class="input-group">
                                        <span class="input-group-text">
                                            <i class="fas fa-lock"></i>
                                        </span>
                                        <input type=input_pasword_type class="form-control input-style-l reset-radius-l" placeholder="*********" oninput=&on_update_password onkeyup=&on_enter_login autocomplete="on" required=true />
                                        <span class="input-group-text reset-box-icon-l" onclick=&on_show_icon>
                                            <i class="far fa-eye-slash" id="show-icon-login"></i>
                                        </span>
                                    </div>
                                </div>
                                <div class="d-flex justify-content-between align-items-center">
                                    <div class="form-check">
                                        <input class="form-check-input" type="checkbox" value="" id="flexCheckDefault" />
                                        <label class="form-check-label text-white noir-light is-size-16 lh-19" for="flexCheckDefault">
                                            {lang::dict("Remember me")}
                                        </label>
                                    </div>
                                    <a onclick=&on_forgot_password_page_mode>
                                        <span class="text-cyan-sky noir-bold is-size-18 lh-22">{lang::dict("Forgot your password?")}</span>
                                    </a>
                                </div>
                                {notifications}
                                <button onclick=&on_app_login class="btn button-login w-100 mb-5">
                                    {lang::dict("Login")}
                                </button>
                            </div>
                        </div>
                    </div>
                }
            }
            PageMode::SignUp => {
                let on_page_mode = self.link.callback(|page| Message::PageMode(page));
                let on_created_status = self.link.callback(|status| Message::UserCreateStatus(status));
                html! {
                    <SignUp on_page_mode={on_page_mode}
                        on_created_status={on_created_status} />
                }
            }
            PageMode::ForgotPassword => {
                let on_page_mode = self.link.callback(|page| Message::PageMode(page));
                html! {
                    <ForgotPage on_page_mode={on_page_mode}/>
                }
            }
        };
        html! {
            <div class="notranslate w-100 h-100">{page_mode}</div>
        }
    }
}