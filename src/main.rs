#![recursion_limit = "512"]

use log::*;
use roboxmaker_loaders::fullscreen_loader::FullScreenLoader;
use uuid::Uuid;
use yew::prelude::*;
// use yew::format::Json;
use roboxmaker_main::config;
use code_location::code_location;
use wasm_bindgen_futures::spawn_local;
use yew_router::{Switch, BrowserRouter};
use yew_router::scope_ext::RouterScopeExt;
use gloo_storage::{Storage, LocalStorage};
// use yew::services::{ fetch::FetchTask, storage::{Area, StorageService}};
// use yew::services::{ fetch::FetchTask, storage::{Area, StorageService}, timeout::TimeoutTask, TimeoutService};

use roboxmaker_home::home::Home;
use roboxmaker_menu::menu::Menu;
use roboxmaker_login::login::LoginPage;
use roboxmaker_post::post_page::PostPage;
// use roboxmaker_home::add_users::AddUsers; //Panel add user
use roboxmaker_home::home_staff::HomeStaff;
use roboxmaker_robot::robot_page::RobotPage;
use roboxmaker_myspace::my_space::MySpaceView;
use roboxmaker_lesson::lesson_page::LessonPage;
use roboxmaker_classes::classes_page::ClassesPage;
use roboxmaker_post::post_page_view::PostPageView;
use roboxmaker_degree::degree_content::DegreeContent;
use roboxmaker_lesson::lesson_page_view::LessonPageView;
use roboxmaker_schools::list_schools::ListOfSchoolsView;
use roboxmaker_whiteboard::whiteboard_view::WhiteboardPage;
use auth::{AccessTokenRequest, RefreshTokenRequest, TokenRequest};
use roboxmaker_degree::degree_content_student::DegreeContentStudent;
use roboxmaker_degree::degree_list::{DegreeList, ListOfGradesFilter};
use roboxmaker_types::types::{SchoolId, GroupId, UserId, AppRoute, MyUserProfile, DataSchoolProfile};
use roboxmaker_graphql::{GraphQLService, GraphQLTask, Subscribe, SubscriptionTask, Request, RequestTask};
use roboxmaker_meetings::{meetings_view::MeetingsView, meet_room::MeetPage, direct_meet_room::DirectMeetingRoom};

// Components
use roboxmaker_models::*;
use roboxmaker_utils::functions::{user_profile_data, school_profile_data};

pub struct App {
    login_status: i32,
    task_school: Option<RequestTask>,
    task_token: Option<RequestTask>,
    auth: Option<(auth::Auth, auth::Token)>,
    school_id: Option<SchoolId>,
    graphql_task: Option<GraphQLTask>,
    user_profile_sub: Option<SubscriptionTask>,
    user_profile: Option<MyUserProfile>,
    school_profile: Option<DataSchoolProfile>,
    view_login: bool,
}

#[derive(Debug)]
pub enum Message {
    AuthUser(Option<user_model::user_by_id::ResponseData>),
    AuthSchool(Option<school_model::school_by_id::ResponseData>),
    AuthToken(Option<(auth::Auth, auth::Token)>, bool),
    LoginWithUsernamePassword(String, String),
    LoginWithRefreshToken(String),
    FetchUserById,
    LogOut,
    LoginStatus,
}

impl App {}

impl Component for App {
    type Message = Message;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        log::set_max_level(LevelFilter::Info);

        let auth: Option<(auth::Auth, auth::Token)> = LocalStorage::get(config::AKER_AUTH_KEY)
            .ok()
            .zip(LocalStorage::get(config::AKER_TOKEN_KEY).ok());

        info!("AUTH {:?}", auth);

        let view_login_mut: bool;

        if let Some((auth, _)) = &auth {
            ctx.link().send_message(Message::LoginWithRefreshToken(auth.refresh_token.clone()));
            view_login_mut = false;
        } else {
            ctx.link().send_message(Message::LogOut);
            view_login_mut = true;
        }

        App {
            school_id: None,
            login_status: 0,
            task_school: None,
            task_token: None,
            auth: None,
            graphql_task: None,
            user_profile_sub: None,
            user_profile: None,
            school_profile: None,
            view_login: view_login_mut,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        info!("{:?}", msg);
        let mut should_render = true;
        match msg {
            Message::AuthUser(response) => {

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
                // info!("DATAROUTE {:?}", response.clone());

                if response.clone().is_some() {
                    LocalStorage::set("USER-PROFILE", self.user_profile.clone()).ok();
                }

            }
            Message::AuthSchool(response) => {
                if response.clone().and_then(|data| data.school_by_pk).is_some() {
                    self.school_id = Some(SchoolId(response.clone().and_then(|data| data.school_by_pk).and_then(|item| item.school_profile).unwrap().school_id));
                }

                self.school_profile = response.clone()
                    .and_then(|data| data.school_by_pk.clone())
                    .and_then(|item| {
                        Some(DataSchoolProfile {
                            name: item.school_profile.clone().and_then(|d|Some(d.name)).unwrap_or("".to_string()),
                            logo: item.school_profile.clone().and_then(|d|d.logo).unwrap_or("https://files.roboxmaker.com/uploads/school.png".to_string()),
                            school_id: SchoolId(item.school_profile.clone().and_then(|d|Some(d.school_id)).unwrap_or(Uuid::default())),
                            group_member_id: GroupId(item.school_profile.clone().and_then(|d| d.inventory_group).and_then(|data| Some(data.group_id)).unwrap_or(Uuid::default())),
                        })
                    });
                if response.is_some() {
                    LocalStorage::set("SCHOOL-PROFILE", self.school_profile.clone()).ok();
                }
            }
            Message::FetchUserById => {

                if let Some((_auth, token)) = &self.auth {
                    if let Some(graphql_task) = self.graphql_task.as_mut() {
                        let vars = user_model::user_by_id::Variables {
                            user_id: token.claims.user_id,
                            // user_id: user_id,
                        };

                        let task = user_model::UserById::subscribe(
                            graphql_task,
                            &ctx, 
                            vars, 
                            |response| {
                                Message::AuthUser(response)
                            },
                        );

                        self.user_profile_sub = Some(task);
                    }
                }
            }
            Message::AuthToken(auth_token, should_notify) => {
                self.auth = auth_token;

                if let Some((auth, token)) = &self.auth {
                    LocalStorage::set(config::AKER_AUTH_KEY, auth).ok();
                    LocalStorage::set(config::AKER_TOKEN_KEY, token).ok();
                    roboxmaker_utils::functions::menu_state();
                    self.login_status = 0;
                    ctx.link().send_message(Message::FetchUserById);
                    self.graphql_task = Some(GraphQLService::connect(&code_location!()));

                    if let Some(graphql_task) = self.graphql_task.as_mut() {
                        let vars = school_model::school_by_id::Variables {
                            school_id: token.claims.school_id,
                        };

                        let task = school_model::SchoolById::request(
                            graphql_task,
                            &ctx, 
                            vars, 
                            |response| {
                                Message::AuthSchool(response)
                            },
                        );
                        self.task_school = Some(task);
                    }

                    if self.user_profile.is_some() && self.school_profile.is_some() {

                        // let user_id = self.user_profile.clone().unwrap().user_id;
                        
                        // let navigator = ctx.link().navigator().unwrap();

                        
                        ctx.link().navigator().unwrap().push(&AppRoute::Home);
                        // let _ = Callback::from(move |_: UserId| navigator.push(&AppRoute::Home{user_id}));

                    }
                    // if self.route == AppRoute::Login {
                        // ctx.link().send_message(Message::ChangeRoute(AppRoute::Home));
                    // }

                    // let refresh_token = auth.refresh_token.clone();
                    // let auth_timeout = TimeoutService::spawn(
                    //     // std::time::Duration::from_secs(600),
                    //     std::time::Duration::from_secs(60),
                    //     ctx.link().callback(move |_| {
                    //         Message::LoginWithRefreshToken(refresh_token.clone())
                    //     }),
                    // );
                    // self.auth_timeout = Some(auth_timeout);
                } else {
                    if should_notify {
                        self.login_status = 204;
                    }
                    ctx.link().send_message(Message::LogOut);

                    // ctx.link().navigator().unwrap().push(&AppRoute::Home);
                }
            }
            Message::LoginStatus => {
                info!("{:?}", "ERROR LOGIN".to_string());
            }
            Message::LoginWithUsernamePassword(username, password) => {

                let token_req = TokenRequest::AccessToken(AccessTokenRequest {
                    grant_type: "password".to_string(),
                    client_id: "app-aker".to_string(),
                    username: username,
                    password: password,
                    scope: "openid".to_string(),
                });

                let link = ctx.link().clone();
                spawn_local(async move {

                    let response = auth::token_req(token_req).await;

                    match response {
                        Ok(data) => {
                            link.send_message(Message::AuthToken(data, true))
                        },
                        Err(e) => {
                            info!("ErrorToken: {:?}", e)
                        },
                    }
                });
            }
            Message::LoginWithRefreshToken(refresh_token) => {
                should_render = false;
                let token_req = TokenRequest::RefreshToken(RefreshTokenRequest {
                    grant_type: "refresh_token".to_string(),
                    client_id: "app-aker".to_string(),
                    refresh_token: refresh_token.clone(),
                    scope: "openid".to_string(),
                });


                let link = ctx.link().clone();
                spawn_local(async move {
                    let response = auth::token_req(token_req).await;
                    match response {
                        Ok(data) => link.send_message(Message::AuthToken(data, false)),
                        Err(_e) => {
                            link.send_message(Message::LoginStatus)
                        },
                    }
                });
            }
            Message::LogOut => {

                LocalStorage::delete(config::AKER_AUTH_KEY);
                LocalStorage::delete(config::AKER_TOKEN_KEY);
                LocalStorage::delete("USER-PROFILE");
                LocalStorage::delete("SCHOOL-PROFILE");
                LocalStorage::delete("home-state");
                LocalStorage::delete("school-state");
                LocalStorage::delete("myspace-state");
                LocalStorage::delete("meets-state");
                
                self.user_profile = None;
                self.school_profile = None;
                self.school_id = None;
                self.graphql_task = None;
                self.user_profile_sub = None;
                self.task_school = None;
                self.task_token = None;
                self.auth = None;
                self.view_login = true;
            }
        }
        should_render
    }

    fn changed(&mut self, ctx: &Context<Self>, old_props: &Self::Properties) -> bool {
        info!("{:?} => {:?}", ctx.props(), old_props);

        // if self.user_profile != self.user_profile {
        //     // ctx.link().send_message(Message::FetchUserById)
        //     LocalStorage::set("USER-PROFILE", self.user_profile.clone()).ok();
        // }

        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {

        let on_logout = ctx.link().callback(move |_| Message::LogOut);

        let on_app_login = ctx.link().callback(|(username, password)| Message::LoginWithUsernamePassword(username, password));

        // info!("USERPROFILE {:?}", roboxmaker_utils::functions::user_profile(Option<user_by_id::ResponseData>));
        // info!("USERPROFILEFN {:?}", roboxmaker_utils::functions::return_user_profile());
        // info!("USERPROFILEFN {:?}", return_user_profile());
        let maybe_menu = html! {
            <Menu user_profile={self.user_profile.clone()}
                school_profile={self.school_profile.clone()}
                on_logout={&on_logout} />
        };

        let maybe_view_page = if self.user_profile.is_some() && self.school_profile.is_some() {
            html! {
                <Switch<AppRoute> render={ switch } />
            }
        } else {
            if self.view_login {
                html! {
                    <LoginPage on_app_login={on_app_login}
                        login_status={self.login_status} />
                }
            } else {
                html! {
                    <FullScreenLoader />
                }
            }
        };

        html! {
            <BrowserRouter>
                <div class="d-flex flex-column flex-sm-column flex-md-column flex-lg-row vh-100">
                    {maybe_menu}
                    {maybe_view_page}
                </div>
            </BrowserRouter>
        }
    }
}

fn switch(route: AppRoute) -> Html {
    match route {
        AppRoute::SchoolGroupSection { school_id, group_id, category } => {
            html! {
                <DegreeContent
                    group_id={group_id}
                    school_id={school_id}
                    category={category} />
            }
        },
        AppRoute::GroupSectionStudent { school_id, user_id, category } => {
            html! {
                <DegreeContentStudent 
                    school_id={school_id}
                    user_id={user_id}
                    category={category} />
            }
        },
        AppRoute::MySpace { user_id } => {
            let user_profile = user_profile_data();
            html! {
                <MySpaceView user_id={user_id} user_profile={user_profile} />
            }
        },
        AppRoute::Robot { robot_id, group_id, user_id } => {
            let user_profile = user_profile_data();
            html! {
                <RobotPage user_profile={user_profile}
                    robot_id={robot_id}
                    group_id={group_id} 
                    user_id={user_id} />
            }
        },
        AppRoute::Lesson { school_id, group_id, lesson_id } => {
            let user_profile = user_profile_data();
            html! {
                <LessonPage user_profile={user_profile}
                    school_id={school_id}
                    group_id={group_id}
                    lesson_id={lesson_id} />
            }
        },
        AppRoute::LessonView { school_id, group_id, lesson_id } => {
            let user_profile = user_profile_data();
            html! {
                <LessonPageView user_profile={user_profile}
                    school_id={school_id}
                    group_id={group_id}
                    lesson_id={lesson_id} />
            }
        },
        AppRoute::Post { school_id, group_id, post_id } => {
            let user_profile = user_profile_data();
            html! {
                <PostPage user_profile={user_profile}
                    school_id={school_id}
                    group_id={group_id}
                    post_id={post_id} />
            }
        },
        AppRoute::PostView { school_id, group_id, post_id } => {
            let user_profile = user_profile_data();
            html! {
                <PostPageView user_profile={user_profile}
                    school_id={school_id}
                    group_id={group_id}
                    post_id={post_id} />
            }
        },
        AppRoute::Classes { school_id, group_id, classes_id } => {
            let user_profile = user_profile_data();
            html! {
                <ClassesPage user_profile={user_profile}
                    school_id={school_id}
                    group_id={group_id}
                    classes_id={classes_id} />
            }
        },
        AppRoute::Meet { group_id, meetings_id } => {
            let user_profile = user_profile_data();
            html! {
                <MeetPage user_profile={user_profile}
                    group_id={group_id} 
                    meetings_id={meetings_id} />
            }
        },
        AppRoute::MeetDirect { group_id } => {
            let user_profile = user_profile_data();
            html! {
                <DirectMeetingRoom user_profile={user_profile}
                    group_id={group_id}/>
            }
        },
        AppRoute::Whiteboard { whiteboard_id } => {
            let user_profile = user_profile_data();
            html! {
                <WhiteboardPage user_profile={user_profile}
                    whiteboard_id={whiteboard_id}/>
            }
        },
        AppRoute::Meetings => {
            let user_profile = user_profile_data();
            html! {
                <MeetingsView user_profile={user_profile}/>
            }
        },
        AppRoute::Schools => {
            let user_profile = user_profile_data();
            html! {
                <ListOfSchoolsView user_profile={user_profile}/>
            }
        },
        AppRoute::GradesByUserId { school_id } => {
            let user_profile = user_profile_data();
            let user_id = user_profile_data().clone().and_then(|d|Some(d.user_id)).unwrap_or(UserId(Uuid::default()));
            html! {
                <DegreeList user_profile={user_profile}
                    school_id={school_id}
                    filter={ListOfGradesFilter::UserGroups(user_id)} />
            }
        },
        AppRoute::GradesBySchoolId {school_id } => {
            let user_profile = user_profile_data();
            html! {
                <DegreeList user_profile={user_profile}
                    school_id={school_id}
                    filter={ListOfGradesFilter::SchoolGroups}/>
            }
        },
        AppRoute::PanelAddUsers => {
            html! {
            }
        },
        AppRoute::Home => {
            let user_profile = user_profile_data();
            let school_profile = school_profile_data();

            let school_id = school_profile.clone().and_then(|school| Some(school.school_id)).unwrap_or(roboxmaker_types::types::SchoolId(Uuid::default()));

            let maybe_option_view_home = user_profile.as_ref().and_then(|user| {
                if user.user_staff.is_some() {
                    Some(html! {
                        <HomeStaff />
                    })
                } else {
                    Some(html! {
                        <Home user_id={user.user_id}
                            school_id={school_id} />
                    })
                }

            }).unwrap_or(html! {});
            html! {
                maybe_option_view_home
            }
        },
        AppRoute::NotFound => {
            html! {
            }
        },
    }
}


fn main() {
    wasm_logger::init(wasm_logger::Config::new(log::Level::Info));
    // yew::start_app::<App>();
    yew::Renderer::<App>::new().render();
}