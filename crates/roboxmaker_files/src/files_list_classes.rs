use log::*;
use std::vec;
use yew::prelude::*;
use code_location::code_location;
use yew::{html, Component, ComponentLink, Html, ShouldRender};

use roboxmaker_models::files_model;
use roboxmaker_types::types::ClassesId;
use roboxmaker_graphql::{GraphQLService, GraphQLTask, Request, RequestTask};

pub struct FilesListClasses {
    link: ComponentLink<Self>,
    props: FilesListClassesProps,
    graphql_task: Option<GraphQLTask>,
    files_list_task: Option<RequestTask>,
    files_list: Vec<files_model::files_title_by_classes_id::FilesTitleByClassesIdFilesProfile>,
}

#[derive(Debug, Properties, Clone, PartialEq)]
pub struct FilesListClassesProps {
    pub classes_id: ClassesId
}

#[derive(Debug)]
pub enum FilesListClassesMessage {
    FetchFilesNames,
    Files(Option<files_model::files_title_by_classes_id::ResponseData>),
}

impl Component for FilesListClasses {
    type Message = FilesListClassesMessage;
    type Properties = FilesListClassesProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        link.send_message(FilesListClassesMessage::FetchFilesNames);
        FilesListClasses {
            link,
            props,
            graphql_task: Some(GraphQLService::connect(&code_location!())),
            files_list_task: None,
            files_list: vec![],
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        info!("{:?}", msg);
        let should_update = true;
        match msg {
            FilesListClassesMessage::FetchFilesNames => {
                if let Some(graphql_task) = self.graphql_task.as_mut() {
                    let vars = files_model::files_title_by_classes_id::Variables {
                        classes_id: self.props.classes_id.0, 
                    };

                    let task = files_model::FilesTitleByClassesId::request(
                            graphql_task,
                            &self.link,
                            vars,
                            |response| {
                                FilesListClassesMessage::Files(response)
                            },
                    );
                    self.files_list_task = Some(task);
                }
            }
            FilesListClassesMessage::Files(files_list) => {
                self.files_list = files_list.clone().and_then(|data| Some(data.files_profile)).unwrap_or(vec![])
            }
        }
        should_update
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        info!("{:?} => {:?}", self.props, props);
        let should_render = false;
        if self.props != props {
            self.props = props;
            true;
        } else {
            false;
        }
        should_render
    }

    fn view(&self) -> Html {
        let list_files = self.files_list.iter().map(|files| {
            let title = files.title.clone();
            html! {
                <>
                    <span class="text-primary-blue-dark noir-bold is-size-18 lh-22 mt-4">
                        <i class="fas fa-paperclip"></i>
                        <span class="ps-2">{&title}</span>
                    </span>
                </>
            }
        }).collect::<Html>();
        let maybe_files = {
            if self.files_list.len() > 0 {
                html! {
                    {list_files}
                }
            } else {
                html! {
                    <div class="text-center">
                        <span class="is-size-5">{"No hay recursos"}</span>
                    </div>
                }
            }
        };
        html! {
            {maybe_files}
        }
    }
}
