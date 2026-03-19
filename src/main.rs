#![recursion_limit = "512"]

use log::*;
use uuid::Uuid;
use yew::prelude::*;
use yew::format::Json;
use gloo_storage::Storage;
use yew::virtual_dom::VNode;
use code_location::code_location;
use web_sys::History;
use wasm_bindgen::JsValue;
use yew_router::{route::Route, service::RouteService, Switch};
use yew::services::{ fetch::FetchTask, storage::{Area, StorageService}};
// use yew::services::{ fetch::FetchTask, storage::{Area, StorageService}, timeout::TimeoutTask, TimeoutService};

use roboxmaker_home::home::Home;
use roboxmaker_menu::menu::Menu;
use roboxmaker_post::post::View;
use roboxmaker_login::login::LoginPage;
use roboxmaker_home::add_users::AddUsers; //Panel add user
use roboxmaker_home::home_staff::HomeStaff;
use roboxmaker_quizzes::quiz_page::QuizPage;
use roboxmaker_robot::robot_page::RobotPage;
use roboxmaker_myspace::my_space::MySpaceView;
use roboxmaker_lesson::lesson_page::LessonPage;
use roboxmaker_quiz_panel::quiz_panel::QuizzesPanel;
// use roboxmaker_classes::classes_page::ClassesPage;
use roboxmaker_degree::degree_content::DegreeContent;
use roboxmaker_lesson::lesson_page_view::LessonPageView;
use roboxmaker_schools::list_schools::ListOfSchoolsView;
use roboxmaker_whiteboard::whiteboard_view::WhiteboardPage;
use roboxmaker_teacher_resource::tr_page::TeacherResourcePage;
use auth::{AccessTokenRequest, RefreshTokenRequest, TokenRequest};
use roboxmaker_degree::degree_content_student::DegreeContentStudent;
use roboxmaker_degree::degree_list::{DegreeList, ListOfGradesFilter};
use roboxmaker_graphql::{GraphQLService, GraphQLTask, Subscribe, SubscriptionTask, Request, RequestTask};
use roboxmaker_meetings::{meetings_view::MeetingsView, meet_room::MeetPage, direct_meet_room::DirectMeetingRoom};
use roboxmaker_types::types::{AppRoute, ClassGroupCategory, ClassesId, GroupId, Groups, LessonId, MeetingsId, MyUserProfile, PageMode, PostId, ResourceId, RobotId, SchoolId, Schools, UserId};

// Components
use roboxmaker_models::*;

pub struct App {
    route_service: RouteService<()>,
    login_status: i32,
    route: Route<()>,
    link: ComponentLink<Self>,
    task_school: Option<RequestTask>,
    task_token: Option<FetchTask>,
    auth_user: Option<user_model::user_by_id::ResponseData>,
    auth_school: Option<school_model::school_by_id::SchoolByIdSchoolByPk>,
    auth: Option<(auth::Auth, auth::Token)>,
    school_id: Option<SchoolId>,
    graphql_task: Option<GraphQLTask>,
    user_profile_sub: Option<SubscriptionTask>,
    saved_sidebar_state: bool,
    user_profile: Option<MyUserProfile>,

    req_schools_task: Option<RequestTask>,
    req_class_group_task: Option<RequestTask>,
    general_data_schools: Vec<Schools>,
    general_data_groups: Vec<Groups>,
}

#[derive(Debug)]
pub enum Message {
    AuthUser(Option<user_model::user_by_id::ResponseData>),
    AuthSchool(Option<school_model::school_by_id::ResponseData>),
    ChangeRoute(AppRoute),
    RouteChanged(Route<()>),
    AuthToken(Option<(auth::Auth, auth::Token)>, bool),
    LoginWithUsernamePassword(String, String),
    LoginWithRefreshToken(String),
    FetchUserById,
    LogOut,


    FetchSchools,
    SchoolList(Option<general_model::get_schools::ResponseData>),
    FetchClassGroups(Vec<Uuid>),
    ClassGroups(Option<general_model::groups_by_school_id::ResponseData>),
}

impl App {}

impl Component for App {
    type Message = Message;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        log::set_max_level(LevelFilter::Info);

        let mut route_service: RouteService<()> = RouteService::new();

        let auth = StorageService::new(Area::Local).ok().and_then(|storage| {
            let auth: Option<auth::Auth> = storage
                .restore::<Json<anyhow::Result<auth::Auth>>>(auth::AKER_AUTH_KEY)
                .0
                .ok();
            let token: Option<auth::Token> = storage
                .restore::<Json<anyhow::Result<auth::Token>>>(auth::AKER_TOKEN_KEY)
                .0
                .ok();
            auth.zip(token)
        });
        // Obtener la ruta actual del navegador ANTES de cualquier lógica
        let route = route_service.get_route();
        let callback = link.callback(Message::RouteChanged);
        route_service.register_callback(callback);

        if let Some((auth, _)) = &auth {
            // Hay token guardado, intentar refrescar
            link.send_message(Message::LoginWithRefreshToken(auth.refresh_token.clone()));
            // NO cambiar la ruta aquí - mantener la ruta actual del navegador
            info!("[ROUTE] Init with auth, current route: {:?}", AppRoute::switch(route.clone()));
        } else {
            // No hay token, verificar si necesitamos redirigir
            let current_app_route = AppRoute::switch(route.clone());
            info!("[ROUTE] Init without auth, current route: {:?}", current_app_route);

            match current_app_route {
                Some(AppRoute::Login) => {
                    // Ya estamos en Login, no hacer nada
                    info!("[ROUTE] Already on Login route");
                }
                Some(AppRoute::Landing) => {
                    // Ya estamos en Landing, no hacer nada
                    info!("[ROUTE] Already on Landing route");
                }
                _ => {
                    // Cualquier otra ruta sin auth, redirigir a Landing
                    info!("[ROUTE] Redirecting to Landing");
                    link.send_message(Message::ChangeRoute(AppRoute::Landing));
                }
            }
        }
        let saved_sidebar_state = if let Ok(value) = gloo_storage::LocalStorage::get("saved_sidebar_state") {
            value 
        } else {
            true
        };

        App {
            route_service,
            route,
            link,
            school_id: None,
            login_status: 0,
            task_school: None,
            task_token: None,
            auth_user: None,
            auth_school: None,
            auth: None,
            graphql_task: None,
            user_profile_sub: None,
            saved_sidebar_state,
            user_profile: None,

            req_schools_task: None,
            req_class_group_task: None,
            general_data_schools: vec![],
            general_data_groups: vec![],
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        info!("{:?}", msg);
        let mut should_render = true;
        match msg {
            Message::RouteChanged(route) => self.route = route,
            Message::ChangeRoute(route) => {
                let new_route: Route<()> = route.into();
                // Solo actualizar si la ruta es diferente
                if self.route != new_route {
                    let target_path = new_route.route.clone();

                    info!("[ROUTE] =====================================");
                    info!("[ROUTE] ChangeRoute: Navigating to {}", target_path);
                    info!("[ROUTE] Old route: {}", self.route.route);

                    // Actualizar la URL DEL NAVEGADOR PRIMERO antes de cambiar self.route
                    let window = gloo_utils::window();
                    let location = window.location();

                    // Mostrar URL actual ANTES del cambio
                    if let Ok(before_href) = location.href() {
                        info!("[ROUTE] BEFORE - location.href: {}", before_href);
                    }
                    if let Ok(before_pathname) = location.pathname() {
                        info!("[ROUTE] BEFORE - location.pathname: {}", before_pathname);
                    }

                    if let Ok(history) = window.history() {
                        // Usar pushState con solo el path (no URL completa)
                        info!("[ROUTE] Calling history.push_state_with_url(null, '', '{}')", target_path);

                        match history.push_state_with_url(&JsValue::NULL, "", Some(&target_path)) {
                            Ok(_) => {
                                info!("[ROUTE] ✓ push_state executed successfully");

                                // Verificar inmediatamente después
                                if let Ok(after_href) = location.href() {
                                    info!("[ROUTE] AFTER - location.href: {}", after_href);
                                }
                                if let Ok(after_pathname) = location.pathname() {
                                    info!("[ROUTE] AFTER - location.pathname: {}", after_pathname);
                                }
                            }
                            Err(e) => {
                                info!("[ROUTE] ✗ Error in push_state: {:?}", e);
                            }
                        }
                    } else {
                        info!("[ROUTE] ✗ Could not get window.history()");
                    }

                    // AHORA actualizar el estado interno
                    self.route = new_route;
                    info!("[ROUTE] Internal route updated to: {}", self.route.route);
                    info!("[ROUTE] =====================================");
                }
            }
            Message::AuthUser(response) => {
                self.auth_user = response.clone();

                self.user_profile = response.clone()
                    .and_then(|data| data.user_by_pk)
                    .and_then(|item| {
                        Some(MyUserProfile {
                            email: item.user_profile.clone().and_then(|d|d.email).unwrap_or("example.123@gmail.co".to_string()),
                            full_name: item.user_profile.clone().and_then(|d|Some(d.full_name)).unwrap_or("".to_string()),
                            pic_path: item.user_profile.clone().and_then(|d|d.pic_path).unwrap_or("/static/avatar.png".to_string()),
                            user_id: roboxmaker_types::types::UserId(item.user_profile.clone().and_then(|d|Some(d.user_id)).unwrap_or(Uuid::default())),
                            school_name: item.user_profile.clone().and_then(|d|d.group_member).and_then(|group_member|group_member.school_group).and_then(|school_group|Some(school_group.school)).and_then(|school|school.school_profile).and_then(|school_profile|Some(school_profile.name)).unwrap_or("".to_string()),
                            user_student: item.user_student.and_then(|d|Some(UserId(d.user_id))),
                            user_teacher: item.user_teacher.and_then(|d|Some(UserId(d.user_id))),
                            user_staff: item.user_staff.and_then(|d|Some(UserId(d.user_id))),
                            license: item.license.and_then(|d|Some(d.license)).unwrap_or("AAAAAAAAAAAAAAA".to_string()),
                            group_member_id: roboxmaker_types::types::GroupId(item.group_member.and_then(|d|Some(d.group_id)).unwrap_or(Uuid::default())),
                        })
                    });

                if self.user_profile.clone().and_then(|item| item.user_staff).is_some() ||
                    self.user_profile.clone().and_then(|item| item.user_teacher).is_some() {
                    self.link.send_message(Message::FetchSchools);
                }
            }
            Message::AuthSchool(school) => {
                self.auth_school = school.clone().and_then(|data| data.school_by_pk);
                if school.clone().and_then(|data| data.school_by_pk).is_some() {
                    self.school_id = Some(SchoolId(school.clone().and_then(|data| data.school_by_pk).unwrap().id));
                }
            }
            Message::FetchUserById => {
                if let Some((_auth, token)) = &self.auth {
                    if let Some(graphql_task) = self.graphql_task.as_mut() {
                        let vars = user_model::user_by_id::Variables {
                            user_id: token.claims.user_id,
                        };
                        let task = user_model::UserById::subscribe(
                            graphql_task,
                            &self.link, 
                            vars, 
                            |response| {
                                Message::AuthUser(response)
                            },
                        );
                        self.user_profile_sub = Some(task);
                    }
                }
            }
            Message::AuthToken(auth, should_notify) => {
                if let Ok(mut storage) = StorageService::new(Area::Local) {
                    auth.as_ref()
                        .and_then(|(auth, _)| Some(storage.store(auth::AKER_AUTH_KEY, Json(auth))));
                    auth.as_ref().and_then(|(_, token)| {
                        Some(storage.store(auth::AKER_TOKEN_KEY, Json(token)))
                    });
                }
                self.auth = auth;

                if let Some((_auth, token)) = &self.auth {
                    self.login_status = 0;
                    self.link.send_message(Message::FetchUserById);
                    self.graphql_task = Some(GraphQLService::connect(&code_location!()));

                    if let Some(graphql_task) = self.graphql_task.as_mut() {
                        let vars = school_model::school_by_id::Variables {
                            school_id: token.claims.school_id,
                        };
                        let task = school_model::SchoolById::request(
                            graphql_task,
                            &self.link, 
                            vars, 
                            |response| {
                                Message::AuthSchool(response)
                            },
                        );
                        self.task_school = Some(task);
                    }

                    // Solo redirigir a Home si estamos en Login, Landing o ruta inválida
                    let current_app_route = AppRoute::switch(self.route.clone());
                    info!("[ROUTE] AuthToken - Current route: {:?}", current_app_route);
                    if matches!(current_app_route, Some(AppRoute::Login) | Some(AppRoute::Landing) | None) {
                        info!("[ROUTE] Redirecting to Home");
                        self.link.send_message(Message::ChangeRoute(AppRoute::Home));
                    } else {
                        info!("[ROUTE] Keeping current route");
                    }
                    // Si estamos en cualquier otra ruta válida, mantenerla

                } else {
                    if should_notify {
                        self.login_status = 204;
                    }

                    self.link.send_message(Message::LogOut)
                }
            }
            Message::LoginWithUsernamePassword(username, password) => {
                should_render = false;
                let token_req = TokenRequest::AccessToken(AccessTokenRequest {
                    grant_type: "password".to_string(),
                    client_id: "app-aker".to_string(),
                    username: username,
                    password: password,
                    scope: "openid".to_string(),
                });
                let task_token = auth::fetch_token(&self.link, token_req, |response| {
                    Message::AuthToken(response, true)
                });

                self.task_token = task_token.ok();
            }
            Message::LoginWithRefreshToken(refresh_token) => {
                should_render = false;
                let token_req = TokenRequest::RefreshToken(RefreshTokenRequest {
                    grant_type: "refresh_token".to_string(),
                    client_id: "app-aker".to_string(),
                    refresh_token: refresh_token,
                    scope: "openid".to_string(),
                });
                let task_token = auth::fetch_token(&self.link, token_req, |response| {
                    Message::AuthToken(response, false)
                });

                self.task_token = task_token.ok()
            }
            Message::LogOut => {
                StorageService::new(Area::Local)
                    .as_mut()
                    .ok()
                    .and_then(|storage| Some(storage.remove(auth::AKER_AUTH_KEY)));
                StorageService::new(Area::Local)
                    .as_mut()
                    .ok()
                    .and_then(|storage| Some(storage.remove(auth::AKER_TOKEN_KEY)));
                    
                self.link.send_message(Message::ChangeRoute(AppRoute::Landing));

                self.user_profile = None;
                self.auth_user = None;
                self.auth_school = None;
                self.school_id = None;
                self.graphql_task = None;
                self.user_profile_sub = None;
                self.task_school = None;
                self.task_token = None;
                self.auth = None;
            }

            Message::FetchSchools => {
                if let Some(graphql_task) = self.graphql_task.as_mut() {
                    let vars = general_model::get_schools::Variables {};

                    let task = general_model::GetSchools::request(
                        graphql_task,
                        &self.link,
                        vars,
                        |response| {
                            Message::SchoolList(response)
                        },
                    );
                    self.req_schools_task = Some(task);
                }
            }
            Message::SchoolList(response) => {
                // info!("QuizzesPanelGeneralDataSchools: {:?}", response);
                if response.is_some() {

                    self.general_data_schools = response
                        .clone()
                        .and_then(|data| Some(data.inventory_group))
                        .unwrap_or(vec![])
                        .iter()
                        .map(|schools| {
                            let school_group = schools.school_group.clone();
                            let school = school_group.clone().and_then(|data| Some(data.school));
                            let school_profile = school.clone().and_then(|data| data.school_profile);
                            let name = school_profile.clone().and_then(|data| Some(data.name)).unwrap_or("".to_string());
                            let inventory_group = school_group.clone().and_then(|data| Some(data.group_id)).unwrap_or(Uuid::default());
                            let school_id = school_group.clone().and_then(|data| Some(data.school_id)).unwrap_or(Uuid::default());
                            Schools {
                                name,
                                inventory_group,
                                school_id: SchoolId(school_id),
                            }
                        }).collect();
                    let schools_ids = self
                        .general_data_schools
                        .iter()
                        .map(|item| item.school_id.0)
                        .collect::<Vec<Uuid>>();
        
                    self.link.send_message(Message::FetchClassGroups(schools_ids));
                }
            }
            Message::FetchClassGroups(school_ids) => {
                if let Some(graphql_task) = self.graphql_task.as_mut() {
                    let vars = general_model::groups_by_school_id::Variables {
                        school_ids,
                    };

                    let task = general_model::GroupsBySchoolId::request(
                        graphql_task,
                        &self.link,
                        vars,
                        |response| {
                            Message::ClassGroups(response)
                        },
                    );
                    self.req_class_group_task = Some(task);
                }
            },
            Message::ClassGroups(response) => {
                // info!("QuizzesPanelGeneralDataGroups: {:?}", response);
                if response.is_some() {
                    self.general_data_groups = response
                        .clone()
                        .and_then(|data| Some(data.class_group))
                        .unwrap_or(vec![])
                        .iter()
                        .map(|class_group| {
                            let class_name = class_group.class_profile.clone().and_then(|data| Some(data.name)).unwrap_or("".to_string());
                            let group_id = class_group.group_id;

                            let school_id = class_group.school_group.clone().and_then(|item| Some(item.school_id)).unwrap_or_default();
                            
                            Groups {
                                class_name,
                                group_id: GroupId(group_id),
                                school_id,
                            }
                        })
                        .collect();
                }
            }
        }
        should_render
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        let on_logout = self.link.callback(move |_| Message::LogOut);

        let on_app_route = self.link.callback(move |route: AppRoute| Message::ChangeRoute(route));

        let on_app_login = self.link.callback(|(username, password)| {Message::LoginWithUsernamePassword(username, password)});

        let auth_user_id = self.user_profile.as_ref().and_then(|data| Some(data.user_id)).unwrap_or(UserId(Uuid::default()));
        
        let pic_path = self.user_profile.clone().and_then(|data| Some(data.pic_path)).unwrap_or("/static/avatar.png".to_string());

        let full_name = self.user_profile.clone().and_then(|data| Some(data.full_name)).unwrap_or("".to_string());

        let school_id = self.auth_school.clone().and_then(|data| Some(data.id)).unwrap_or(Uuid::default());

        let app_page = match AppRoute::switch(self.route.clone()) {
            Some(AppRoute::Landing) => {
                html!{
                    <iframe allow="camera; microphone; fullscreen; display-capture" src="https://landing.roboxmaker.com/"
                        style="height: 100%; width: 100%; border: 0px; padding: 0px, margin: 0px;"></iframe>
                }
            }
            Some(AppRoute::SchoolGroupSection(school_id, group_id, category)) => {
                html! { 
                    <DegreeContent
                        group_id=group_id
                        route=self.route.clone()
                        school_id=school_id
                        category=category
                        auth_school=self.auth_school.clone()
                        user_profile=self.user_profile.clone() 
                        on_app_route=on_app_route.clone()
                        saved_sidebar_state=self.saved_sidebar_state.clone() />
                }
            }
            Some(AppRoute::GroupSectionStudent(school_id, user_id, category)) => {
                html! { 
                    <DegreeContentStudent
                        user_id=user_id
                        route=self.route.clone()
                        school_id=school_id
                        category=category
                        auth_school=self.auth_school.clone()
                        user_profile=self.user_profile.clone() 
                        on_app_route=on_app_route.clone()
                        saved_sidebar_state=self.saved_sidebar_state.clone() />
                }
            }
            Some(AppRoute::PanelAddUsers) => {
                let maybe_option_user = self.user_profile.as_ref().and_then(|user| {
                    if user.user_staff.is_some() {
                        Some(html! {
                            <>
                                <AddUsers user_id={user.user_id}
                                    user_profile=self.user_profile.clone()
                                    auth_school=self.auth_school.clone()
                                    on_app_route=on_app_route.clone() 
                                    saved_sidebar_state=self.saved_sidebar_state.clone() />
                            </>
                        })
                    } else {
                        Some(html! {})
                    }
                }).unwrap_or(html! {});
                html! { 
                    {maybe_option_user} 
                }
            }
            Some(AppRoute::Schools) => {
                html! { 
                    <>
                        <ListOfSchoolsView
                            auth_school=self.auth_school.clone()
                            user_profile=self.user_profile.clone()
                            on_app_route=on_app_route.clone()
                            pic_path=pic_path.clone()
                            saved_sidebar_state=self.saved_sidebar_state.clone() />
                    </>    
                }
            }
            Some(AppRoute::GradesBySchoolId(school_id)) => {
                html! { 
                    <>
                        <DegreeList
                            user_profile=self.user_profile.clone()
                            on_app_route=on_app_route.clone()
                            school_id=school_id.clone()
                            filter=ListOfGradesFilter::SchoolGroups />
                    </>
                }
            }
            Some(AppRoute::GradesByUserId(school_id)) => {
                html! { 
                    <>
                        <DegreeList
                            user_profile=self.user_profile.clone()
                            on_app_route=on_app_route.clone()
                            school_id=school_id.clone()
                            filter=ListOfGradesFilter::UserGroups(auth_user_id)/>
                    </>
                }
            }
            Some(AppRoute::MySpace(user_id)) => {
                html! { <MySpaceView user_id=user_id
                    school_id=self.school_id
                    user_profile=self.user_profile.clone() 
                    on_app_route=on_app_route.clone()
                    saved_sidebar_state=self.saved_sidebar_state.clone() />
                }
            }
            Some(AppRoute::Login) => html! {
                <>
                    <LoginPage on_app_login=on_app_login.clone()
                        login_status=self.login_status />
                </>
            },
            Some(AppRoute::Robot(robot_id, group_id, user_id)) => {
                html! { <RobotPage on_app_route=on_app_route.clone() 
                    user_profile=self.user_profile.clone() 
                    user_id=Some(user_id) 
                    robot_id=robot_id 
                    group_id=group_id
                    pic_path=pic_path
                    saved_sidebar_state=self.saved_sidebar_state.clone() />
                }
            }
            Some(AppRoute::Lesson(school_id, group_id, lesson_id)) => {
                html! { <LessonPage user_profile=self.user_profile.clone() 
                    on_app_route=on_app_route.clone() 
                    lesson_id=lesson_id 
                    school_id=school_id
                    group_id=group_id /> 
                }
            }
            Some(AppRoute::LessonView(school_id, group_id, lesson_id)) => {
                html! { <LessonPageView user_profile=self.user_profile.clone() 
                    on_app_route=on_app_route.clone() 
                    lesson_id=lesson_id 
                    group_id=group_id
                    school_id=school_id
                    saved_sidebar_state=self.saved_sidebar_state.clone() /> 
                }
            }
            Some(AppRoute::Post(school_id, group_id, post_id, page_mode)) => {
                html! { <View user_profile=self.user_profile.clone() 
                    on_app_route=on_app_route.clone() 
                    post_id=post_id 
                    school_id=school_id
                    group_id=group_id
                    page_mode=page_mode
                    saved_sidebar_state=self.saved_sidebar_state.clone() />
                }
            }
            // Some(AppRoute::Classes(school_id, group_id, classes_id)) => {
            //     html! { 
            //         <ClassesPage user_profile=self.user_profile.clone() 
            //             on_app_route=on_app_route.clone() 
            //             school_id=school_id
            //             classes_id=classes_id 
            //             group_id=group_id />
            //     }
            // }
            Some(AppRoute::Resource(school_id, group_id, resource_id)) => {
                html! { 
                    <TeacherResourcePage user_profile=self.user_profile.clone() 
                        on_app_route=on_app_route.clone() 
                        school_id=school_id
                        resource_id=resource_id 
                        group_id=group_id />
                }
            }
            Some(AppRoute::Quizzes(school_id, group_id, quiz_id)) => {
                html! { 
                    <QuizPage 
                    user_profile=self.user_profile.clone() 
                    on_app_route=on_app_route.clone() 
                    quiz_id=quiz_id 
                    school_id=school_id
                    group_id=group_id
                    saved_sidebar_state=self.saved_sidebar_state.clone() />
                }
            }
            Some(AppRoute::Meet(group_id, meetings_id)) => {
                html! {<MeetPage user_profile=self.user_profile.clone() 
                    group_id=group_id
                    meetings_id=meetings_id.clone() />
                }
            }
            Some(AppRoute::MeetDirect(group_id)) => {
                html! {<DirectMeetingRoom user_profile=self.user_profile.clone() 
                    group_id=group_id />
                }
            }
            Some(AppRoute::Whiteboard(whiteboard_id)) => {
                html! { <WhiteboardPage user_profile=self.user_profile.clone() 
                    whiteboard_id=whiteboard_id />
                }
            }
            Some(AppRoute::Meetings) => {
                html! { 
                    <MeetingsView 
                        user_profile=self.user_profile.clone()
                        auth_school=self.auth_school.clone()
                        on_app_route=on_app_route.clone()
                        pic_path=pic_path.clone()
                        full_name=full_name.clone()
                        saved_sidebar_state=self.saved_sidebar_state.clone() />
                }
            }
            Some(AppRoute::Home) => {
                let maybe_option_view_home = self.user_profile.as_ref().and_then(|user| {
                    if user.user_staff.is_some() {
                        Some(html! {
                            <>
                                <HomeStaff user_id={user.user_id}
                                    user_profile=self.user_profile.clone()
                                    auth_school=self.auth_school.clone()
                                    on_app_route=on_app_route.clone() 
                                    saved_sidebar_state=self.saved_sidebar_state.clone() />
                            </>
                        })
                    } else {
                        Some(html! {
                            <>
                                <Home user_id={user.user_id}
                                    user_profile=self.user_profile.clone()
                                    auth_school=self.auth_school.clone()
                                    on_app_route=on_app_route.clone()
                                    school_id=SchoolId(school_id)
                                    saved_sidebar_state=self.saved_sidebar_state.clone() />
                            </>
                        })
                    }
                }).unwrap_or(html! {});
                html! {
                    <>  
                        {maybe_option_view_home}
                    </>
                }
            },
            Some(AppRoute::QuizzesPanel) => {
                self.user_profile.as_ref().and_then(|user| {
                    if user.user_staff.is_some() || user.user_teacher.is_some() {
                        Some(html! {
                            <QuizzesPanel on_app_route={ on_app_route.clone() }
                                user_profile={ self.user_profile.clone() }
                                general_data_schools={ self.general_data_schools.clone() }
                                general_data_groups={ self.general_data_groups.clone() } />
                        })
                    } else {
                        None
                    }
                }).unwrap_or(html! {})
            },
            None => VNode::from("404"),
        };

        let maybe_app_page = match AppRoute::switch(self.route.clone()) {
            Some(AppRoute::Login) 
            | Some(AppRoute::Landing) => app_page,
            _ => {
                if self.auth_user.is_some() && self.user_profile.is_some() && self.auth_school.is_some() {
                    app_page
                } else {
                    html! {
                        <progress class="progress is-small is-primary" max="100"></progress>
                    }
                }
            }
        };

        let maybe_menu = match AppRoute::switch(self.route.clone()) {
            Some(AppRoute::Login) => html! {},
            Some(AppRoute::GradesByUserId(school_id)) => html! {
                <Menu route=self.route.clone() 
                    user_profile=self.user_profile.clone() 
                    auth_school=self.auth_school.clone()
                    on_app_route={on_app_route.clone()}
                    category=ClassGroupCategory::Posts
                    group_id=GroupId(Uuid::default())
                    post_id=PostId(Uuid::default()) 
                    robot_id=RobotId(Uuid::default())
                    user_id=UserId(Uuid::default()) 
                    lesson_id=LessonId(Uuid::default()) 
                    classes_id=ClassesId(Uuid::default())
                    meetings_id=MeetingsId(Uuid::default())
                    resource_id=ResourceId(Uuid::default())
                    school_id=school_id.clone()
                    on_logout={&on_logout}
                    page_mode=PageMode::Edit
                    quiz_id={Uuid::default()} />
            },
            Some(AppRoute::GradesBySchoolId(school_id)) => html! {
                <Menu route=self.route.clone() 
                    user_profile=self.user_profile.clone() 
                    auth_school=self.auth_school.clone()
                    on_app_route={on_app_route.clone()}
                    category=ClassGroupCategory::Posts
                    group_id=GroupId(Uuid::default())
                    post_id=PostId(Uuid::default()) 
                    robot_id=RobotId(Uuid::default())
                    user_id=UserId(Uuid::default()) 
                    lesson_id=LessonId(Uuid::default()) 
                    classes_id=ClassesId(Uuid::default())
                    meetings_id=MeetingsId(Uuid::default())
                    resource_id=ResourceId(Uuid::default())
                    school_id=school_id.clone()
                    on_logout={&on_logout}
                    page_mode=PageMode::Edit
                    quiz_id={Uuid::default()} />
            },
            Some(AppRoute::SchoolGroupSection(school_id, group_id, category)) => html! {
                <Menu route=self.route.clone() 
                    user_profile=self.user_profile.clone() 
                    auth_school=self.auth_school.clone()
                    on_app_route={on_app_route.clone()}
                    category=category.clone()
                    group_id=group_id.clone()
                    post_id=PostId(Uuid::default()) 
                    robot_id=RobotId(Uuid::default())
                    user_id=UserId(Uuid::default()) 
                    lesson_id=LessonId(Uuid::default()) 
                    classes_id=ClassesId(Uuid::default())
                    meetings_id=MeetingsId(Uuid::default())
                    resource_id=ResourceId(Uuid::default())
                    school_id=school_id.clone()
                    on_logout={&on_logout}
                    page_mode=PageMode::Edit
                    quiz_id={Uuid::default()} />
            },
            Some(AppRoute::GroupSectionStudent(school_id, user_id, category)) => html! {
                <Menu route=self.route.clone() 
                    user_profile=self.user_profile.clone() 
                    auth_school=self.auth_school.clone()
                    on_app_route={on_app_route.clone()}
                    category=category.clone()
                    group_id=GroupId(Uuid::default())
                    post_id=PostId(Uuid::default()) 
                    robot_id=RobotId(Uuid::default())
                    user_id=user_id
                    lesson_id=LessonId(Uuid::default()) 
                    classes_id=ClassesId(Uuid::default())
                    meetings_id=MeetingsId(Uuid::default())
                    resource_id=ResourceId(Uuid::default())
                    school_id=school_id.clone()
                    on_logout={&on_logout}
                    page_mode=PageMode::Edit
                    quiz_id={Uuid::default()} />
            },
            Some(AppRoute::Schools) => html! {
                <Menu route=self.route.clone() 
                    user_profile=self.user_profile.clone() 
                    auth_school=self.auth_school.clone()
                    on_app_route={on_app_route.clone()}
                    category=ClassGroupCategory::Posts
                    group_id=GroupId(Uuid::default())
                    post_id=PostId(Uuid::default()) 
                    robot_id=RobotId(Uuid::default())
                    user_id=UserId(Uuid::default()) 
                    lesson_id=LessonId(Uuid::default()) 
                    classes_id=ClassesId(Uuid::default())
                    meetings_id=MeetingsId(Uuid::default())
                    school_id=SchoolId(Uuid::default())
                    resource_id=ResourceId(Uuid::default())
                    on_logout={&on_logout}
                    page_mode=PageMode::Edit
                    quiz_id={Uuid::default()} />
            },
            Some(AppRoute::Meet(group_id, meetings_id)) => html! {
                <Menu route=self.route.clone() 
                    user_profile=self.user_profile.clone() 
                    auth_school=self.auth_school.clone()
                    on_app_route={on_app_route.clone()}
                    category=ClassGroupCategory::Posts
                    post_id=PostId(Uuid::default()) 
                    robot_id=RobotId(Uuid::default())
                    user_id=UserId(Uuid::default()) 
                    lesson_id=LessonId(Uuid::default()) 
                    classes_id=ClassesId(Uuid::default())
                    group_id=group_id.clone() 
                    meetings_id=meetings_id.clone()
                    school_id=SchoolId(Uuid::default())
                    resource_id=ResourceId(Uuid::default())
                    on_logout={&on_logout}
                    page_mode=PageMode::Edit
                    quiz_id={Uuid::default()} />
            },
            Some(AppRoute::MeetDirect(group_id)) => html! {
                <Menu route=self.route.clone() 
                    user_profile=self.user_profile.clone() 
                    auth_school=self.auth_school.clone()
                    on_app_route={on_app_route.clone()}
                    category=ClassGroupCategory::Posts
                    post_id=PostId(Uuid::default()) 
                    robot_id=RobotId(Uuid::default())
                    user_id=UserId(Uuid::default()) 
                    lesson_id=LessonId(Uuid::default()) 
                    classes_id=ClassesId(Uuid::default())
                    group_id=group_id.clone() 
                    meetings_id=MeetingsId(Uuid::default())
                    school_id=SchoolId(Uuid::default())
                    resource_id=ResourceId(Uuid::default())
                    on_logout={&on_logout}
                    page_mode=PageMode::Edit
                    quiz_id={Uuid::default()} />
            },
            Some(AppRoute::MySpace(user_id)) => html! {
                <Menu route=self.route.clone() 
                user_profile=self.user_profile.clone() 
                auth_school=self.auth_school.clone()
                on_app_route={on_app_route.clone()}
                category=ClassGroupCategory::Posts
                group_id=GroupId(Uuid::default())
                post_id=PostId(Uuid::default()) 
                robot_id=RobotId(Uuid::default())
                user_id=user_id.clone()
                lesson_id=LessonId(Uuid::default()) 
                classes_id=ClassesId(Uuid::default()) 
                meetings_id=MeetingsId(Uuid::default())
                school_id=SchoolId(Uuid::default())
                resource_id=ResourceId(Uuid::default())
                on_logout={&on_logout}
                page_mode=PageMode::Edit
                quiz_id={Uuid::default()} />
            },
            // Some(AppRoute::Classes(school_id, group_id, classes_id)) => html! {
            //     <Menu route=self.route.clone() 
            //         user_profile=self.user_profile.clone() 
            //         auth_school=self.auth_school.clone()
            //         on_app_route={on_app_route.clone()}
            //         category=ClassGroupCategory::Classes
            //         group_id=group_id.clone()
            //         post_id=PostId(Uuid::default()) 
            //         robot_id=RobotId(Uuid::default())
            //         user_id=UserId(Uuid::default()) 
            //         lesson_id=LessonId(Uuid::default()) 
            //         meetings_id=MeetingsId(Uuid::default())
            //         resource_id=ResourceId(Uuid::default())
            //         classes_id=classes_id.clone() 
            //         school_id=school_id
            //         on_logout={&on_logout}
            //         page_mode=PageMode::Edit
            //         quiz_id={Uuid::default()} />
            // },
            Some(AppRoute::Lesson(school_id, group_id, lesson_id)) => html! {
                <Menu route=self.route.clone() 
                    user_profile=self.user_profile.clone() 
                    auth_school=self.auth_school.clone()
                    on_app_route={on_app_route.clone()}
                    category=ClassGroupCategory::Lessons
                    group_id=group_id.clone()
                    post_id=PostId(Uuid::default()) 
                    robot_id=RobotId(Uuid::default())
                    user_id=UserId(Uuid::default()) 
                    lesson_id=lesson_id.clone() 
                    meetings_id=MeetingsId(Uuid::default())
                    classes_id=ClassesId(Uuid::default())
                    resource_id=ResourceId(Uuid::default())
                    school_id=school_id
                    on_logout={&on_logout}
                    page_mode=PageMode::Edit
                    quiz_id={Uuid::default()} />
            },
            Some(AppRoute::LessonView(school_id, group_id, lesson_id)) => html! {
                <Menu route=self.route.clone() 
                    user_profile=self.user_profile.clone() 
                    auth_school=self.auth_school.clone()
                    on_app_route={on_app_route.clone()}
                    category=ClassGroupCategory::Lessons
                    group_id=group_id.clone()
                    post_id=PostId(Uuid::default()) 
                    robot_id=RobotId(Uuid::default())
                    user_id=UserId(Uuid::default()) 
                    lesson_id=lesson_id.clone() 
                    meetings_id=MeetingsId(Uuid::default())
                    classes_id=ClassesId(Uuid::default()) 
                    resource_id=ResourceId(Uuid::default())
                    school_id=school_id
                    on_logout={&on_logout}
                    page_mode=PageMode::Edit
                    quiz_id={Uuid::default()} />
            },
            Some(AppRoute::Robot(robot_id, group_id, user_id)) => html! {
                <Menu route=self.route.clone() 
                    user_profile=self.user_profile.clone() 
                    auth_school=self.auth_school.clone()
                    on_app_route={on_app_route.clone()}
                    category=ClassGroupCategory::Robots
                    group_id=group_id.clone()
                    post_id=PostId(Uuid::default()) 
                    robot_id=robot_id.clone()
                    user_id=user_id.clone() 
                    lesson_id=LessonId(Uuid::default()) 
                    meetings_id=MeetingsId(Uuid::default())
                    classes_id=ClassesId(Uuid::default()) 
                    school_id=SchoolId(Uuid::default())
                    resource_id=ResourceId(Uuid::default())
                    on_logout={&on_logout}
                    page_mode=PageMode::Edit
                    quiz_id={Uuid::default()} />
            },
            Some(AppRoute::Post(school_id, group_id, post_id, page_mode)) => html! {
                <Menu route=self.route.clone() 
                    user_profile=self.user_profile.clone() 
                    auth_school=self.auth_school.clone()
                    on_app_route={on_app_route.clone()}
                    category=ClassGroupCategory::Posts
                    group_id=group_id.clone()
                    post_id=post_id.clone() 
                    robot_id=RobotId(Uuid::default())
                    user_id=UserId(Uuid::default()) 
                    lesson_id=LessonId(Uuid::default()) 
                    meetings_id=MeetingsId(Uuid::default())
                    classes_id=ClassesId(Uuid::default())
                    resource_id=ResourceId(Uuid::default())
                    school_id=school_id
                    on_logout={&on_logout}
                    page_mode=page_mode
                    quiz_id={Uuid::default()} />
            },
            Some(AppRoute::Quizzes(school_id, group_id, quiz_id)) => html! {
                <Menu route=self.route.clone() 
                    user_profile=self.user_profile.clone() 
                    auth_school=self.auth_school.clone()
                    on_app_route={on_app_route.clone()}
                    category=ClassGroupCategory::Posts
                    group_id=group_id.clone()
                    post_id=PostId(Uuid::default()) 
                    robot_id=RobotId(Uuid::default())
                    user_id=UserId(Uuid::default()) 
                    lesson_id=LessonId(Uuid::default()) 
                    meetings_id=MeetingsId(Uuid::default())
                    classes_id=ClassesId(Uuid::default())
                    resource_id=ResourceId(Uuid::default())
                    school_id=school_id
                    on_logout={&on_logout}
                    page_mode=PageMode::Edit
                    quiz_id={quiz_id} />
            },
            Some(AppRoute::Meetings) => html! {
                <Menu route=self.route.clone() 
                    user_profile=self.user_profile.clone() 
                    auth_school=self.auth_school.clone()
                    on_app_route={on_app_route.clone()}
                    category=ClassGroupCategory::Posts
                    group_id=GroupId(Uuid::default())
                    post_id=PostId(Uuid::default())
                    robot_id=RobotId(Uuid::default())
                    user_id=UserId(Uuid::default())  
                    meetings_id=MeetingsId(Uuid::default())
                    lesson_id=LessonId(Uuid::default()) 
                    classes_id=ClassesId(Uuid::default())
                    school_id=SchoolId(Uuid::default())
                    resource_id=ResourceId(Uuid::default())
                    on_logout={&on_logout}
                    page_mode=PageMode::Edit
                    quiz_id={Uuid::default()} />
            },
            Some(AppRoute::Resource(school_id, group_id , resource_id)) => html! {
                <Menu route=self.route.clone() 
                    user_profile=self.user_profile.clone() 
                    auth_school=self.auth_school.clone()
                    on_app_route={on_app_route.clone()}
                    category=ClassGroupCategory::Posts
                    group_id=group_id
                    post_id=PostId(Uuid::default())
                    robot_id=RobotId(Uuid::default())
                    user_id=UserId(Uuid::default())  
                    meetings_id=MeetingsId(Uuid::default())
                    lesson_id=LessonId(Uuid::default()) 
                    classes_id=ClassesId(Uuid::default())
                    school_id=school_id
                    resource_id=resource_id
                    on_logout={&on_logout}
                    page_mode=PageMode::Edit
                    quiz_id={Uuid::default()} />
            },
            _ => html! {
                <Menu route=self.route.clone() 
                    user_profile=self.user_profile.clone() 
                    auth_school=self.auth_school.clone()
                    on_app_route={on_app_route.clone()}
                    category=ClassGroupCategory::Posts
                    group_id=GroupId(Uuid::default())
                    post_id=PostId(Uuid::default())
                    robot_id=RobotId(Uuid::default()) 
                    user_id=auth_user_id.clone()
                    meetings_id=MeetingsId(Uuid::default())
                    lesson_id=LessonId(Uuid::default())
                    classes_id=ClassesId(Uuid::default()) 
                    school_id=SchoolId(Uuid::default())
                    resource_id=ResourceId(Uuid::default())
                    on_logout={&on_logout}
                    page_mode=PageMode::Edit
                    quiz_id={Uuid::default()} />
            },
        };

        html! {
            <div class="d-flex flex-column flex-sm-column flex-md-column flex-lg-row vh-100">
                { maybe_menu }
                { maybe_app_page }
            </div>
        }
    }
}

fn main() {
    // wasm_logger::init(wasm_logger::Config::new(log::Level::Info));
    info!("Starting Roboxmaker App");
    yew::start_app::<App>();
}