use log::*;
use regex::Regex;
// use uuid::Uuid;
// use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use crate::signup::SignUp;
// use gloo_storage::{Storage, LocalStorage};
use yew::{html, Component, Html};
use crate::forgot_password::ForgotPage;

use roboxmaker_main::lang;
// use roboxmaker_graphql::{SubscriptionTask, GraphQLTask};
use roboxmaker_utils::functions::get_value_from_input_event;
// use roboxmaker_models::auth;
// use roboxmaker_types::types::{MyUserProfile, DataSchoolProfile};

pub struct LoginPage {
    // graphql_task: Option<GraphQLTask>,
    show_password: bool,
    page_mode: PageMode,
    verify_password: bool,
    validate_username: bool,
    username: String,
    password: String,
    confirmation_password: String,
    user_created_status: i32,
    change_eyes: IconLogin,
    login_status: i32,
    // user_profile: Option<MyUserProfile>,
    // school_profile: Option<DataSchoolProfile>,
    // auth: Option<(auth::Auth, auth::Token)>,
    // user_profile_sub: Option<SubscriptionTask>,
    // route: AppRoute,
}

#[derive(Properties, Clone, PartialEq)]
pub struct Properties {
    // pub on_app_route: AppRoute,
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
    // LoginWithUsernamePassword,
    // AuthToken(Option<(auth::Auth, auth::Token)>, bool),
    // AuthUser(Option<user_model::user_by_id::ResponseData>),
    // AuthSchool(Option<school_model::school_by_id::ResponseData>),
    // FetchUserById,
}

impl Component for LoginPage {
    type Message = Message;
    type Properties = Properties;

    fn create(_ctx: &Context<Self>) -> Self {

        LoginPage {
            // graphql_task: None,
            page_mode: PageMode::Login,
            show_password: false,
            verify_password: false,
            validate_username: false,
            username: "".to_string(),
            password: "".to_string(),
            confirmation_password: "".to_string(),
            user_created_status: 0,
            change_eyes: IconLogin::EyeSlash,
            login_status: 0,
            // user_profile: None,
            // school_profile: None,
            // auth: None,
            // user_profile_sub: None,
            // route: AppRoute::Login,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        info!("{:?}", msg);
        let mut should_render = false;
        match msg {
            Message::AppLogIn => {
                ctx.props()
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
                ctx.link().send_message(Message::ShowPassword);
                should_render = true;
            }
            Message::UserCreateStatus(status) => {
                self.user_created_status = status
            }
            // Message::LoginWithUsernamePassword => {
            //     let token_req = TokenRequest::AccessToken(AccessTokenRequest {
            //         grant_type: "password".to_string(),
            //         client_id: "app-aker".to_string(),
            //         username: self.username.clone(),
            //         password: self.password.clone(),
            //         scope: "openid".to_string(),
            //     });

            //     let link = ctx.link().clone();
            //     spawn_local(async move {

            //         let reponse = auth::token_req(token_req).await;

            //         match reponse {
            //             Ok(data) => {
            //                 link.send_message(Message::AuthToken(data, true))
            //             },
            //             Err(e) => {
            //                 info!("ErrorToken: {:?}", e)
            //             },
            //         }
            //     });
            // }
            // Message::AuthToken(auth, should_notify) => {
            //     if let Some((auth, token)) = auth {
            //         gloo_storage::LocalStorage::set(config::AKER_AUTH_KEY, auth).ok();
            //         gloo_storage::LocalStorage::set(config::AKER_TOKEN_KEY, token).ok();

            //         self.login_status = 0;

            //         if ctx.props().on_app_route == AppRoute::Login {
            //             ctx.link().navigator().unwrap().push(&AppRoute::Home);
            //         }

            //     } else {
            //         if should_notify {
            //             self.login_status = 204;
            //         }
            //         // ctx.link()
            //         //     .send_message(Message::ChangeRoute(AppRoute::Login));
            //         ctx.link().navigator().unwrap().push(&AppRoute::Login);
            //     }
            // }
            // Message::AuthUser(response) => {
            //     self.user_profile = response.clone()
            //         .and_then(|data| data.user_by_pk)
            //         .and_then(|item| {
            //             Some(MyUserProfile {
            //                 email: item.user_profile.clone().and_then(|d|d.email).unwrap_or("example.123@gmail.co".to_string()),
            //                 full_name: item.user_profile.clone().and_then(|d|Some(d.full_name)).unwrap_or("".to_string()),
            //                 pic_path: item.user_profile.clone().and_then(|d|d.pic_path).unwrap_or("/static/avatar.png".to_string()),
            //                 user_id: roboxmaker_types::types::UserId(item.user_profile.clone().and_then(|d|Some(d.user_id)).unwrap_or(Uuid::default())),
            //                 school_name: item.user_profile.clone().and_then(|d|d.group_member).and_then(|group_member|group_member.school_group).and_then(|school_group|Some(school_group.school)).and_then(|school|school.school_profile).and_then(|school_profile|Some(school_profile.name)).unwrap_or("".to_string()),
            //                 user_student: item.user_student.and_then(|d|Some(UserId(d.user_id))),
            //                 user_teacher: item.user_teacher.and_then(|d|Some(UserId(d.user_id))),
            //                 user_staff: item.user_staff.and_then(|d|Some(UserId(d.user_id))),
            //                 license: item.license.and_then(|d|Some(d.license)).unwrap_or("AAAAAAAAAAAAAAA".to_string()),
            //                 group_member_id: roboxmaker_types::types::GroupId(item.group_member.and_then(|d|Some(d.group_id)).unwrap_or(Uuid::default())),
            //             })
            //         });
            //     if response.is_some() {
            //         LocalStorage::set("USER-PROFILE", self.user_profile.clone()).ok();
            //     }

            // }
            // Message::AuthSchool(response) => {
            //     self.school_profile = response.clone()
            //         .and_then(|data| data.school_by_pk.clone())
            //         .and_then(|item| {
            //             Some(DataSchoolProfile {
            //                 name: item.school_profile.clone().and_then(|d|Some(d.name)).unwrap_or("".to_string()),
            //                 logo: item.school_profile.clone().and_then(|d|d.logo).unwrap_or("https://files.roboxmaker.com/uploads/school.png".to_string()),
            //                 school_id: SchoolId(item.school_profile.clone().and_then(|d|Some(d.school_id)).unwrap_or(Uuid::default())),
            //                 group_member_id: GroupId(item.school_profile.clone().and_then(|d| d.inventory_group).and_then(|data| Some(data.group_id)).unwrap_or(Uuid::default())),
            //             })
            //         });
            //     if response.is_some() {
            //         LocalStorage::set("SCHOOL-PROFILE", self.school_profile.clone()).ok();
            //     }
            // }
            // Message::FetchUserById => {
            //     if let Some((_auth, token)) = &self.auth {
            //         if let Some(graphql_task) = self.graphql_task.as_mut() {
            //             let vars = user_model::user_by_id::Variables {
            //                 user_id: token.claims.user_id,
            //             };

            //             let task = user_model::UserById::subscribe(
            //                 graphql_task,
            //                 &ctx, 
            //                 vars, 
            //                 |response| {
            //                     Message::AuthUser(response)
            //                 },
            //             );

            //             self.user_profile_sub = Some(task);
            //         }
            //     }
            // }
        }
        should_render
    }
    fn changed(&mut self, _ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
        true
    }
    fn view(&self, ctx: &Context<Self>) -> Html {

        let on_app_login = ctx.link().callback(|_| Message::AppLogIn);
        let on_enter_login = ctx.link().callback(|evt: web_sys::KeyboardEvent| {
            if evt.key_code() == 13 {
                Message::AppLogIn
            } else {
                Message::Nothing
            }
        });

        let on_update_username_login = ctx.link().callback(|e: InputEvent| Message::UpdateUsername(get_value_from_input_event(e)));
        let on_update_password = ctx.link().callback(|e: InputEvent| Message::UpdatePassword(get_value_from_input_event(e)));
        let on_signup = ctx.link().callback(move |_| Message::PageMode(PageMode::SignUp));
        let on_forgot_password_page_mode = ctx.link().callback(move |_| Message::PageMode(PageMode::ForgotPassword));

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
                } else if self.login_status == 204 {
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
                let on_show_icon = ctx.link().callback(move |_| Message::ChangeIconEye);
                html! {
                    <div class="card bg-dark text-white h-100 w-100">
                        <img src="/static/arte-1.png" class="card-img" alt="..." style="height: 100vh; width: 100vw; object-fit: cover;" />
                        <div class="card-img-overlay">
                            <div class="col-sm-12 col-md-12 col-lg-6 col-xl-5 col-xxl-4 bg-section-login pd-sm-4 p-md-7 p-sm-6 p-4 d-flex flex-column justify-content-between h-85 h-sm-100">
                                <div class="col-4">
                                    <img src="/static/logo-robox-maker.png" class="img-fluid" />
                                </div>
                                <div>
                                    <h1 class="text-white noir-bold is-size-48 lh-58">{lang::dict("LOGIN")}</h1>
                                    <div class="d-flex flex-row mt-3">
                                        <span class="text-white noir-light is-size-18 lh-22">{lang::dict("You do not have an account?")}</span>
                                        <a onclick={&on_signup}>
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
                                        <input type="text" class="form-control input-style-l" placeholder="alexa@example.com" oninput={&on_update_username_login} autocomplete="off" required=true />
                                    </div>
                                </div>
                                <div>
                                    <label class="form-label text-white noir-bold is-size-16 lh-20">{lang::dict("Password")}</label>
                                    <div class="input-group">
                                        <span class="input-group-text">
                                            <i class="fas fa-lock"></i>
                                        </span>
                                        <input type={input_pasword_type} class="form-control input-style-l reset-radius-l" placeholder="*********" oninput={&on_update_password} onkeyup={&on_enter_login} autocomplete="off" required=true />
                                        <span class="input-group-text reset-box-icon-l" onclick={&on_show_icon}>
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
                                    <a onclick={&on_forgot_password_page_mode}>
                                        <span class="text-cyan-sky noir-bold is-size-18 lh-22">{lang::dict("Forgot your password?")}</span>
                                    </a>
                                </div>
                                {notifications}
                                <button onclick={&on_app_login} class="btn button-login w-100 mb-5">
                                    {lang::dict("Login")}
                                </button>
                            </div>
                        </div>
                    </div>
                }
            }
            PageMode::SignUp => {
                let on_page_mode = ctx.link().callback(|page| Message::PageMode(page));
                let on_created_status = ctx.link().callback(|status| Message::UserCreateStatus(status));
                html! {
                    <SignUp on_page_mode={on_page_mode}
                        on_created_status={on_created_status} />
                }
            }
            PageMode::ForgotPassword => {
                let on_page_mode = ctx.link().callback(|page| Message::PageMode(page));
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