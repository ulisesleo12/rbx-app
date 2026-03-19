use log::*;
use roboxmaker_models::lesson_model::get_lesson_list;
use uuid::Uuid;
use yew::prelude::*;
use serde::Serialize;
use serde::Deserialize;
use code_location::code_location;
use yew::{html, Component, Html};
use crate::lesson_card::LessonCard;
use crate::lesson_select::LessonSelect;
use yew_router::scope_ext::RouterScopeExt;
use crate::lesson_select::LessonSelectOption;

use roboxmaker_main::lang;
use roboxmaker_models::lesson_model;
use roboxmaker_utils::functions::get_creation_date;
use roboxmaker_models::lesson_model::lesson_class_and_group_create;
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
    pub lesson_type: get_lesson_list::RoboxLessonTypeEnum,
    // pub lesson_type: String,
}

pub struct LessonList {
    graphql_task: Option<GraphQLTask>,
    lesson_sub: Option<SubscriptionTask>,
    lesson_delete_task: Option<RequestTask>,
    lesson_add_task: Option<RequestTask>,
    show_dropdown_filter: bool,
    filter: LessonFilter,
    lesson_list: Vec<LessonProfile>,
    lesson_list_view: Vec<LessonProfile>,
    more_lessons: bool,
    section_id: Option<Uuid>,
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
    #[prop_or(None)]
    pub inventory_group: Option<Uuid>,
    pub class_name: String,
}

#[derive(Debug)]
pub enum LessonListMessage {
    FetchLessonsByGroupId,
    Lessons(Option<lesson_model::get_lesson_list::ResponseData>),
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

    fn create(ctx: &Context<Self>) -> Self {
        ctx.link().send_message(LessonListMessage::FetchLessonsByGroupId);

        LessonList {
            graphql_task: Some(GraphQLService::connect(&code_location!())),
            lesson_sub: None,
            lesson_delete_task: None,
            lesson_add_task: None,
            lesson_list: vec![],
            lesson_list_view: vec![],
            show_dropdown_filter: false,
            filter: LessonFilter::Alls,
            more_lessons: false,
            section_id: None,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        info!("{:?}", msg);
        let should_update = true;
        match msg {
            LessonListMessage::FetchLessonsByGroupId => {
                if let Some(graphql_task) = self.graphql_task.as_mut() {

                    let vars = lesson_model::get_lesson_list::Variables {
                        group_id: ctx.props().group_id.0,
                    };

                    let task = lesson_model::GetLessonList::subscribe(
                            graphql_task,
                            &ctx,
                            vars,
                            |response| {
                                LessonListMessage::Lessons(response)
                            },
                    );
                    self.lesson_sub = Some(task);
                }
            }
            LessonListMessage::Lessons(response) => { 
                if let Some(class_group) = response.clone().and_then(|data| Some(data.class_group)) {

                    
                    for class_lesson in class_group.iter() {
                        self.section_id = class_lesson.class_profile.clone().and_then(|data| Some(data.section_id));

                        let lessons = class_lesson.class_profile.clone().and_then(|data| Some(data.class_lesson)).unwrap_or(vec![]);

                        let school_name = class_lesson.school_group.clone().and_then(|data| data.school.school_profile).and_then(|school| Some(school.name)).unwrap_or(String::new());
                        let school_logo = class_lesson.school_group.clone().and_then(|data| data.school.school_profile).and_then(|school| school.logo).unwrap_or(String::new());
                        let lesson_list = lessons.iter().map(|item| {

                            let timestamp = item.lesson_profile.timestamp;
                            
                            let time_fn = get_creation_date(timestamp);
    
                            let my_id = ctx.props().user_profile.clone().and_then(|user_by_pk| Some(user_by_pk.user_id)).unwrap_or(UserId(Uuid::default()));
                            let author_id = if item.lesson_profile.author_id.clone() == my_id.0 {
                                true
                            } else {
                                false
                            };
    
                            let lesson_type = item.lesson_profile.lesson_type.clone().unwrap_or(get_lesson_list::RoboxLessonTypeEnum::Extra);

                            LessonProfile {
                                title: item.lesson_profile.title.clone(), 
                                timestamp: time_fn, 
                                lesson_id: LessonId(item.lesson_profile.lesson_id), 
                                author_full_name: item.lesson_profile.author.user_profile.clone().and_then(|user_profile| Some(user_profile.full_name)).unwrap_or("".to_string()), 
                                author_pic_path: item.lesson_profile.author.user_profile.clone().and_then(|user_profile| user_profile.pic_path).unwrap_or("".to_string()), 
                                author_id,
                                archived: item.lesson_profile.lesson_group.clone().and_then(|data| Some(data.archived)).unwrap_or(false), 
                                send_to_degree: item.lesson_profile.lesson_group.clone().and_then(|data| Some(data.send_to_grade)).unwrap_or(false), 
                                school_name: school_name.clone(), 
                                school_logo: school_logo.clone(), 
                                content: item.lesson_profile.lesson_content.clone().and_then(|data| Some(data.content)).unwrap_or("".to_string()),
                                on_dropdown_menu: false,
                                lesson_type,
                            }
                        }).collect();
                        
                        self.lesson_list = lesson_list;
                        
                    }

                    ctx.link().send_message(LessonListMessage::ChangeFilter(self.filter.clone()));
                }
            }
            LessonListMessage::AddLesson(lesson_id) => {
                if let (Some(section_id), Some(graphql_task)) = (self.section_id, self.graphql_task.as_mut()) {
                    
                    let vars = lesson_model::lesson_class_and_group_add::Variables { 
                        group_id: ctx.props().group_id.0,
                        lesson_id: lesson_id.0,
                        section_id
                    };

                    let task = lesson_model::LessonClassAndGroupAdd::request(
                        graphql_task,
                        &ctx,
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
                        group_id: ctx.props().group_id.0,
                        lesson_id: lesson_id.0,
                    };

                    let task = lesson_model::LessonGroupDelete::request(
                        graphql_task,
                        &ctx,
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

                    let vars = lesson_model::delete_lesson::Variables { 
                        lesson_id: lesson_id.0,
                    };

                    let task = lesson_model::DeleteLesson::request(
                        graphql_task,
                        &ctx,
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
                if let (Some(section_id), Some(inventory_group_id), Some(graphql_task)) = (self.section_id, ctx.props().inventory_group, self.graphql_task.as_mut()) {

                    let local = chrono::Local::now().naive_local();

                    let type_lesson = lesson_class_and_group_create::RoboxLessonTypeEnum::Extra;

                    let vars = lesson_model::lesson_class_and_group_create::Variables { 
                        title: String::from(lang::dict("~ New Lesson ~")),
                        content: String::from(""),
                        group_id: ctx.props().group_id.0,
                        inventory_group_id,
                        lesson_id: Uuid::new_v4(),
                        timestamp: local,
                        lesson_type: type_lesson,
                        section_id
                    };

                    let task = lesson_model::LessonClassAndGroupCreate::request(
                        graphql_task,
                        &ctx,
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
            LessonListMessage::LessonAdded(lesson_id) => {
                let group_id = ctx.props().group_id;
                let school_id = ctx.props().school_id;

                if let Some(lesson_id) = lesson_id {
                    ctx.link().navigator().unwrap().push(&AppRoute::Lesson{school_id, group_id, lesson_id});
                }
            }
            LessonListMessage::LessonRemoved(lesson_id) => {
                info!("Remove Lesson {:?}", lesson_id);

            }
            LessonListMessage::ShowDropdown => {
                self.show_dropdown_filter = !self.show_dropdown_filter;
            }
            LessonListMessage::ChangeFilter(filter) => {
                self.filter = filter;
                self.show_dropdown_filter = false;

                let lessons_clone = self.lesson_list.clone();

                let lessons: Vec<LessonProfile> = lessons_clone.iter().filter(|filter| {
                    self.filter == LessonFilter::Alls && {filter.archived == true || filter.archived == false || filter.send_to_degree == true || filter.send_to_degree == false} ||
    
                    self.filter == LessonFilter::Published && filter.send_to_degree == true && filter.archived == false ||
    
                    self.filter == LessonFilter::Unpublished && filter.archived == false && filter.send_to_degree == false
                })
                .cloned()
                .collect();

                info!("FILTER {:?} <-----> LESSONS - VIEW {:?} ", self.filter, lessons);

                self.lesson_list_view = lessons;

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

    fn changed(&mut self, ctx: &Context<Self>, old_props: &Self::Properties) -> bool {
        info!("{:?} => {:?}", ctx.props(), old_props);

        
        ctx.props() != old_props
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let group_id = ctx.props().group_id;
        let on_alls = ctx.link().callback(|_| LessonListMessage::ChangeFilter(LessonFilter::Alls));
        let on_published = ctx.link().callback(|_| LessonListMessage::ChangeFilter(LessonFilter::Published));
        let on_unpublished = ctx.link().callback(|_| LessonListMessage::ChangeFilter(LessonFilter::Unpublished));
        // let on_archived = ctx.link().callback(|_| LessonListMessage::ChangeFilter(LessonFilter::Archived));
        let on_dropdown = ctx.link().callback(|_| LessonListMessage::ShowDropdown);
        let on_more_lessons = ctx.link().callback(|_| LessonListMessage::ShowMoreLessons);
        let on_change_list = ctx.link().callback(|(lesson_id, send_to_grade, archived)| LessonListMessage::UpdateLessonList(lesson_id, send_to_grade, archived));
        let on_lesson_delete = ctx.link().callback(|lesson_id| LessonListMessage::RemoveLesson(lesson_id));
        let on_del_lesson_entirely = ctx.link().callback(|lesson_id| LessonListMessage::RemoveLessonEntirely(lesson_id));

        let teaching_cards_count = self.lesson_list.iter().filter(|lesson| lesson.lesson_type == get_lesson_list::RoboxLessonTypeEnum::TeachingCards).count();
        let lesson_electronics_count = self.lesson_list.iter().filter(|lesson| lesson.lesson_type == get_lesson_list::RoboxLessonTypeEnum::ElectronicsLessons).count();
        let others_lessons = self.lesson_list.iter().filter(|item| item.author_id == true && item.lesson_type == get_lesson_list::RoboxLessonTypeEnum:: Extra).count();
        let others_lessons_student = self.lesson_list.iter().filter(|item| item.lesson_type == get_lesson_list::RoboxLessonTypeEnum:: Extra).count();

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
        let maybe_dropdown_by_user = ctx
            .props()
            .user_profile
            .as_ref()
            .and_then(|user|{
                if user.user_staff.is_some() || user.user_teacher.is_some() {
                    Some(html! {
                        <div class="dropdown me-5">
                            <button class={class_dropdown} type="button" id="dropdownMenuButton2" data-bs-toggle="dropdown" aria-expanded="false" onclick={on_dropdown}>
                                <img src="/icons/filter.svg" style="height: 22px;" />
                                <span class="universal-select-option text-secondary-purple noir-regular is-size-18 lh-22">{lang::dict(maybe_option_seleted)}</span>
                            </button>
                            <ul class={class_dropdown_list} aria-labelledby="dropdownMenuButton2">
                                <li>
                                    <a class="dropdown-item d-flex flex-wrap align-items-center mt-1 pe-0" onclick={on_alls}>
                                        <input class="bg-checkbox" type="checkbox" checked={if self.filter == LessonFilter::Alls {true} else {false}} />
                                        <span class={if self.filter == LessonFilter::Alls {"text-blue-purple noir-regular is-size-18 lh-22 ps-2"} else {"text-gray-purple noir-regular is-size-18 lh-22 ps-2"}}>{lang::dict("Everyone")}</span>
                                    </a>
                                </li>
                                <li>
                                    <a class="dropdown-item d-flex flex-wrap align-items-center pe-0" onclick={on_published}>
                                        <input class="bg-checkbox" type="checkbox" checked={if self.filter == LessonFilter::Published {true} else {false}} />
                                        <span class={if self.filter == LessonFilter::Published {"text-blue-purple noir-regular is-size-18 lh-22 ps-2"} else {"text-gray-purple noir-regular is-size-18 lh-22 ps-2"}}>{lang::dict("Released")}</span>
                                    </a>
                                </li>
                                <li>
                                    <a class="dropdown-item d-flex flex-wrap align-items-center pe-0" onclick={on_unpublished}>
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

        let maybe_lesson_staff = self.lesson_list_view.iter()
            // .filter(| lessons | {

            // self.filter == LessonFilter::Alls && lessons.lesson_type == get_lesson_list::RoboxLessonTypeEnum::TeachingCards && {lessons.archived == true || lessons.archived == false || lessons.send_to_degree == true || lessons.send_to_degree == false} ||

            // self.filter == LessonFilter::Published && lessons.lesson_type == get_lesson_list::RoboxLessonTypeEnum::TeachingCards && lessons.send_to_degree == true && lessons.archived == false ||

            // self.filter == LessonFilter::Unpublished && lessons.lesson_type == get_lesson_list::RoboxLessonTypeEnum::TeachingCards && lessons.archived == false && lessons.send_to_degree == false

            // })
            .map(|item| {
                let lesson_profile = item.clone();
                html! {
                <>
                    <LessonCard lesson_id={item.lesson_id.clone()}
                        user_profile={ctx.props().user_profile.clone()}
                        group_id={ctx.props().group_id}
                        on_lesson_delete={on_lesson_delete.clone()}
                        on_del_lesson_entirely={&on_del_lesson_entirely}
                        on_change_list={on_change_list.clone()}
                        lesson_profile={lesson_profile}
                        archived={item.archived}
                        send_to_grade={item.send_to_degree}
                        school_id={ctx.props().school_id} />
                </>
            }
        }).collect::<Html>();

        let maybe_lesson_staff_electronic = self.lesson_list_view.iter()
            .filter(| lessons | {

                lessons.lesson_type == get_lesson_list::RoboxLessonTypeEnum::ElectronicsLessons
            // self.filter == LessonFilter::Alls && lessons.lesson_type == get_lesson_list::RoboxLessonTypeEnum::ElectronicsLessons && {lessons.archived == true || lessons.archived == false || lessons.send_to_degree == true || lessons.send_to_degree == false} ||

            // self.filter == LessonFilter::Published && lessons.lesson_type == get_lesson_list::RoboxLessonTypeEnum::ElectronicsLessons && lessons.send_to_degree == true && lessons.archived == false ||

            // self.filter == LessonFilter::Unpublished && lessons.lesson_type == get_lesson_list::RoboxLessonTypeEnum::ElectronicsLessons && lessons.archived == false && lessons.send_to_degree == false

            })
            .map(|item| {
                let lesson_profile = item.clone();

                html! {
                <>
                    <LessonCard lesson_id={item.lesson_id.clone()}
                        user_profile={ctx.props().user_profile.clone()}
                        group_id={ctx.props().group_id}
                        on_lesson_delete={on_lesson_delete.clone()}
                        on_del_lesson_entirely={&on_del_lesson_entirely}
                        on_change_list={on_change_list.clone()}
                        lesson_profile={lesson_profile}
                        archived={item.archived}
                        send_to_grade={item.send_to_degree}
                        school_id={ctx.props().school_id} />
                </>
            }
        }).collect::<Html>();


        let maybe_your_lessons = self
            .lesson_list_view
            .iter()
            .filter(| lessons | {

                lessons.lesson_type == get_lesson_list::RoboxLessonTypeEnum::Extra
            //     self.filter == LessonFilter::Alls && lessons.lesson_type == get_lesson_list::RoboxLessonTypeEnum::Extra && {lessons.archived == true || lessons.archived == false || lessons.send_to_degree == true || lessons.send_to_degree == false} && {lessons.author_id == true} ||

            //     self.filter == LessonFilter::Published && lessons.lesson_type == get_lesson_list::RoboxLessonTypeEnum::Extra && lessons.send_to_degree == true && lessons.archived == false && {lessons.author_id == true} ||

            //     self.filter == LessonFilter::Unpublished && lessons.lesson_type == get_lesson_list::RoboxLessonTypeEnum::Extra && lessons.archived == false && lessons.send_to_degree == false && {lessons.author_id == true}

            })
            .map(|item| {
            let lesson_profile = item.clone();

            html! {
                <LessonCard lesson_id={item.lesson_id.clone()}
                    user_profile={ctx.props().user_profile.clone()} 
                    group_id={group_id}
                    on_lesson_delete={on_lesson_delete.clone()}
                    on_del_lesson_entirely={&on_del_lesson_entirely}
                    on_change_list={on_change_list.clone()}
                    lesson_profile={lesson_profile}
                    archived={item.archived}
                    send_to_grade={item.send_to_degree}
                    school_id={ctx.props().school_id} />
            }
        }).collect::<Html>();

        let maybe_electronic_by_student = self.lesson_list_view.iter()
            // .filter(| lessons | {

            // self.filter == LessonFilter::Alls && lessons.lesson_type == get_lesson_list::RoboxLessonTypeEnum::ElectronicsLessons && lessons.send_to_degree == true && lessons.archived == false

            // })
            .map(|item| {
                let lesson_profile = item.clone();

                html! {
                <>
                    <LessonCard lesson_id={item.lesson_id.clone()}
                        user_profile={ctx.props().user_profile.clone()}
                        group_id={ctx.props().group_id}
                        on_lesson_delete={on_lesson_delete.clone()}
                        on_del_lesson_entirely={&on_del_lesson_entirely}
                        on_change_list={on_change_list.clone()}
                        lesson_profile={lesson_profile}
                        archived={item.archived}
                        send_to_grade={item.send_to_degree}
                        school_id={ctx.props().school_id} />
                </>
            }
        }).collect::<Html>();

        let maybe_other_lessons_by_student = self.lesson_list_view.iter()
            // .filter(| lessons | {

            // self.filter == LessonFilter::Alls && lessons.lesson_type == get_lesson_list::RoboxLessonTypeEnum::Extra && lessons.send_to_degree == true && lessons.archived == false

            // })
            .map(|item| {
                let lesson_profile = item.clone();

                html! {
                <>
                    <LessonCard lesson_id={item.lesson_id.clone()}
                        user_profile={ctx.props().user_profile.clone()}
                        group_id={ctx.props().group_id}
                        on_lesson_delete={on_lesson_delete.clone()}
                        on_del_lesson_entirely={&on_del_lesson_entirely}
                        on_change_list={on_change_list.clone()}
                        lesson_profile={lesson_profile}
                        archived={item.archived}
                        send_to_grade={item.send_to_degree}
                        school_id={ctx.props().school_id} />
                </>
            }
        }).collect::<Html>();

        let maybe_lesson_search = ctx
            .props()
            .user_profile
            .as_ref()
            .and_then(|item| {
                let on_select = ctx.link().callback(|select_option| match select_option {
                    LessonSelectOption::Lesson(lesson_id) => { LessonListMessage::AddLesson(lesson_id) }
                });
                if item.user_staff.is_some() || item.user_teacher.is_some() {
                    Some(html! {
                        <LessonSelect on_select={on_select} 
                            allow_create={true}
                            school_id={ctx.props().school_id} />
                    })
                } else {
                    None
                }
            })
            .unwrap_or(html! {});

        let class_name = ctx.props().class_name.clone().to_uppercase();

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
                        <a onclick={on_more_lessons}>
                            <span class="text-secondary-purple noir-bold is-size-18 lh-22 pt-5">{lang::dict("See All Electronics Lessons")}</span>
                        </a>
                    </div>
                </>
            }
        } else {
            html! {
                <>
                    <div class="d-flex justify-content-center">
                        <a onclick={on_more_lessons}>
                            <span class="text-secondary-purple noir-bold is-size-18 lh-22 pt-5">{lang::dict("See All Teacher Resources")}</span>
                        </a>
                    </div>
                </>
            }
        };


        let maybe_user_profile_pic = ctx
            .props()
            .user_profile
            .as_ref()
            .and_then(|user_profile| Some(user_profile.pic_path.clone()))
            .and_then(|pic_path| {
                Some(html! {
                    <img class="img-card-72" src={pic_path.clone()} alt="photo of user" />
                })
            })
            .unwrap_or(html! {<img class="img-card-72" src="/static/avatar.png"/>
            });

        let navigator = ctx.link().navigator().unwrap();
        let on_direct_meet = Callback::from(move |_| navigator.push(&AppRoute::MeetDirect{group_id}));

        let head_section = html! {
            <div class="d-flex flex-wrap align-items-center justify-content-between mb-5">
                <h1 class="text-primary-blue-dark text-uppercase noir-bold is-size-36 lh-43 mb-0">
                    {ctx.props().class_name.clone()}
                </h1>
                <a class="btn btn-outline-light text-primary-blue-dark noir-regular is-size-18 lh-22" onclick={on_direct_meet}>
                    <img class="me-3" src="/icons/video-2.svg" style="height: 30px;" />
                    <span>{lang::dict("Meet up")}</span>
                </a>
                {maybe_lesson_search}
                {maybe_user_profile_pic}
            </div>
        };

        let maybe_new = ctx
            .props()
            .user_profile
            .as_ref()
            .and_then(|item| {
                let on_select = ctx.link().callback(move |_| LessonListMessage::CreateLesson);
                if item.user_staff.is_some() || item.user_teacher.is_some() {
                    Some(html! {
                        <a class="button btn-create-card bg-primary-blue-dark d-flex align-items-center justify-content-center" onclick={on_select.clone()}>
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
        let maybe_option_staff = if self.lesson_list_view.iter().count() > 0 {
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
        let maybe_option_staff_electronic = if self.lesson_list_view.iter().count() > 0 {
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

        let maybe_option_teacher = if self.lesson_list_view.iter().filter(|item| item.author_id == true && item.lesson_type == get_lesson_list::RoboxLessonTypeEnum:: Extra).count() > 0 {
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

        let by_user_profile_view = ctx
            .props()
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