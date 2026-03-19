use log::*;
use std::vec;
use uuid::Uuid;
use yew::prelude::*;
use roboxmaker_main::lang;
use code_location::code_location;
use crate::activity_card_classes::ActivityCardClasses;
use crate::{activity_card::ActivityCard, ActivityStyle};

use roboxmaker_models::activity_model;
use roboxmaker_graphql::{GraphQLService, GraphQLTask, Subscribe, SubscriptionTask};
use roboxmaker_types::types::{ClassesId, UserId, GroupId, AppRoute, ActivityId, MyUserProfile};

#[derive(Clone, Debug, PartialEq)]
pub struct ActivityProfile {
    pub activity_id: ActivityId,
    pub user_id: UserId,
    pub full_name: String,
    pub pic_path: String,
    pub user_staff: Option<Uuid>,
    pub user_teacher: Option<Uuid>,
    pub user_student: Option<Uuid>,
    pub timestamp: String,
    pub title: String,
    pub score: i64,
    pub deliver: String,
    pub content: String,
}

pub struct ActivityList {
    link: ComponentLink<Self>,
    props: ActivityListProperties,
    graphql_task: Option<GraphQLTask>,
    activity_list_task: Option<SubscriptionTask>,
    activity: Vec<ActivityProfile>,
}

#[derive(Debug, Properties, Clone, PartialEq)]
pub struct ActivityListProperties {
    pub on_app_route: Callback<AppRoute>,
    pub user_id: Option<UserId>,
    pub user_profile: Option<MyUserProfile>,
    pub classes_id: ClassesId,
    pub group_id: GroupId,
    pub maybe_style: ActivityStyle,
}

#[derive(Debug)]
pub enum ActivityListMessage {
    FetchActivityByClassesGroup,
    Activities(Option<activity_model::activity_by_classes_group::ResponseData>),
}

impl Component for ActivityList {
    type Message = ActivityListMessage;
    type Properties = ActivityListProperties;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        link.send_message(ActivityListMessage::FetchActivityByClassesGroup);
        ActivityList {
            link,
            props,
            graphql_task: Some(GraphQLService::connect(&code_location!())),
            activity_list_task: None,
            activity: vec![],
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        info!("{:?}", msg);
        let should_update = true;
        match msg {
            ActivityListMessage::FetchActivityByClassesGroup => {
                if let Some(graphql_task) = self.graphql_task.as_mut() {
                    let vars = activity_model::activity_by_classes_group::Variables {
                        group_id: self.props.group_id.0,
                        classes_id: self.props.classes_id.0,
                    };

                    let task = activity_model::ActivityByClassesGroup::subscribe(
                            graphql_task,
                            &self.link,
                            vars,
                            |response| {
                                ActivityListMessage::Activities(response)
                            },
                    );
                    self.activity_list_task = Some(task);
                }
            }
            ActivityListMessage::Activities(activity) => {
                self.activity = activity
                    .clone()
                    .and_then(|data| Some(data.activity_profile))
                    .unwrap_or(vec![])
                    .iter()
                    .map(|item| {
                        let user_staff = item.author.clone().and_then(|data| data.user_staff).clone().and_then(|user| Some(user.user_id));
                        let user_teacher = item.author.clone().and_then(|data| data.user_teacher).clone().and_then(|user| Some(user.user_id));
                        let user_student = item.author.clone().and_then(|data| data.user_student).clone().and_then(|user| Some(user.user_id));
                        ActivityProfile {
                            activity_id: ActivityId(item.activity_id),
                            user_id: UserId(item.author.clone().and_then(|data| Some(data.user_id)).unwrap_or(Uuid::default())),
                            full_name: item.author.clone().and_then(|data| Some(data.full_name)).unwrap_or("".to_string()),
                            pic_path: item.author.clone().and_then(|data| data.pic_path).unwrap_or("".to_string()),
                            user_staff: user_staff,
                            user_teacher: user_teacher,
                            user_student: user_student,
                            timestamp: item.timestamp.format("%e-%b-%Y").to_string(),
                            title: item.title.clone().unwrap_or("".to_string()),
                            score: item.score,
                            deliver: item.deliver.format("%e-%b-%Y").to_string(),
                            content: item.activity_content.clone().and_then(|data| Some(data.content)).unwrap_or("".to_string()),
                        }
                    }).collect();
            }
        }
        should_update
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        info!("{:?} => {:?}", self.props, props);
        let mut should_render = false;

        if self.props.classes_id != props.classes_id {
            self.link.send_message(ActivityListMessage::FetchActivityByClassesGroup);
        }

        if self.props != props {
            self.props = props;
            should_render = true;
        } 

        should_render
    }

    fn view(&self) -> Html {
        let user_profile = self.props.user_profile.clone();
        let activity = |activity_profile: &ActivityProfile | {
            html! {
                <ActivityCard on_app_route=self.props.on_app_route.clone()
                user_profile=user_profile.clone()
                group_id=self.props.group_id
                classes_id=self.props.classes_id
                maybe_style=self.props.maybe_style
                activity_profile=activity_profile.clone() />
            }
        };
        let activity_classes = |activity_profile: &ActivityProfile | {
            html! {
                <ActivityCardClasses group_id=self.props.group_id
                    classes_id=self.props.classes_id
                    maybe_style=self.props.maybe_style
                    activity_profile=activity_profile.clone() />
            }
        };
        let maybe_add = html! {
                <ActivityCard on_app_route=self.props.on_app_route.clone()
                user_profile=user_profile.clone()
                group_id=self.props.group_id
                classes_id=self.props.classes_id
                maybe_style=self.props.maybe_style
                activity_profile=None />
        };
        let maybe_activities = {
            if self.activity.len() > 0 {
                html! {
                    <>
                        {maybe_add}
                        {
                            self.activity
                                .iter()
                                .map(|activity_profile| {
                                    activity(activity_profile)
                            }).collect::<Html>()
                        }
                        {
                            self.activity
                                .iter()
                                .map(|activity_profile| {
                                    activity_classes(activity_profile)
                            }).collect::<Html>()
                        }
                    </>
                }
            } else {
                html! {
                    <>
                        {maybe_add}
                        <div class="text-center">
                            <span class="text-gray-strong is-size-18 lh-20">{lang::dict("No activities here")}</span>
                        </div>
                    </>
                }
            }
        };
        html! {maybe_activities}
    }
}
