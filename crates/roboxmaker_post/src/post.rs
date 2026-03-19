use log::*;
use uuid::Uuid;
use yew::prelude::*;
use crate::post_page::PostPage;
use code_location::code_location;
use yew::{html, Component, ComponentLink, Html, ShouldRender};

use roboxmaker_models::post_model;
use roboxmaker_graphql::{GraphQLService, GraphQLTask, Subscribe, SubscriptionTask};
use roboxmaker_types::types::{AppRoute, ClassGroupCategory, ClassGroupPost, GroupId, MyUserProfile, PageMode, PostId, SchoolId, UserId, PostPageContent};


#[derive(Debug)]
pub enum PostPageEdit {
    None,
    Edit,
    Save,
}

pub struct View {
    link: ComponentLink<Self>,
    props: PostPageProperties,
    graphql_task: Option<GraphQLTask>,
    task: Option<SubscriptionTask>,
    post: Option<PostPageContent>,
}

#[derive(Debug, Properties, Clone, PartialEq)]
pub struct PostPageProperties {
    pub on_app_route: Callback<AppRoute>,
    pub user_profile: Option<MyUserProfile>,
    pub post_id: PostId,
    pub group_id: GroupId,
    pub posts: Option<ClassGroupPost>,
    pub school_id: SchoolId,
    pub page_mode: PageMode,
    pub saved_sidebar_state: bool,
}

#[derive(Debug)]
pub enum Message {
    AppRoute(AppRoute),
    FetchPostById(PostId, GroupId),
    Post(Option<post_model::post_by_id::ResponseData>),
}

impl Component for View {
    type Message = Message;
    type Properties = PostPageProperties;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        link.send_message(Message::FetchPostById(props.post_id, props.group_id));

        View {
            link,
            props,
            graphql_task: Some(GraphQLService::connect(&code_location!())),
            task: None,
            post: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Message::AppRoute(route) => {
                self.props.on_app_route.emit(route)
            }
            Message::FetchPostById(post_id, group_id) => {
                if let Some(graphql_task) = self.graphql_task.as_mut() {
                    let vars = post_model::post_by_id::Variables { 
                        group_id: group_id.0,
                        post_id: post_id.0,
                    };

                    let task = post_model::PostById::subscribe(
                        graphql_task,
                        &self.link,
                        vars,
                        |response| {
                            Message::Post(response)
                        },
                    );
                    self.task = Some(task);
                }
            }
            Message::Post(response) => {
                let user_id = self.props.user_profile.clone().and_then(|item| Some(item.user_id)).unwrap_or(UserId(Uuid::default()));

                if let Some(resp) = response.clone().and_then(|data| data.post_group_by_pk) {
                    let class_name = resp.class_profile.clone().and_then(|data| data.class_profile).and_then(|class_profile| Some(class_profile.name)).unwrap_or_default();
                    let title = resp.post_profile.clone().and_then(|item| Some(item.topic)).unwrap_or_default();
                    let content = resp.post_content.clone().and_then(|item| Some(item.content)).unwrap_or_default();
                    let timestamp = resp.post_profile.clone().and_then(|item| Some(item.timestamp.format("%a %e %b %Y %T").to_string())).unwrap_or_default();
                    let author_user_id = resp.post_profile.clone().and_then(|item| item.author_profile).and_then(|author| Some(author.user_id)).unwrap_or(Uuid::default());
                    let author_full_name = resp.post_profile.clone().and_then(|item| item.author_profile).and_then(|author| Some(author.full_name)).unwrap_or_default();
                    let author_pic_path = resp.post_profile.clone().and_then(|item| item.author_profile).and_then(|author| author.pic_path).unwrap_or(String::from("https://files.roboxmaker.network/uploads/avatar.png"));
                    let published = resp.published;
                    let archived = resp.archived;
                    
                    self.post = Some(PostPageContent {
                        title,
                        content,
                        timestamp,
                        author_user_id,
                        author_full_name,
                        author_pic_path,
                        published,
                        archived,
                        class_name,
                    });

                    // if quiz_id.is_some() {
                    //     self.post_or_quiz = false
                    // }

                    // if let Some(quiz_json) = resp.post_profile.clone().and_then(|item| item.maybe_quiz) {
                    //     self.quiz = Some(handle::get_quiz(quiz_json.clone(), user_id.0));

                    //     if let Some(quiz) = &self.quiz {
                    //         self.quiz_state = quiz.state.clone();
                    //     }

                    //     info!(">>> Quiz: {:?}", self.quiz)
                    // }
                }
                let school_id = self.props.school_id;
                let group_id = self.props.group_id;
                
                if response.clone().and_then(|data| data.post_group_by_pk).is_none() {
                    if self.props.user_profile.clone().and_then(|item| Some(item.user_staff.is_some() || item.user_teacher.is_some())).unwrap_or(false) {
                        self.link.send_message(Message::AppRoute(AppRoute::SchoolGroupSection(school_id, group_id, ClassGroupCategory::Posts)));
                    } else {
                        self.link.send_message(Message::AppRoute(AppRoute::GroupSectionStudent(school_id, user_id, ClassGroupCategory::Posts)));
                    }
                }
            }
        };
        true
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

        if let Some(post_page_content) = &self.post {
            // let group_id = self.props.group_id;
            // let school_id = self.props.school_id;
            // let user_id = self.props.user_profile.clone().and_then(|item| Some(item.user_id)).unwrap_or(UserId(Uuid::default()));

            // let on_class_group_posts = self.link.callback(move |_| {
            //     Message::AppRoute(AppRoute::SchoolGroupSection(
            //         school_id.clone(),
            //         group_id.clone(),
            //         ClassGroupCategory::Posts,
            //     ))
            // });
            // let on_class_group_posts_st = self.link.callback(move |_| {
            //     Message::AppRoute(AppRoute::GroupSectionStudent(
            //         school_id.clone(),
            //         user_id.clone(),
            //         ClassGroupCategory::Posts,
            //     ))
            // });

            // let go_back_grade = self.props.user_profile.clone()
            //     .and_then(|item| {
            //         if item.user_teacher.is_some() || item.user_staff.is_some() {
            //             Some(html! {
            //                 <a onclick={ on_class_group_posts }>
            //                     <span class="icon-text text-gray-strong noir-medium is-size-16 lh-19 d-flex align-items-center">
            //                         <span class="icon">
            //                             <i class="fas fa-arrow-left"></i>
            //                         </span>
            //                         <span class="mx-2">{ lang::dict("To Publications") }</span>
            //                         { post_page_content.class_name.clone() }
            //                     </span>
            //                 </a>
            //             })
            //         } else {
            //             Some(html! {
            //                 <a onclick={ on_class_group_posts_st }>
            //                     <span class="icon-text text-gray-strong noir-medium is-size-16 lh-19 d-flex align-items-center">
            //                         <span class="icon">
            //                             <i class="fas fa-arrow-left"></i>
            //                         </span>
            //                         <span class="mx-2">{ lang::dict("To Publications") }</span>
            //                         { post_page_content.class_name.clone() }
            //                     </span>
            //                 </a>
            //             })
            //         }
            //     }).unwrap_or(html! {});

            html! {
                <PostPage
                    user_profile={ self.props.user_profile.clone() } 
                    on_app_route={ self.props.on_app_route.clone() } 
                    post_id={ self.props.post_id } 
                    group_id={ self.props.group_id }
                    school_id={ self.props.school_id }
                    page_mode={ self.props.page_mode }
                    post={ post_page_content.clone() }
                    saved_sidebar_state={ self.props.saved_sidebar_state } />
            }
            // if self.quiz.is_some() && self.quiz_state == String::from("CREATED") {
            //     html! {
            //         <div class="w-100 h-100 scroll-y p-3 p-md-5 p-lg-7">
            //             <div class="d-flex align-items-center justify-content-between">
            //                 { go_back_grade }
            //                 <div class="d-flex align-items-center flex-wrap">
            //                     <UserPic  user_profile={ self.props.user_profile.clone() }/>
            //                 </div>
            //             </div>
            //             <PostContentAuthor post_page_content={ post_page_content.clone() } />
            //             <quizresponder::QuizResponder 
            //                 quiz={ self.quiz.clone().unwrap() } 
            //                 user_profile={ self.props.user_profile.clone() }
            //                 post_id={ self.props.post_id }
            //                 group_id={ self.props.group_id }
            //                 school_id={ self.props.school_id }
            //                 quiz_id={ post_page_content.quiz_id } 
            //                 />
            //         </div>
            //     }
            // } else {
            //     {
            //         html! {
            //             {
            //                 if self.post_or_quiz {
            //                 } else {
            //                     html! {
            //                         <div class="w-100 h-100 scroll-y p-3 p-md-5 p-lg-7">
            //                             <div class="d-flex align-items-center justify-content-between">
            //                                 { go_back_grade }
            //                                 <div class="d-flex align-items-center flex-wrap">
            //                                     <UserPic  user_profile={ self.props.user_profile.clone() }/>
            //                                 </div>
            //                             </div>
            //                             <PostContentAuthor post_page_content={ post_page_content.clone() } />
            //                             <QuizApp
            //                                 quiz_mode={ QuizMode::Create }
            //                                 post_id={ self.props.post_id }
            //                                 group_id={ self.props.group_id }
            //                                 school_id={ self.props.school_id }
            //                                 quiz_id={ post_page_content.quiz_id } />
            //                         </div>
            //                     }
            //                 }
            //             }
            //         }
            //     }
            // }
        } else {
            html! {
                <progress class="progress is-small is-primary" max="100"></progress>
            }
        }
    }
}