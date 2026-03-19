use log::*;
use regex::Regex;
use uuid::Uuid;
use crate::login::PageMode;
use dyn_fmt::AsStrFormatExt;
use yew::{html, Component, Html};
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

use roboxmaker_main::{config, lang};
use roboxmaker_utils::functions::get_value_from_input_event;
use roboxmaker_models::registration::{
    self, class_name_group, inventory_group_id, new_user_add, user_exist, check_school_by_license
};

use roboxmaker_types::types::{GroupId, SchoolId};

#[derive(Debug, Clone, PartialEq)]
pub struct SchoolDataCheck {
    pub school_name: String,
    pub school_id: Uuid,
    pub exist: bool,
    pub used: bool,
}

pub struct SignUp {
    url_photo: String,
    show_password: bool,
    show_confirmation_password: bool,
    school_id: Option<SchoolId>,
    group_id: Option<GroupId>,
    page_mode: PageModeSignUp,
    verify_license: bool,
    verify_password: bool,
    validate_username: bool,
    validate_email: bool,
    license: String,
    firstname: String,
    lastname: String,
    email: String,
    username: String,
    password: String,
    confirmation_password: String,
    school_name: String,
    grade: String,
    section: String,
    select_node: NodeRef,
    license_verify_status: i32,
    user_created_status: i32,
    verify_username_status: i32,
    verify_email_status: i32,
    change_eyes: IconLogin,
    // school_data: Option<registration::check_license_information::ResponseData>,
    // school_data: Vec<SchoolDataCheck>,
    license_exist: bool,
    license_used: bool,
}

#[derive(Properties, Clone, PartialEq)]
pub struct Properties {
    pub on_page_mode: Callback<PageMode>,
    pub on_created_status: Callback<i32>,
}

#[derive(Debug)]
pub enum PageModeSignUp {
    VerifyLicense,
    SignUp,
    SelectInstitution,
}

#[derive(Debug, PartialEq)]
pub enum IconLogin {
    Eyes,
    EyeSlash,
}

#[derive(Debug)]
pub enum Message {
    ChangePageMode(PageMode),
    PageMode(PageModeSignUp),
    ShowPassword,
    ShowConfirmationPassword,
    UserSignUp(Uuid),
    NewUserCreated(Option<registration::new_user_create::ResponseData>),
    UpdateLicense(String),
    UpdateFirstName(String),
    UpdateLastName(String),
    UpdateEmail(String),
    UpdateUsername(String),
    UpdatePassword(String),
    UpdateConfimationPassword(String),
    UpdateSection,
    FetchSchoolIdByLicense,
    // SchoolId(Option<registration::school_id_by_license::ResponseData>),
    SchoolData(Option<registration::check_license_information::ResponseData>),
    FetchGroupId,
    FetchInventoryGroupId(GroupId),
    Nothing,
    ChangeIconEye,
    ChangeIconEyeConfirmation,
    VerifyUsername,
    VerifyEmail,
    ResponseUsername(Option<registration::verify_user_exist::ResponseData>),
    ResponseEmail(Option<registration::verify_user_exist::ResponseData>),
}

impl Component for SignUp {
    type Message = Message;
    type Properties = Properties;

    fn create(_ctx: &Context<Self>) -> Self {
        SignUp {
            page_mode: PageModeSignUp::VerifyLicense,
            show_password: false,
            show_confirmation_password: false,
            verify_license: false,
            verify_password: false,
            validate_username: false,
            validate_email: false,
            url_photo: format!("{}/uploads/avatar.png", config::AKER_FILES_URL),
            license: "".to_string(),
            firstname: "".to_string(),
            lastname: "".to_string(),
            email: "".to_string(),
            username: "".to_string(),
            password: "".to_string(),
            confirmation_password: "".to_string(),
            school_name: "".to_string(),
            grade: "".to_string(),
            section: "A".to_string(),
            school_id: None,
            group_id: None,
            license_verify_status: 0,
            user_created_status: 0,
            verify_username_status: 0,
            verify_email_status: 0,
            select_node: NodeRef::default(),
            change_eyes: IconLogin::EyeSlash,
            // school_data: vec![],
            license_exist: false,
            license_used: false,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        info!("{:?}", msg);
        let mut should_render = false;
        match msg {
            Message::ChangePageMode(page_mode) => ctx.props().on_page_mode.emit(page_mode),
            Message::PageMode(mode) => {
                self.page_mode = mode;
                self.user_created_status = 0;
                match &self.page_mode {
                    PageModeSignUp::VerifyLicense => {
                        self.verify_license = false;
                        self.verify_password = false;
                    }
                    PageModeSignUp::SignUp => {
                        self.show_password = false;
                        self.show_confirmation_password = false;
                        self.verify_username_status = 0;
                        self.verify_email_status = 0;
                    }
                    PageModeSignUp::SelectInstitution => {}
                }
                should_render = true;
            }
            Message::UpdateLicense(license) => {
                self.license = license.clone();
                if license.len() > 4 {
                    let nivel_char = license[3..4].to_string();
                    let grade_char = license[4..5].to_string();
                    info!("LICENSE {}", license);

                    if nivel_char == "K" && grade_char == "4" {
                        self.grade = "Kinder 4".to_string();
                    } else if nivel_char == "K" && grade_char == "5" {
                        self.grade = "Kinder 5".to_string();
                    } else if nivel_char == "P" && grade_char == "P" {
                        self.grade = "Preparatoria".to_string();
                    } else if nivel_char == "P" {
                        self.grade = format!("{} {}", "Primaria", grade_char);
                    } else if nivel_char == "S" {
                        self.grade = format!("{} {}", "Secundaria", grade_char);
                    } else if nivel_char == "B" {
                        self.grade = format!("{} {}", "Bachillerato", grade_char);
                    } else {
                        self.grade = "".to_string();
                    }
                    info!("{}", self.grade)
                }
                // if license.len() < 15 {
                // }
                if license.len() <= 15 {
                    ctx.link().send_message(Message::FetchSchoolIdByLicense)
                } else {
                    self.license_verify_status = 102;
                }
                should_render = true;
            }
            Message::UpdateFirstName(firstname) => {
                self.firstname = firstname;
                should_render = true;
            }
            Message::UpdateLastName(lastname) => {
                self.lastname = lastname;
                should_render = true;
            }
            Message::UpdateEmail(email) => {
                let email_regex = Regex::new(r"^([a-z0-9_+]([a-z0-9_+.]*[a-z0-9_+])?)@([a-z0-9]+([\-\.]{1}[a-z0-9]+)*\.[a-z]{2,6})").unwrap();
                if email_regex.is_match(&email) {
                    self.validate_email = true;
                    ctx.link().send_message(Message::VerifyEmail);
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
                if self.username.len() > 5 {
                    ctx.link().send_message(Message::VerifyUsername);
                }
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
            Message::UpdateConfimationPassword(confirmation_password) => {
                self.confirmation_password = confirmation_password.clone();
                if confirmation_password == self.password && self.password.len() > 5 {
                    self.verify_password = true;
                } else {
                    self.verify_password = false;
                }
                should_render = true;
            }
            Message::UpdateSection => {
                let node = self
                    .select_node
                    .cast::<web_sys::HtmlSelectElement>()
                    .unwrap();
                self.section = node.value();
                info!("{}", self.section);
            }
            Message::ShowPassword => {
                self.show_password = !self.show_password;
                should_render = true;
            }
            Message::ShowConfirmationPassword => {
                self.show_confirmation_password = !self.show_confirmation_password;
                should_render = true;
            }
            Message::FetchSchoolIdByLicense => {
                self.license_verify_status = 102;

                let vars = registration::check_license_information::Variables {
                    license: self.license.clone(),
                };
                let license = self.license.clone();

                let link = ctx.link().clone();
                spawn_local(async move {
                    let response = check_school_by_license(vars, license).await;
                    match response {
                        Ok(data) => link.send_message(Message::SchoolData(data)),
                        Err(_e) => {}
                    }
                });
                should_render = true;
            }
            Message::FetchGroupId => {
                self.user_created_status = 102;
                if self.school_id.is_some() {
                    let vars = registration::group_id_by_class_name::Variables {
                        class_name: format!("{}-{}", self.grade.clone(), self.section.clone()),
                        school_id: self.school_id.unwrap().0,
                    };
                    let link = ctx.link().clone();
                    spawn_local(async move {
                        let response = class_name_group(vars).await;
                        match response {
                            Ok(data) => {
                                let group_id = data
                                    .clone()
                                    .and_then(|data| Some(data.class_group))
                                    .unwrap_or(vec![])[0]
                                    .group_id;
                                link.send_message(Message::FetchInventoryGroupId(GroupId(group_id)))
                            }
                            Err(_e) => {}
                        }
                    });
                    // let class_name = format!("{}-{}", self.grade.clone(), self.section.clone());
                    // info!("DATA?: {:?}", class_name);
                }
                should_render = true;
            }
            Message::SchoolData(response) => {
                if let Some(data) = response
                    .clone()
                    .and_then(|data| Some(data.check_license_registered_exist))
                    .unwrap_or(vec![])
                    .first() {
                        self.school_id = Some(SchoolId(data.school_id.unwrap_or_default()));
                        self.school_name = data.school_name.clone();
                        self.license_exist = data.exist;
                        self.license_used = data.used;
                        self.license_verify_status = 200;
                    }

                info!("DATA?: {:?}", response.clone());

                info!("verify_license {}", self.verify_license);
                should_render = true;
            }
            Message::FetchInventoryGroupId(group_id) => {
                let school_id = self.school_id.unwrap();
                self.group_id = Some(group_id);
                let vars = registration::inventory_group_id_by_school_id::Variables {
                    school_id: school_id.0,
                };
                let link = ctx.link().clone();
                spawn_local(async move {
                    let response = inventory_group_id(vars).await;
                    match response {
                        Ok(data) => {
                            let inventory_group_data = &data
                                .clone()
                                .and_then(|data| Some(data.school_group))
                                .clone()
                                .unwrap_or(vec![])[0]
                                .inventory_group;
                            let inventory_group_id = inventory_group_data.clone().unwrap().group_id;
                            link.send_message(Message::UserSignUp(inventory_group_id))
                        }
                        Err(_e) => {}
                    }
                });
            }
            Message::UserSignUp(inventory_group_id) => {
                let full_name = format!("{} {}", self.firstname, self.lastname);
                let school_id = self.school_id.unwrap();
                let group_id = self.group_id.unwrap();
                let vars = registration::new_user_create::Variables {
                    user_id: Uuid::new_v4(),
                    license: self.license.clone(),
                    full_name: full_name.clone(),
                    pic_path: self.url_photo.clone(),
                    first_name: self.firstname.clone(),
                    last_name: self.lastname.clone(),
                    email: self.email.clone(),
                    password: self.password.clone(),
                    username: self.username.clone(),
                    role: "student".to_string(),
                    school_id: school_id.0,
                    group_id: group_id.0,
                    inventory_group_id: inventory_group_id,
                };
                let license = self.license.clone();
                let link = ctx.link().clone();
                spawn_local(async move {
                    let response = new_user_add(vars, license).await;
                    match response {
                        Ok(data) => link.send_message(Message::NewUserCreated(data)),
                        Err(_e) => {}
                    }
                });
            }
            Message::NewUserCreated(user) => {
                if user
                    .and_then(|data| data.create_user_action)
                    .and_then(|create_user_action| Some(create_user_action.code))
                    .unwrap_or(000)
                    == 201
                {
                    ctx.link()
                        .send_message(Message::ChangePageMode(PageMode::Login));

                    self.verify_license = false;
                    self.license = "".to_string();
                    self.grade = "".to_string();
                    self.user_created_status = 201;
                    ctx.props().on_created_status.emit(201);
                } else {
                    ctx.props().on_created_status.emit(404);
                    self.user_created_status = 404;
                }
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
            Message::ChangeIconEyeConfirmation => {
                if let Some(element) =
                    gloo_utils::document().get_element_by_id("show-icon-login-confirmation")
                {
                    if self.change_eyes == IconLogin::Eyes {
                        self.change_eyes = IconLogin::EyeSlash;
                        let _ = element.set_attribute("class", "far fa-eye-slash");
                    } else {
                        self.change_eyes = IconLogin::Eyes;
                        let _ = element.set_attribute("class", "far fa-eye");
                    }
                }
                ctx.link().send_message(Message::ShowConfirmationPassword);
                should_render = true;
            }
            Message::VerifyUsername => {
                let vars = registration::verify_user_exist::Variables {
                    username: self.username.clone(),
                    email: String::from(""),
                };
                let link = ctx.link().clone();
                spawn_local(async move {
                    let response = user_exist(vars).await;
                    match response {
                        Ok(data) => link.send_message(Message::ResponseUsername(data)),
                        Err(_e) => {}
                    }
                });
                should_render = true;
            }
            Message::ResponseUsername(response) => {
                if response
                    .clone()
                    .and_then(|data| data.user_exist_action)
                    .clone()
                    .and_then(|username| Some(username.code))
                    .unwrap_or(000)
                    == 409
                {
                    self.verify_username_status = 409;
                } else {
                    self.verify_username_status = 404;
                }
                should_render = true;
            }
            Message::VerifyEmail => {
                let vars = registration::verify_user_exist::Variables {
                    username: String::from(""),
                    email: self.email.clone(),
                };
                let link = ctx.link().clone();
                spawn_local(async move {
                    let response = user_exist(vars).await;
                    match response {
                        Ok(data) => link.send_message(Message::ResponseEmail(data)),
                        Err(_e) => {}
                    }
                });
                should_render = true;
            }
            Message::ResponseEmail(response) => {
                if response
                    .clone()
                    .and_then(|data| data.user_exist_action)
                    .clone()
                    .and_then(|email| Some(email.code))
                    .unwrap_or(000)
                    == 409
                {
                    self.verify_email_status = 408;
                    self.validate_email = false;
                } else {
                    self.verify_email_status = 404;
                    self.validate_email = true;
                }
                should_render = true;
            }
        }
        should_render
    }
    fn changed(&mut self, _ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
        true
    }
    fn view(&self, ctx: &Context<Self>) -> Html {

        let on_update_license = ctx
            .link()
            .callback(|e: InputEvent| Message::UpdateLicense(get_value_from_input_event(e)));
        let on_update_first_name = ctx
            .link()
            .callback(|e| Message::UpdateFirstName(get_value_from_input_event(e)));
        let on_update_last_name = ctx
            .link()
            .callback(|e| Message::UpdateLastName(get_value_from_input_event(e)));
        let on_update_email = ctx
            .link()
            .callback(|e| Message::UpdateEmail(get_value_from_input_event(e)));
        let on_update_username = ctx
            .link()
            .callback(|e| Message::UpdateUsername(get_value_from_input_event(e)));
        let on_update_password = ctx
            .link()
            .callback(|e| Message::UpdatePassword(get_value_from_input_event(e)));

        let on_update_confirmation_password = ctx.link().callback(|e: InputEvent| {
            Message::UpdateConfimationPassword(get_value_from_input_event(e))
        });
        let on_change_section = ctx.link().callback(move |_| Message::UpdateSection);
        let maybe_user_create = ctx.link().callback(|_| Message::FetchGroupId);
        let on_login_page_mode = ctx
            .link()
            .callback(move |_| Message::ChangePageMode(PageMode::Login));
        let on_select_institution = ctx
            .link()
            .callback(move |_| Message::PageMode(PageModeSignUp::SelectInstitution));

        let maybe_license = {
            if !self.license_exist && self.license_verify_status == 200 && self.license.len() == 15 {
                html! {
                    <span class="text-danger bg-white w-100 rounded-3 p-3">
                        {"Esta licencia no existe"}
                    </span>
                }
            } else if self.license_used && self.license_verify_status == 200 && self.license.len() == 15 {
                html! {
                    <span class="text-danger bg-white w-100 rounded-3 p-3">
                        {"Esta licencia ya fue ocupada"}
                    </span>
                }
            } else {
                html! {}
            }
        };

        let notifications = match self.page_mode {
            PageModeSignUp::VerifyLicense => {
                // if !self.verify_license
                //     && self.license_verify_status == 404
                //     && self.license.len() > 14
                // {
                //     html! {
                //         <span class="text-danger bg-white w-100 rounded-3 p-3">
                //             {lang::dict("This license has already been occupied or not exist")}
                //         </span>
                //     }
                // } else if !self.verify_license
                //     && self.license_verify_status == 102
                //     && self.license.len() > 14
                // {
                //     html! {
                //         <span class="text-warning bg-white w-100 rounded-3 p-3">
                //             {lang::dict("Verifying License...")}
                //         </span>
                //     }
                // } else if !self.verify_license && self.license.len() > 0 && self.license.len() < 15 {
                if !self.verify_license && self.license.len() > 0 && self.license.len() < 15 {
                    let license_len = lang::dict("License needs {} more characters")
                        .format(&[15 - self.license.len()]);
                    html! {
                        <span class="text-warning bg-white w-100 rounded-3 p-3">
                            {license_len}
                        </span>
                    }
                } else {
                    html! {
                        {maybe_license}
                    }
                }
            }
            PageModeSignUp::SignUp => {
                let password = if self.password.len() > 0 && self.password.len() < 6
                    || self.confirmation_password.len() > 0 && self.confirmation_password.len() < 6
                {
                    html! {
                        <span class="text-danger bg-white w-100 rounded-3 p-3">
                            {lang::dict("Very short password")}
                            <br/>
                            {lang::dict("6 or more characters")}
                        </span>
                    }
                } else if !self.verify_password
                    && self.password.len() > 5
                    && self.confirmation_password.len() > 5
                    && self.confirmation_password != self.password
                {
                    html! {
                        <span class="text-danger bg-white w-100 rounded-3 p-3">
                            {lang::dict("Passwords do not match")}
                            <br/>
                            {lang::dict("Check")}
                        </span>
                    }
                } else {
                    html! {}
                };
                let created = if self.user_created_status == 404 {
                    html! {
                        <span class="text-danger bg-white w-100 rounded-3 p-3">
                            {lang::dict("Registration failed")}
                            <br/>
                            {lang::dict("A student already exists with this username or email")}
                        </span>
                    }
                } else if self.user_created_status == 102 {
                    html! {
                        <span class="text-warning bg-white w-100 rounded-3 p-3">
                            {lang::dict("Processing register...")}
                        </span>
                    }
                } else {
                    html! {}
                };
                let username = if self.verify_username_status == 409 {
                    html! {
                        <span class="text-danger bg-white w-100 rounded-3 p-3">
                            {lang::dict("This username already exists")}
                        </span>
                    }
                } else if self.verify_username_status == 404 {
                    html! {
                        // <span class="text-success bg-white w-100 rounded-3 p-3">
                        //     {lang::dict("This username is available")}
                        // </span>
                    }
                } else {
                    html! {}
                };
                let username_characters = if self.username.len() > 0 && self.username.len() < 5 {
                    let username_len = lang::dict("Your username is less than {} characters")
                        .format(&[5 - self.username.len()]);
                    html! {
                        <span class="text-warning bg-white w-100 rounded-3 p-3">
                            {username_len}
                        </span>
                    }
                } else {
                    html! {}
                };
                let email = if self.verify_email_status == 408 {
                    html! {
                        <span class="text-danger bg-white w-100 rounded-3 p-3">
                            {lang::dict("This email has already been used in an account")}
                        </span>
                    }
                } else if self.verify_email_status == 404 {
                    html! {
                        // <h6>{lang::dict("This email has not been linked to any account")}</h6>
                    }
                } else {
                    html! {}
                };
                html! {
                    <>
                        {username}
                        {username_characters}
                        {email}
                        {password}
                        {created}
                    </>
                }
            }
            PageModeSignUp::SelectInstitution => {
                if self.user_created_status == 102 {
                    html! {
                        <span class="text-warning bg-white w-100 rounded-3 p-3">
                            {lang::dict("Processing register...")}
                        </span>
                    }
                } else {
                    html! {}
                }
            }
        };
        let on_disabled_button = if self.license_exist
            && self.verify_password
            && self.validate_email
            && !self.firstname.is_empty()
            && !self.lastname.is_empty()
            && !self.username.is_empty()
            && self.verify_username_status == 404
            && self.verify_email_status == 404
        {
            false
        } else {
            true
        };
        let input_pasword_type = if self.show_password {
            "text"
        } else {
            "password"
        };
        let input_confirmation_password_type = if self.show_confirmation_password == true {
            "text"
        } else {
            "password"
        };
        let section = html! {
            <>
                <div class="field mb-4">
                    <span class="text-white noir-bold is-size-16 lh-20">{lang::dict("Section")}</span>
                    <div class="select mt-2" style="width: 100%; height: 50px;">
                        <select ref={self.select_node.clone()} onchange={on_change_section} class="input-style-l is-fullwidth" style="width: 100%; border-color: #DBDBFF; color: #BCBDFD; top: auto; padding-left: 16px;">
                            <option value="A">{"A"}</option>
                            <option value="B">{"B"}</option>
                            <option value="C">{"C"}</option>
                            <option value="D">{"D"}</option>
                            <option value="E">{"E"}</option>
                        </select>
                    </div>
                </div>
            </>
        };
        let on_license_verified = {
            html! {
                <>
                    <div class="mt-3">
                        <label class="form-label text-white noir-bold is-size-16 lh-20">{lang::dict("Name")}</label>
                        <div class="input-group">
                            <span class="input-group-text">
                                <i class="far fa-id-badge"></i>
                            </span>
                            <input type="text" class="form-control input-style-l" placeholder="Alexa Esmeralda" oninput={&on_update_first_name} value={self.firstname.clone()} required=true />
                        </div>
                    </div>
                    <div class="mt-3">
                        <label class="form-label text-white noir-bold is-size-16 lh-20">{lang::dict("Last Name")}</label>
                        <div class="input-group">
                            <span class="input-group-text">
                                <i class="far fa-id-badge"></i>
                            </span>
                            <input type="text" class="form-control input-style-l" placeholder="Hernández Gutiérrez" oninput={&on_update_last_name} value={self.lastname.clone()} required=true />
                        </div>
                    </div>
                    <div class="mt-3">
                        <label class="form-label text-white noir-bold is-size-16 lh-20">{lang::dict("Username")}</label>
                        <div class="input-group">
                            <span class="input-group-text">
                                <i class="far fa-user"></i>
                            </span>
                            <input type="text" class="form-control input-style-l" placeholder="alexahrdz01" oninput={&on_update_username} value={self.username.clone()} required=true />
                        </div>
                    </div>
                    <div class="mt-3">
                        <label class="form-label text-white noir-bold is-size-16 lh-20">{lang::dict("Email")}</label>
                        <div class="input-group">
                            <span class="input-group-text">
                                <i class="far fa-user"></i>
                            </span>
                            <input type="text" class="form-control input-style-l" placeholder="alexa@example.com" oninput={&on_update_email} value={self.email.clone()} required=true />
                        </div>
                    </div>
                </>
            }
        };
        let page_mode = match self.page_mode {
            PageModeSignUp::VerifyLicense => {
                let maybe_disabled_btn = if 
                    !self.license_used 
                    && self.license_exist
                    && self.license_verify_status == 200 
                    && self.license.len() == 15 || 
                    self.license_exist 
                    && !self.license_used
                    && self.license_verify_status == 200 
                    && self.license.len() == 15 {
                    false
                } else {
                    true
                };

                let on_verify_license = ctx.link().callback(|_| Message::PageMode(PageModeSignUp::SignUp));
                html! {
                    <div class="card bg-dark text-white h-100 w-100">
                        <img src="/static/arte-4.png" class="card-img" alt="..." style="height: 100vh; width: 100vw; object-fit: cover;" />
                        <div class="card-img-overlay d-flex justify-content-start justify-content-sm-center justify-content-md-end justify-content-lg-end">
                            <div class="col-sm-12 col-md-12 col-lg-6 col-xl-5 col-xxl-4 bg-section-login pd-sm-4 p-md-7 p-sm-6 p-4 d-flex flex-column justify-content-between h-85 h-sm-100">
                                <div class="col-4">
                                    <img src="/static/logo-robox-maker.png" class="img-fluid" />
                                </div>
                                <div>
                                    <h1 class="text-white noir-bold is-size-48 lh-58">{lang::dict("Create Account")}</h1>
                                    <div class="d-flex flex-row mt-3">
                                        <span class="text-white noir-light is-size-18 lh-22">{lang::dict("Do you already have an account?")}</span>
                                        <a onclick={&on_login_page_mode}>
                                            <span class="text-cyan-sky noir-bold is-size-18 lh-22 mx-2">{lang::dict("Login")}</span>
                                        </a>
                                    </div>
                                </div>
                                <div class="text-center">
                                    <label class="form-label text-white noir-bold is-size-16 lh-20">{lang::dict("Enter a valid license to continue")}</label>
                                    <div class="input-group">
                                        <span class="input-group-text">
                                            <i class="far fa-credit-card"></i>
                                        </span>
                                        <input type="text" class="form-control input-style-l" placeholder="AAAAAAAAAAAAAAA" oninput={&on_update_license} minlength="15" maxlength="15" required=true value={self.license.to_uppercase()} />
                                    </div>
                                </div>
                                {notifications}
                                <button onclick={&on_verify_license} disabled={maybe_disabled_btn} class="btn button-login w-100 mb-5">
                                    <span class="text-white">{lang::dict("Continue")}</span>
                                    <i class="fas fa-arrow-right mx-2"></i>
                                </button>
                            </div>
                        </div>
                    </div>
                }
            }
            PageModeSignUp::SignUp => {
                let on_show_icon = ctx.link().callback(move |_| Message::ChangeIconEye);
                let on_show_icon_confirmation = ctx
                    .link()
                    .callback(move |_| Message::ChangeIconEyeConfirmation);
                let section_signup_2 = html! {
                    <>
                        <div class="mt-3">
                            <label class="form-label text-white noir-bold is-size-16 lh-20">{lang::dict("Password")}</label>
                            <div class="input-group">
                                <span class="input-group-text">
                                    <i class="fas fa-lock"></i>
                                </span>
                                <input type={input_pasword_type} class="form-control input-style-l reset-radius-l" placeholder="*********" oninput={&on_update_password} required=true />
                                <span class="input-group-text reset-box-icon-l" onclick={&on_show_icon}>
                                    <i class="far fa-eye-slash" id="show-icon-login"></i>
                                </span>
                            </div>
                        </div>
                        <div class="mt-3">
                            <label class="form-label text-white noir-bold is-size-16 lh-20">{lang::dict("Confirm Password")}</label>
                            <div class="input-group">
                                <span class="input-group-text">
                                    <i class="fas fa-lock"></i>
                                </span>
                                <input type={input_confirmation_password_type} class="form-control input-style-l reset-radius-l" placeholder="*********" oninput={&on_update_confirmation_password} required=true />
                                <span class="input-group-text reset-box-icon-l" onclick={&on_show_icon_confirmation}>
                                    <i class="far fa-eye-slash" id="show-icon-login-confirmation"></i>
                                </span>
                            </div>
                        </div>
                        {notifications}
                        <button onclick={&on_select_institution} class="btn button-login w-100 mt-4 py-3"
                            disabled={on_disabled_button}>
                            <span>{lang::dict("Continue")}</span>
                            <i class="fas fa-arrow-right mx-2"></i>
                        </button>
                    </>
                };
                html! {
                    <div class="card bg-dark text-white h-100 w-100">
                        <img src="/static/arte-4.png" class="card-img" alt="..." style="height: 100vh; width: 100vw; object-fit: cover;" />
                        <div class="card-img-overlay d-flex justify-content-start justify-content-sm-center justify-content-md-end justify-content-lg-end">
                            <div class="col-sm-12 col-md-12 col-lg-6 col-xl-5 col-xxl-4 bg-section-login pd-sm-4 p-md-7 p-sm-6 p-4 d-flex flex-column justify-content-between h-85 h-sm-100 scroll-y">
                                <div class="col-4">
                                    <img src="/static/logo-robox-maker.png" class="img-fluid" />
                                </div>
                                <div>
                                    <h1 class="text-white noir-bold is-size-48 lh-58">{lang::dict("Create Account")}</h1>
                                    <div class="d-flex flex-row mt-3">
                                        <span class="text-white noir-light is-size-18 lh-22">{lang::dict("Do you already have an account?")}</span>
                                        <a onclick={&on_login_page_mode}>
                                            <span class="text-cyan-sky noir-bold is-size-18 lh-22 mx-2">{lang::dict("Login")}</span>
                                        </a>
                                    </div>
                                </div>
                                <span class="text-cyan-turquesa noir-bold is-size-20 lh-25 mt-5 mb-3">{lang::dict("Account Information")}</span>
                                <div>
                                    <label class="form-label text-white noir-bold is-size-16 lh-20">{lang::dict("License")}</label>
                                    <div class="input-group input-group-disabled">
                                        <span class="input-group-text">
                                            <img src="./icons/id-card.svg" style="height: 18px;" />
                                        </span>
                                        <input type="text" class="form-control input-style-l reset-radius-l form-control-disabled" minlength="15" value={self.license.clone()} maxlength="15" required=true disabled={true} />
                                        <span class="input-group-text reset-box-icon-l">
                                            <i class="fas fa-check"></i>
                                        </span>
                                    </div>
                                </div>
                                {on_license_verified}
                                {section_signup_2}
                            </div>
                        </div>
                    </div>
                }
            }
            PageModeSignUp::SelectInstitution => {
                html! {
                    <div class="card bg-dark text-white h-100 w-100">
                        <img src="/static/arte-4.png" class="card-img" alt="..." style="height: 100vh; width: 100vw; object-fit: cover;" />
                        <div class="card-img-overlay d-flex justify-content-start justify-content-sm-center justify-content-md-end justify-content-lg-end">
                            <div class="col-sm-12 col-md-12 col-lg-6 col-xl-5 col-xxl-4 bg-section-login pd-sm-4 p-md-7 p-sm-6 p-4 d-flex flex-column justify-content-between h-85 h-sm-100">
                                <div class="col-4">
                                    <img src="/static/logo-robox-maker.png" class="img-fluid" />
                                </div>
                                <div>
                                    <h1 class="text-white noir-bold is-size-48 lh-58">{lang::dict("Create Account")}</h1>
                                    <div class="d-flex flex-row mt-3">
                                        <span class="text-white noir-light is-size-18 lh-22">{lang::dict("Do you already have an account?")}</span>
                                        <a onclick={&on_login_page_mode}>
                                            <span class="text-cyan-sky noir-bold is-size-18 lh-22 mx-2">{lang::dict("Login")}</span>
                                        </a>
                                    </div>
                                </div>
                                <span class="text-cyan-turquesa noir-bold is-size-20 lh-25 mt-5 mb-3">{lang::dict("Institution Information")}</span>
                                <div>
                                    <label class="form-label text-white noir-bold is-size-16 lh-20">{lang::dict("School")}</label>
                                    <div class="input-group input-group-disabled">
                                        <span class="input-group-text">
                                            <img src="./icons/school.svg" style="height: 23px;" />
                                        </span>
                                        <input type="text" class="form-control input-style-l form-control-disabled" value={self.school_name.clone()} disabled=true />
                                    </div>
                                </div>
                                <div>
                                    <label class="form-label text-white noir-bold is-size-16 lh-20">{lang::dict("Grade")}</label>
                                    <div class="input-group input-group-disabled">
                                        <span class="input-group-text">
                                            <img src="./icons/graduation.svg" style="height: 18px;" />
                                        </span>
                                        <input type="text" class="form-control input-style-l form-control-disabled" value={self.grade.clone()} disabled=true />
                                    </div>
                                </div>
                                {section}
                                {notifications}
                                <button onclick={&maybe_user_create} class="btn button-login w-100 mt-4 py-3">
                                    <span>{lang::dict("Continue")}</span>
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
