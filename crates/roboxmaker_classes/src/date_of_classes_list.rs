use log::*;
use yew::prelude::*;
use code_location::code_location;
use yew::{html, Component, ComponentLink, Html, ShouldRender};

use roboxmaker_models::classes_model;
use roboxmaker_types::types::ClassesId;
use roboxmaker_graphql::{GraphQLService, GraphQLTask, Request, RequestTask};

pub struct DateOfClassesList {
    link: ComponentLink<Self>,
    props: DateOfClassesListProps,
    graphql_task: Option<GraphQLTask>,
    date_classes_task: Option<RequestTask>,
    activity_date: Vec<classes_model::date_activity_classes_by_id::DateActivityClassesByIdActivityGroup>,
}

#[derive(Debug, Properties, Clone, PartialEq)]
pub struct DateOfClassesListProps {
    pub classes_id: ClassesId,
}

#[derive(Debug)]
pub enum DateOfClassesListMessage {
    FetchActivityByClassesId(ClassesId),
    ActivityResponse(Option<classes_model::date_activity_classes_by_id::ResponseData>),
}

impl Component for DateOfClassesList {
    type Message = DateOfClassesListMessage;
    type Properties = DateOfClassesListProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        link.send_message(DateOfClassesListMessage::FetchActivityByClassesId(props.classes_id));
        DateOfClassesList {
            link,
            props,
            graphql_task: Some(GraphQLService::connect(&code_location!())),
            date_classes_task: None,
            activity_date: vec![],
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        info!("{:?}", msg);
        let should_update = true;
        match msg {
            DateOfClassesListMessage::FetchActivityByClassesId(classes_id) => {
                if let Some(graphql_task) = self.graphql_task.as_mut() {

                    let vars = classes_model::date_activity_classes_by_id::Variables {
                        classes_id: classes_id.0,
                        limit: 1,
                    };

                    let task = classes_model::DateActivityClassesById::request(
                            graphql_task,
                            &self.link,
                            vars,
                            |response| {
                                DateOfClassesListMessage::ActivityResponse(response)
                            },
                    );
                    self.date_classes_task = Some(task);
                }
            }
            DateOfClassesListMessage::ActivityResponse(activity_date) => {
                self.activity_date = activity_date.clone().and_then(|data| Some(data.activity_group)).unwrap_or(vec![]);
            }
        }
        should_update
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        info!("{:?} => {:?}", self.props, props);
        let mut should_render = false;

        if self.props.classes_id != props.classes_id {
            self.link.send_message(DateOfClassesListMessage::FetchActivityByClassesId(props.classes_id));
        }

        if self.props != props {
            self.props = props;
            should_render = true;
        } 
        
        should_render
    }

    fn view(&self) -> Html {
        let date = self
            .activity_date
            .iter()
            .map(|activity | {
                let date = activity.activity_profile.clone().and_then(|data| Some(data.deliver.format("%d-%m-%Y").to_string())).unwrap_or("01-01-2022".to_string());
                html! {
                    <span class="text-brown noir-light is-size-13 lh-22  d-flex align-items-center">
                        <i class="far fa-clock me-1"></i>
                        <span class="text-brown noir-light is-size-13 lh-22 ">{"Hasta "}{&date}</span>
                    </span>
                }
            })
            .collect::<Html>();
        html! {
            {date}
        }
    }
}
