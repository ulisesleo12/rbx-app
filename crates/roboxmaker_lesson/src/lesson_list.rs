use log::*;
use uuid::Uuid;
use yew::prelude::*;
use serde::Serialize;
use serde::Deserialize;
use code_location::code_location;
use crate::lesson_card::LessonCard;
use crate::lesson_select::LessonSelect;
use crate::lesson_select::LessonSelectOption;
use yew::{html, Component, ComponentLink, Html, ShouldRender};

use roboxmaker_main::lang;
use roboxmaker_utils::funtions::get_creation_date;
use roboxmaker_models::{school_model, lesson_model};
use roboxmaker_models::lesson_model::lesson_group_create;
use roboxmaker_models::lesson_model::lessons_list_by_group;
use roboxmaker_types::types::{GroupId, LessonId, AppRoute, SchoolId, MyUserProfile, UserId};
use roboxmaker_graphql::{GraphQLService, GraphQLTask, Subscribe, SubscriptionTask, Request, RequestTask};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TypeLesson {
    TeachingCards,
    ElectronicsLessons,
    Extra,
    None,
}


#[derive(Debug, Clone, PartialEq)]
pub struct LessonProfile {
    pub title: String,
    pub timestamp: String,
    pub lesson_id: LessonId,
    pub author_full_name: String,
    pub author_pic_path: String,
    pub author_id: bool,
    pub archived: bool,
    pub send_to_degree: bool,
    pub school_name: String,
    pub school_logo: String,
    pub content: String,
    pub on_dropdown_menu: bool,
    pub lesson_type: lessons_list_by_group::RoboxLessonTypeEnum,
    pub order: Option<String>,
    // pub lesson_type: String,
}

pub struct LessonList {
    link: ComponentLink<Self>,
    props: LessonListProperties,
    graphql_task: Option<GraphQLTask>,
    lesson_sub: Option<SubscriptionTask>,
    lesson_delete_task: Option<RequestTask>,
    lesson_add_task: Option<RequestTask>,
    show_dropdown_filter: bool,
    filter: LessonFilter,
    lesson_list: Vec<LessonProfile>,
    more_lessons: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub enum LessonFilter {
    Alls,
    Published,
    Unpublished,
    // Archived,
}

#[derive(Debug, Properties, Clone, PartialEq)]
pub struct LessonListProperties {
    pub group_id: GroupId,
    pub school_id: SchoolId,
    pub user_profile: Option<MyUserProfile>,
    pub auth_school: Option<school_model::school_by_id::SchoolByIdSchoolByPk>,
    pub on_app_route: Callback<AppRoute>,
    pub on_list_change: Option<Callback<()>>,
    pub inventory_group: Option<Uuid>,
    pub class_name: String,
}

#[derive(Debug)]
pub enum LessonListMessage {
    AppRoute(AppRoute),
    FetchLessonsByGroupId,
    Lessons(Option<lesson_model::lessons_list_by_group::ResponseData>),
    AddLesson(LessonId),
    RemoveLesson(LessonId),
    RemoveLessonEntirely(LessonId),
    CreateLesson,
    LessonAdded(Option<LessonId>),
    LessonRemoved(Option<LessonId>),
    ShowDropdown,
    ChangeFilter(LessonFilter),
    ShowMoreLessons,
    UpdateLessonList(LessonId, bool, bool),
}

impl Component for LessonList {
    type Message = LessonListMessage;
    type Properties = LessonListProperties;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        link.send_message(LessonListMessage::FetchLessonsByGroupId);
        LessonList {
            link,
            props,
            graphql_task: Some(GraphQLService::connect(&code_location!())),
            lesson_sub: None,
            lesson_delete_task: None,
            lesson_add_task: None,
            lesson_list: vec![],
            show_dropdown_filter: false,
            filter: LessonFilter::Alls,
            more_lessons: false,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        info!("{:?}", msg);
        let mut should_update = true;
        match msg {
            LessonListMessage::AppRoute(route) => {
                self.props.on_app_route.emit(route)
            }
            LessonListMessage::FetchLessonsByGroupId => {
                if let Some(graphql_task) = self.graphql_task.as_mut() {

                    let vars = lesson_model::lessons_list_by_group::Variables {
                        group_id: self.props.group_id.0,
                    };

                    let task = lesson_model::LessonsListByGroup::subscribe(
                            graphql_task,
                            &self.link,
                            vars,
                            |response| {
                                LessonListMessage::Lessons(response)
                            },
                    );
                    self.lesson_sub = Some(task);
                }
            }
            LessonListMessage::Lessons(response) => { 
                info!("Lessons response: {:?}", response);
                self.lesson_list = response
                    .clone()
                    .and_then(|data| Some(data.lesson_group))
                    .unwrap_or(vec![])
                    .iter()
                    .filter(|data| {
                        
                        self.filter == LessonFilter::Alls && {data.archived == true || data.archived == false || data.send_to_grade == true || data.send_to_grade == false} ||
    
                        self.filter == LessonFilter::Published && data.send_to_grade == true && data.archived == false ||
            
                        self.filter == LessonFilter::Unpublished && data.archived == false && data.send_to_grade == false

                    })
                    .map(|item| {
                        let naive = chrono::NaiveDate::from_ymd_opt(2023, 01, 01).unwrap().and_hms_opt(23, 59, 59).unwrap();

                        let timestamp = item.lesson_profile.clone().and_then(|data| Some(data.timestamp)).unwrap_or(naive);
                        
                        let time_fn = get_creation_date(timestamp);

                        let my_id = self.props.user_profile.clone().and_then(|user_by_pk| Some(user_by_pk.user_id)).unwrap_or(UserId(Uuid::default()));
                        let author_id = if item.lesson_profile.clone().and_then(|data| Some(data.author_id)).unwrap_or(Uuid::default()) == my_id.0 {
                            true
                        } else {
                            false
                        };

                        let lesson_type = item.lesson_profile.clone().and_then(|data| data.lesson_type).unwrap_or(lessons_list_by_group::RoboxLessonTypeEnum::Extra);

                        LessonProfile { 
                            title: item.lesson_profile.clone().and_then(|data| Some(data.title)).unwrap_or("".to_string()), 
                            timestamp: time_fn, 
                            lesson_id: LessonId(item.lesson_id), 
                            author_full_name: item.lesson_profile.clone().and_then(|data| Some(data.author)).clone().and_then(|author| author.user_profile).clone().and_then(|user_profile| Some(user_profile.full_name)).unwrap_or("".to_string()), 
                            author_pic_path: item.lesson_profile.clone().and_then(|data| Some(data.author)).clone().and_then(|author| author.user_profile).clone().and_then(|user_profile| user_profile.pic_path).unwrap_or("".to_string()), 
                            author_id,
                            archived: item.archived, 
                            send_to_degree: item.send_to_grade, 
                            school_name: item.school_group.clone().and_then(|data| Some(data.school)).clone().and_then(|school| school.school_profile).clone().and_then(|school_profile| Some(school_profile.name)).unwrap_or("".to_string()), 
                            school_logo: item.school_group.clone().and_then(|data| Some(data.school)).clone().and_then(|school| school.school_profile).clone().and_then(|school_profile| school_profile.logo).unwrap_or("".to_string()), 
                            content: item.lesson_content.clone().and_then(|data| Some(data.content)).unwrap_or("".to_string()),
                            on_dropdown_menu: false,
                            lesson_type,
                            order: item.lesson_profile.clone().and_then(|data| data.order),
                        }
                    }).collect();

            }
            LessonListMessage::AddLesson(lesson_id) => {
                if let Some(graphql_task) = self.graphql_task.as_mut() {
                        
                    let vars = lesson_model::lesson_group_add::Variables { 
                        group_id: self.props.group_id.0,
                        lesson_id: lesson_id.0,
                    };

                    let task = lesson_model::LessonGroupAdd::request(
                        graphql_task,
                        &self.link,
                        vars,
                        |response| {
                            let lesson_id = if let Some(lesson) = response {
                                lesson.insert_lesson_group_one.and_then(|data| Some(LessonId(data.lesson_id)))
                            } else {
                                None
                            };
                            LessonListMessage::LessonAdded(lesson_id)
                        },
                    );
                    self.lesson_add_task = Some(task);
                }
            }
            LessonListMessage::RemoveLesson(lesson_id) => {
                if let Some(graphql_task) = self.graphql_task.as_mut() {

                    let vars = lesson_model::lesson_group_delete::Variables { 
                        group_id: self.props.group_id.0,
                        lesson_id: lesson_id.0,
                    };

                    let task = lesson_model::LessonGroupDelete::request(
                        graphql_task,
                        &self.link,
                        vars,
                        |response| {
                            let lesson_id = if let Some(response) = response {
                                if response.delete_lesson_group.clone().and_then(|data| Some(data.returning)).unwrap_or(vec![]).len() > 0 {
                                    Some(LessonId(response.delete_lesson_group.clone().and_then(|data| Some(data.returning)).unwrap_or(vec![])[0].lesson_id))
                                } else {
                                    None
                                }
                            } else {
                                None
                            };
                            LessonListMessage::LessonRemoved(lesson_id)
                        },
                    );
                    self.lesson_delete_task = Some(task);
                }
            }
            LessonListMessage::RemoveLessonEntirely(lesson_id) => {
                if let Some(graphql_task) = self.graphql_task.as_mut() {

                    let vars = lesson_model::delete_lesson_by_id::Variables { 
                        lesson_id: lesson_id.0,
                    };

                    let task = lesson_model::DeleteLessonById::request(
                        graphql_task,
                        &self.link,
                        vars,
                        |response| {
                            let lesson_id = if let Some(response) = response {
                                if response.delete_lesson_by_pk.clone().and_then(|data| Some(data.id)).is_some() {
                                    let id = response.delete_lesson_by_pk.clone().and_then(|data| Some(data.id)).unwrap();
                                    Some(LessonId(id))
                                } else {
                                    None
                                }
                            } else {
                                None
                            };
                            LessonListMessage::LessonRemoved(lesson_id)
                        },
                    );
                    self.lesson_delete_task = Some(task);
                }
            }
            LessonListMessage::CreateLesson => {
                if let Some(graphql_task) = self.graphql_task.as_mut() {
                    let local = chrono::Local::now().naive_local();

                    let type_lesson = lesson_group_create::RoboxLessonTypeEnum::Extra;

                    if let Some(inventory_group_id) = self.props.inventory_group {
                        let vars = lesson_model::lesson_group_create::Variables { 
                            title: String::from(lang::dict("~ New Lesson ~")),
                            summary: String::from(""),
                            content: String::from(""),
                            group_id: self.props.group_id.0,
                            inventory_group_id,
                            lesson_id: Uuid::new_v4(),
                            timestamp: local,
                            lesson_type: type_lesson,
                        };
    
                        let task = lesson_model::LessonGroupCreate::request(
                            graphql_task,
                            &self.link,
                            vars,
                            |response| {
                                let lesson_id = if let Some(lesson) = response {
                                    lesson.insert_lesson_group_one.and_then(|data| Some(LessonId(data.lesson_id)))
                                } else {
                                    None
                                };
                                LessonListMessage::LessonAdded(lesson_id)
                            },
                        );
                        self.lesson_add_task = Some(task);
                        self.link.send_message(LessonListMessage::FetchLessonsByGroupId);
                    }
                }
            }
            LessonListMessage::LessonAdded(lesson_id) => {
                let group_id = self.props.group_id;
                let school_id = self.props.school_id;
                let lesson_type = lessons_list_by_group::RoboxLessonTypeEnum::Extra;


                if let Some(lesson_id) = lesson_id {
                    self.lesson_list.push(LessonProfile { 
                        title: String::from(""), timestamp: String::from(""), 
                        lesson_id, 
                        author_full_name: String::from(""), 
                        author_pic_path: String::from(""), 
                        author_id: false, 
                        archived: false, send_to_degree: false, 
                        school_name: String::from(""), school_logo: String::from(""), 
                        content: String::from(""),
                        on_dropdown_menu: false, 
                        lesson_type: lesson_type,
                        order: None,
                    });
                    self.link.send_message(LessonListMessage::AppRoute(AppRoute::Lesson(school_id, group_id, lesson_id)));
                } else {
                    should_update = true;
                }
            }
            LessonListMessage::LessonRemoved(lesson_id) => {
                if let Some(lesson_id) = lesson_id {
                    self.lesson_list.retain(|u| u.lesson_id != lesson_id);
                } else {
                    should_update = true;
                }
            }
            LessonListMessage::ShowDropdown => {
                self.show_dropdown_filter = !self.show_dropdown_filter;
            }
            LessonListMessage::ChangeFilter(filter) => {
                self.filter = filter;
                self.show_dropdown_filter = false;
                self.link.send_message(LessonListMessage::FetchLessonsByGroupId);
            }
            LessonListMessage::ShowMoreLessons => {
                self.more_lessons = !self.more_lessons;
            }
            LessonListMessage::UpdateLessonList(lesson_id, send_to_grade , archived) => {
                for lesson in self.lesson_list.iter_mut() {
                    if lesson.lesson_id == lesson_id {
                        lesson.send_to_degree = send_to_grade;
                        lesson.archived = archived;
                    }
                }
            }
        }
        should_update
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        info!("{:?} => {:?}", self.props, props);
        let mut should_render = false;
 
        if self.props != props {
            self.props = props;
            should_render = true;
        } 

        should_render
    }

    fn view(&self) -> Html {
        let group_id = self.props.group_id;
        let on_alls = self.link.callback(|_| LessonListMessage::ChangeFilter(LessonFilter::Alls));
        let on_published = self.link.callback(|_| LessonListMessage::ChangeFilter(LessonFilter::Published));
        let on_unpublished = self.link.callback(|_| LessonListMessage::ChangeFilter(LessonFilter::Unpublished));
        // let on_archived = self.link.callback(|_| LessonListMessage::ChangeFilter(LessonFilter::Archived));
        let on_dropdown = self.link.callback(|_| LessonListMessage::ShowDropdown);
        let on_more_lessons = self.link.callback(|_| LessonListMessage::ShowMoreLessons);
        let on_change_list = self.link.callback(|(lesson_id, send_to_grade, archived)| LessonListMessage::UpdateLessonList(lesson_id, send_to_grade, archived));
        let on_lesson_delete = self.link.callback(|lesson_id| LessonListMessage::RemoveLesson(lesson_id));
        let on_del_lesson_entirely = self.link.callback(|lesson_id| LessonListMessage::RemoveLessonEntirely(lesson_id));

        let teaching_cards_count = self.lesson_list.iter().filter(|lesson| lesson.lesson_type == lessons_list_by_group::RoboxLessonTypeEnum::TeachingCards).count();
        let lesson_electronics_count = self.lesson_list.iter().filter(|lesson| lesson.lesson_type == lessons_list_by_group::RoboxLessonTypeEnum::ElectronicsLessons).count();
        let others_lessons = self.lesson_list.iter().filter(|item| item.lesson_type == lessons_list_by_group::RoboxLessonTypeEnum:: Extra).count();
        let others_lessons_student = self.lesson_list.iter().filter(|item| item.lesson_type == lessons_list_by_group::RoboxLessonTypeEnum:: Extra).count();

        let maybe_option_seleted = match self.filter {
            LessonFilter::Alls => "Everyone",
            LessonFilter::Published => "Released",
            LessonFilter::Unpublished => "Unpublished",
            // LessonFilter::Archived => "Archived",
        };
        let class_dropdown = if self.show_dropdown_filter {
            "btn btn-secondary btn-see-degree dropdown-toggle show d-flex align-items-center justify-content-between"
        } else {
            "btn btn-secondary btn-see-degree dropdown-toggle d-flex align-items-center justify-content-between"
        };
        let class_dropdown_list = if self.show_dropdown_filter {
            "dropdown-menu dropdown-menu-degree show"
        } else {
            "dropdown-menu dropdown-menu-degree"
        };
        let maybe_dropdown_by_user = self
            .props
            .user_profile
            .as_ref()
            .and_then(|user|{
                if user.user_staff.is_some() || user.user_teacher.is_some() {
                    Some(html! {
                        <div class="dropdown me-5">
                            <button class=class_dropdown type="button" id="dropdownMenuButton2" data-bs-toggle="dropdown" aria-expanded="false" onclick=on_dropdown>
                                <img src="/icons/filter.svg" style="height: 22px;" />
                                <span class="universal-select-option text-secondary-purple noir-regular is-size-18 lh-22">{lang::dict(maybe_option_seleted)}</span>
                            </button>
                            <ul class=class_dropdown_list aria-labelledby="dropdownMenuButton2">
                                <li>
                                    <a class="dropdown-item d-flex flex-wrap align-items-center mt-1 pe-0" onclick=on_alls>
                                        <input class="bg-checkbox" type="checkbox" checked={if self.filter == LessonFilter::Alls {true} else {false}} />
                                        <span class={if self.filter == LessonFilter::Alls {"text-blue-purple noir-regular is-size-18 lh-22 ps-2"} else {"text-gray-purple noir-regular is-size-18 lh-22 ps-2"}}>{lang::dict("Everyone")}</span>
                                    </a>
                                </li>
                                <li>
                                    <a class="dropdown-item d-flex flex-wrap align-items-center pe-0" onclick=on_published>
                                        <input class="bg-checkbox" type="checkbox" checked={if self.filter == LessonFilter::Published {true} else {false}} />
                                        <span class={if self.filter == LessonFilter::Published {"text-blue-purple noir-regular is-size-18 lh-22 ps-2"} else {"text-gray-purple noir-regular is-size-18 lh-22 ps-2"}}>{lang::dict("Released")}</span>
                                    </a>
                                </li>
                                <li>
                                    <a class="dropdown-item d-flex flex-wrap align-items-center pe-0" onclick=on_unpublished>
                                        <input class="bg-checkbox" type="checkbox" checked={if self.filter == LessonFilter::Unpublished {true} else {false}} />
                                        <span class={if self.filter == LessonFilter::Unpublished {"text-blue-purple noir-regular is-size-18 lh-22 ps-2"} else {"text-gray-purple noir-regular is-size-18 lh-22 ps-2"}}>{lang::dict("Unpublished")}</span>
                                    </a>
                                </li>
                                // <li>
                                //     <a class="dropdown-item d-flex flex-wrap align-items-center pe-0" onclick=on_archived>
                                //         <input class="bg-checkbox" type="checkbox" checked={if self.filter == LessonFilter::Archived {true} else {false}} />
                                //         <span class={if self.filter == LessonFilter::Archived {"text-blue-purple noir-regular is-size-18 lh-22 ps-2"} else {"text-gray-purple noir-regular is-size-18 lh-22 ps-2"}}>{lang::dict("Archived")}</span>
                                //     </a>
                                // </li>
                            </ul>
                        </div>
                    })
                } else {
                    Some(html! {})
                }
            })
            .unwrap_or(html! {});

        let mut maybe_lesson_staff_vec: Vec<_> = self.lesson_list.iter()
            .filter(| lessons | {

            self.filter == LessonFilter::Alls && lessons.lesson_type == lessons_list_by_group::RoboxLessonTypeEnum::TeachingCards && {lessons.archived == true || lessons.archived == false || lessons.send_to_degree == true || lessons.send_to_degree == false} ||

            self.filter == LessonFilter::Published && lessons.lesson_type == lessons_list_by_group::RoboxLessonTypeEnum::TeachingCards && lessons.send_to_degree == true && lessons.archived == false ||

            self.filter == LessonFilter::Unpublished && lessons.lesson_type == lessons_list_by_group::RoboxLessonTypeEnum::TeachingCards && lessons.archived == false && lessons.send_to_degree == false

            })
            .collect();

        maybe_lesson_staff_vec.sort_by(|a, b| {
            let extract_number = |order: &Option<String>| -> i32 {
                order.as_ref()
                    .and_then(|s| {
                        // Extraer el número después de 'L' (ejemplo: "P1L10" -> 10)
                        s.split('L').nth(1)
                            .and_then(|num_str| num_str.parse::<i32>().ok())
                    })
                    .unwrap_or(i32::MAX)
            };

            let order_a = extract_number(&a.order);
            let order_b = extract_number(&b.order);
            order_a.cmp(&order_b)
        });

        let maybe_lesson_staff = maybe_lesson_staff_vec.iter()
            .map(|item| {
                let lesson_profile = item.clone();
                html! {
                <>
                    <LessonCard lesson_id=item.lesson_id.clone()
                        user_profile=self.props.user_profile.clone()
                        group_id=self.props.group_id
                        on_app_route=self.props.on_app_route.clone()
                        on_lesson_delete=on_lesson_delete.clone()
                        on_del_lesson_entirely={&on_del_lesson_entirely}
                        auth_school=self.props.auth_school.clone()
                        on_change_list=on_change_list.clone()
                        lesson_profile={lesson_profile.clone()}
                        archived=item.archived
                        send_to_grade=item.send_to_degree
                        school_id=self.props.school_id />
                </>
            }
        }).collect::<Html>();

        let maybe_lesson_staff_electronic = self.lesson_list.iter()
            .filter(| lessons | {

            self.filter == LessonFilter::Alls && lessons.lesson_type == lessons_list_by_group::RoboxLessonTypeEnum::ElectronicsLessons && {lessons.archived == true || lessons.archived == false || lessons.send_to_degree == true || lessons.send_to_degree == false} ||

            self.filter == LessonFilter::Published && lessons.lesson_type == lessons_list_by_group::RoboxLessonTypeEnum::ElectronicsLessons && lessons.send_to_degree == true && lessons.archived == false ||

            self.filter == LessonFilter::Unpublished && lessons.lesson_type == lessons_list_by_group::RoboxLessonTypeEnum::ElectronicsLessons && lessons.archived == false && lessons.send_to_degree == false

            })
            .map(|item| {
                let lesson_profile = item.clone();

                html! {
                <>
                    <LessonCard lesson_id=item.lesson_id.clone()
                        user_profile=self.props.user_profile.clone()
                        group_id=self.props.group_id
                        on_app_route=self.props.on_app_route.clone()
                        on_lesson_delete=on_lesson_delete.clone()
                        on_del_lesson_entirely={&on_del_lesson_entirely}
                        auth_school=self.props.auth_school.clone()
                        on_change_list=on_change_list.clone()
                        lesson_profile={lesson_profile}
                        archived=item.archived
                        send_to_grade=item.send_to_degree
                        school_id=self.props.school_id />
                </>
            }
        }).collect::<Html>();


        let maybe_your_lessons = self
            .lesson_list
            .iter()
            .filter(| lessons | {

                self.filter == LessonFilter::Alls && lessons.lesson_type == lessons_list_by_group::RoboxLessonTypeEnum::Extra && {lessons.archived == true || lessons.archived == false || lessons.send_to_degree == true || lessons.send_to_degree == false} ||

                self.filter == LessonFilter::Published && lessons.lesson_type == lessons_list_by_group::RoboxLessonTypeEnum::Extra && lessons.send_to_degree == true && lessons.archived == false && {lessons.author_id == true} ||

                self.filter == LessonFilter::Unpublished && lessons.lesson_type == lessons_list_by_group::RoboxLessonTypeEnum::Extra && lessons.archived == false && lessons.send_to_degree == false && {lessons.author_id == true}

            })
            .map(|item| {
            let lesson_profile = item.clone();

            html! {
                <LessonCard lesson_id=item.lesson_id.clone()
                    user_profile=self.props.user_profile.clone() 
                    group_id={group_id}
                    on_app_route=self.props.on_app_route.clone()
                    on_lesson_delete=on_lesson_delete.clone()
                    on_del_lesson_entirely={&on_del_lesson_entirely}
                    auth_school=self.props.auth_school.clone()
                    on_change_list=on_change_list.clone()
                    lesson_profile={lesson_profile}
                    archived=item.archived
                    send_to_grade=item.send_to_degree
                    school_id=self.props.school_id />
            }
        }).collect::<Html>();

        let maybe_electronic_by_student = self.lesson_list.iter()
            .filter(| lessons | {

            self.filter == LessonFilter::Alls && lessons.lesson_type == lessons_list_by_group::RoboxLessonTypeEnum::ElectronicsLessons && lessons.send_to_degree == true && lessons.archived == false

            })
            .map(|item| {
                let lesson_profile = item.clone();

                html! {
                <>
                    <LessonCard lesson_id=item.lesson_id.clone()
                        user_profile=self.props.user_profile.clone()
                        group_id=self.props.group_id
                        on_app_route=self.props.on_app_route.clone()
                        on_lesson_delete=on_lesson_delete.clone()
                        on_del_lesson_entirely={&on_del_lesson_entirely}
                        auth_school=self.props.auth_school.clone()
                        on_change_list=on_change_list.clone()
                        lesson_profile={lesson_profile}
                        archived=item.archived
                        send_to_grade=item.send_to_degree
                        school_id=self.props.school_id />
                </>
            }
        }).collect::<Html>();

        let maybe_other_lessons_by_student = self.lesson_list.iter()
            .filter(| lessons | {

            self.filter == LessonFilter::Alls && lessons.lesson_type == lessons_list_by_group::RoboxLessonTypeEnum::Extra && lessons.send_to_degree == true && lessons.archived == false

            })
            .map(|item| {
                let lesson_profile = item.clone();

                html! {
                <>
                    <LessonCard lesson_id=item.lesson_id.clone()
                        user_profile=self.props.user_profile.clone()
                        group_id=self.props.group_id
                        on_app_route=self.props.on_app_route.clone()
                        on_lesson_delete=on_lesson_delete.clone()
                        on_del_lesson_entirely={&on_del_lesson_entirely}
                        auth_school=self.props.auth_school.clone()
                        on_change_list=on_change_list.clone()
                        lesson_profile={lesson_profile}
                        archived=item.archived
                        send_to_grade=item.send_to_degree
                        school_id=self.props.school_id />
                </>
            }
        }).collect::<Html>();

        let maybe_lesson_search = self
            .props
            .user_profile
            .as_ref()
            .and_then(|item| {
                let on_select = self.link.callback(|select_option| match select_option {
                    LessonSelectOption::Lesson(lesson_id) => { LessonListMessage::AddLesson(lesson_id) }
                });
                if item.user_staff.is_some() || item.user_teacher.is_some() {
                    Some(html! {
                        <LessonSelect on_select=on_select 
                            allow_create=true
                            on_app_route=self.props.on_app_route.clone()
                            school_id=self.props.school_id />
                    })
                } else {
                    None
                }
            })
            .unwrap_or(html! {});

        let class_name = self.props.class_name.clone().to_uppercase();

        let arduino_electronic = if class_name.contains("RECURSOS EXTRA - ARDUINO") 
            || class_name.contains("RECURSOS EXTRA - ELECTRÓNICA") {
            true
        } else {
            false
        };

        let lessons_class = if self.more_lessons {
            // "d-flex flex-wrap more-lesson-view"
            "d-flex flex-wrap"
        } else {
            "d-flex flex-wrap more-lesson-hidden"
        };

        let maybe_more_lesson = if arduino_electronic {
            html! {
                <>
                    <div class="d-flex justify-content-center">
                        <a onclick=on_more_lessons>
                            <span class="text-secondary-purple noir-bold is-size-18 lh-22 pt-5">{lang::dict("See All Electronics Lessons")}</span>
                        </a>
                    </div>
                </>
            }
        } else {
            html! {
                <>
                    <div class="d-flex justify-content-center">
                        <a onclick=on_more_lessons>
                            <span class="text-secondary-purple noir-bold is-size-18 lh-22 pt-5">{lang::dict("See All Teacher Resources")}</span>
                        </a>
                    </div>
                </>
            }
        };


        let maybe_user_profile_pic = self
            .props
            .user_profile
            .as_ref()
            .and_then(|user_profile| Some(user_profile.pic_path.clone()))
            .and_then(|pic_path| {
                Some(html! {
                    <img class="img-card-72" src=pic_path.clone() alt="photo of user" />
                })
            })
            .unwrap_or(html! {<img class="img-card-72" src="/static/avatar.png"/>
            });
        let on_direct_meet = self.link.callback(move |_| LessonListMessage::AppRoute(AppRoute::MeetDirect(group_id)));

        let head_section = html! {
            <div class="d-flex flex-wrap align-items-center justify-content-between mb-5">
                <h1 class="text-primary-blue-dark text-uppercase noir-bold is-size-36 lh-43 mb-0">
                    {self.props.class_name.clone()}
                </h1>
                <a class="btn btn-outline-light text-primary-blue-dark noir-regular is-size-18 lh-22" onclick=on_direct_meet>
                    <img class="me-3" src="/icons/video-2.svg" style="height: 30px;" />
                    <span>{lang::dict("Meet up")}</span>
                </a>
                {maybe_lesson_search}
                {maybe_user_profile_pic}
            </div>
        };

        let maybe_new = self
            .props
            .user_profile
            .as_ref()
            .and_then(|item| {
                let on_select = self.link.callback(move |_| LessonListMessage::CreateLesson);
                if item.user_staff.is_some() || item.user_teacher.is_some() {
                    Some(html! {
                        <a class="button btn-create-card bg-primary-blue-dark d-flex align-items-center justify-content-center" onclick=on_select.clone()>
                            <span class="text-white noir-bold is-size-16 lh-20 d-flex align-items-center">
                                <i class="fas fa-plus me-2"></i>
                                <span>{lang::dict("New Lesson")}</span>
                            </span>
                        </a>
                    })
                } else {
                    None
                }
            }).unwrap_or(html! {});


        let title_lessons_option = if arduino_electronic {
            html! {
                <span class="text-primary-blue-dark noir-bold is-size-24 lh-29 mb-3 mb-sm-3 mb-md-3 mb-lg-0">
                    {lang::dict("Electronics Lessons")} <span class="ps-1">{"("}{lesson_electronics_count}{")"}</span>
                </span>
            }
        } else {
            html! {
                <span class="text-primary-blue-dark noir-bold is-size-24 lh-29 mb-3 mb-sm-3 mb-md-3 mb-lg-0">
                    {lang::dict("Teacher Resources")} <span class="ps-1">{"("}{teaching_cards_count}{")"}</span>
                </span>
            }
        };

        let maybe_dropdown = html! {
            <div class="d-flex flex-wrap align-items-center justify-content-between pb-4">
                {title_lessons_option}
                <div class="d-flex flex-wrap">
                    {maybe_dropdown_by_user}
                    {maybe_new}
                </div>
            </div>
        };

        // let maybe_option_staff = if self.lesson_list.iter().filter(|item| item.type_filter == false).count() > 0 {
        let maybe_option_staff = if self.lesson_list.iter().count() > 0 {
            html! {
                <>
                    <div class={lessons_class}>
                        {maybe_lesson_staff}
                    </div>
                </>
            }
        } else {
            html! {
                <div class="text-center">
                    <span class="text-gray-strong is-size-18 lh-20">{lang::dict("No teacher resources.")}</span>
                </div>
            }
        };

        // let maybe_option_staff_electronic = if self.lesson_list.iter().filter(|item| item.type_filter == true).count() > 0 {
        let maybe_option_staff_electronic = if self.lesson_list.iter().count() > 0 {
            let lessons_class = if self.more_lessons {
                "d-flex flex-wrap pt-4"
            } else {
                "d-flex flex-wrap pt-4 more-lesson-hidden"
            };
            if arduino_electronic {
                html! {
                    <div class={lessons_class}>
                        {maybe_lesson_staff_electronic}
                    </div>
                }
            } else {
                html! {
                    <div class="d-flex flex-wrap pt-4">
                        {maybe_lesson_staff_electronic}
                    </div>
                }
            }
        } else {
            if arduino_electronic {
                html! {
                    <div class="text-center">
                        <span class="text-gray-strong is-size-18 lh-20">{lang::dict("No electronics lessons.")}</span>
                    </div>
                }
            } else {
                html! {}
            }
        };

        // let maybe_option_teacher = if self.lesson_list.iter().filter(|item| item.author_id == true && item.lesson_type == lessons_list_by_group::RoboxLessonTypeEnum:: Extra).count() > 0 {
        let maybe_option_teacher = if self.lesson_list.iter().filter(|item| item.lesson_type == lessons_list_by_group::RoboxLessonTypeEnum:: Extra).count() > 0 {
            html! {
                <div class="d-flex flex-wrap mt-5 pb-9 mb-8">
                    {maybe_your_lessons}
                </div>
            }
        } else {
            html! {
                <div class="text-center pb-9 mb-7">
                    <span class="text-gray-strong is-size-18 lh-20">{lang::dict("No lessons here.")}</span>
                </div>
            }
        };

        // let electronics_lessons = if self.lesson_list.iter().filter(|item| item.type_filter == true).count() > 0 {
        // let electronics_lessons = if self.lesson_list.iter().count() > 0 {
        let electronics_lessons = if arduino_electronic {
                html! {
                    {maybe_option_staff_electronic.clone()}
                }
            } else {
                html! {
                    <>
                        {maybe_option_staff_electronic.clone()}
                        <br/>
                    </>
                }
            };
        // } else {
        //     html! {}
        // };

        let no_view_lesson_electronic_title = if class_name.contains("KINDER") 
            || class_name.contains("PREPARATORIA") {
            true
        } else {
            false
        };

        let by_user_profile_view = self
            .props
            .user_profile
            .as_ref()
            .and_then(|item| {
                if item.user_staff.is_some() || item.user_teacher.is_some() {
                    Some(html! {
                        <>
                            {maybe_dropdown}
                            {if arduino_electronic {
                                html! {}
                            } else {
                                html! {maybe_option_staff}
                            }}
                            {if arduino_electronic {
                                html! {}
                            } else {
                                html! {maybe_more_lesson.clone()}
                            }}
                            {if arduino_electronic {
                                html! {electronics_lessons.clone()}
                            } else {
                                html! {
                                    <>
                                        {if no_view_lesson_electronic_title {
                                            html! {}
                                        } else {
                                            html! {
                                                <>
                                                    <br/>
                                                    <br/>
                                                    <span class="text-primary-blue-dark noir-bold is-size-24 lh-29 mb-3 mb-sm-3 mb-md-3 mb-lg-0">
                                                        {lang::dict("Electronics Lessons")} <span class="ps-1">{"("}{lesson_electronics_count}{")"}</span>
                                                    </span>
                                                </>
                                            }
                                        }}
                                        <br/>
                                        {electronics_lessons.clone()}
                                    </>
                                }
                            }}
                            {if arduino_electronic {
                                html! {maybe_more_lesson.clone()}
                            } else {
                                html! {}
                            }}
                            <span class="text-primary-blue-dark noir-bold is-size-24 lh-29 mb-3 mb-sm-3 mb-md-3 mb-lg-0">
                                {lang::dict("Your Lessons")} <span class="ps-1">{"("}{others_lessons}{")"}</span>
                            </span>
                            {maybe_option_teacher}
                        </>
                    })
                } else {
                    Some(html! {
                        <>
                            <span class="text-primary-blue-dark noir-bold is-size-24 lh-29 mb-3 mb-sm-3 mb-md-3 mb-lg-0">
                                // {lang::dict("Other Lessons")} <span class="ps-1">{"("}{lesson_electronics_count}{")"}</span>
                                {lang::dict("Electronics Lessons")} <span class="ps-1">{"("}{lesson_electronics_count}{")"}</span>
                            </span>
                            // {   if self.lesson_list.iter().filter(|item| item.type_filter == true).count() > 0 {
                            {   if lesson_electronics_count > 0 {
                                    html! {
                                        <>
                                            <div class="d-flex flex-wrap pt-4">
                                                {maybe_electronic_by_student}
                                            </div>
                                        </>
                                    }
                                } else {
                                    // if arduino_electronic {
                                        html! {
                                            <div class="text-center">
                                                <span class="text-gray-strong is-size-18 lh-20">{lang::dict("No electronics lessons.")}</span>
                                            </div>
                                        }
                                    // } else {
                                        // html! {}
                                    // }
                                }
                            }
                            <br/>
                            <br/>
                            <span class="text-primary-blue-dark noir-bold is-size-24 lh-29 mb-3 mb-sm-3 mb-md-3 mb-lg-0">
                                {lang::dict("Other Lessons")} <span class="ps-1">{"("}{others_lessons_student}{")"}</span>
                            </span>
                            {
                                if others_lessons_student > 0 {
                                    html! {
                                        <>
                                            <div class="d-flex flex-wrap pt-4 pb-9 mb-8">
                                                {maybe_other_lessons_by_student}
                                            </div>
                                        </>
                                    }
                                } else {
                                    html! {
                                        <div class="text-center pb-9 mb-7">
                                            <span class="text-gray-strong is-size-18 lh-20">{lang::dict("No lessons here.")}</span>
                                        </div>
                                    }
                                }
                            }
                        </>
                    })
                }
            }).unwrap_or(html! {});

        html! { 
            <div class="scroll-y w-100 h-100 p-3 p-md-4 p-lg-7 pb-6">
                {head_section}
                {by_user_profile_view}
            </div>
        }
    }
}