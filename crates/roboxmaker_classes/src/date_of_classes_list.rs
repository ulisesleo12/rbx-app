use log::*;
use yew::prelude::*;
use code_location::code_location;
use yew::{html, Component, Html};

use roboxmaker_models::classes_model;
use roboxmaker_types::types::ClassesId;
use roboxmaker_graphql::{GraphQLService, GraphQLTask, Request, RequestTask};

pub struct DateOfClassesList {
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

    fn create(ctx: &Context<Self>) -> Self {

        ctx.link().send_message(DateOfClassesListMessage::FetchActivityByClassesId(ctx.props().classes_id));

        DateOfClassesList {
            graphql_task: Some(GraphQLService::connect(&code_location!())),
            date_classes_task: None,
            activity_date: vec![],
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
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
                            &ctx,
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

    fn changed(&mut self, ctx: &Context<Self>, old_props: &Self::Properties) -> bool {
        info!("{:?} => {:?}", ctx.props(), old_props);
        let mut should_render = false;

        if ctx.props().classes_id != old_props.classes_id {
            ctx.link().send_message(DateOfClassesListMessage::FetchActivityByClassesId(ctx.props().classes_id));
        }

        if ctx.props() != old_props {
            should_render = true;
        } 
        
        should_render
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
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
