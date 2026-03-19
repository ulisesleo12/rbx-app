use log::*;
use std::vec;
use yew::prelude::*;
use code_location::code_location;
use yew::{html, Component, ComponentLink, Html, ShouldRender};

use roboxmaker_models::{user_model, school_model, files_model};
use roboxmaker_types::types::{GroupId, FilesId, ClassGroupFiles, AppRoute};
use roboxmaker_graphql::{GraphQLService, GraphQLTask, Request, RequestTask};
use roboxmaker_loaders::placeholders::card_files_list::CardFilesListPlaceholder;

pub struct FilesUserCard {
    link: ComponentLink<Self>,
    props: FilesUserCardProps,
    graphql_task: Option<GraphQLTask>,
    files_id_task: Option<RequestTask>,
    files: Option<files_model::files_by_id::FilesByIdFilesByPk>,
}

#[derive(Debug, Properties, Clone, PartialEq)]
pub struct FilesUserCardProps {
    pub on_app_route: Callback<AppRoute>,
    pub user_profile: Option<MyUserProfile>,
    pub auth_school: Option<school_model::school_by_id::SchoolByIdSchoolByPk>,
    pub group_id: GroupId,
    pub files: ClassGroupFiles,
    pub on_files_delete: Option<Callback<FilesId>>,
    pub on_list_change: Option<Callback<()>>,
}

#[derive(Debug)]
pub enum FilesUserCardMessage {
    FetchFilesById(FilesId),
    Files(Option<files_model::files_by_id::ResponseData>),
    DeleteFiles(FilesId),
}

impl Component for FilesUserCard {
    type Message = FilesUserCardMessage;
    type Properties = FilesUserCardProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        link.send_message(FilesUserCardMessage::FetchFilesById(props.files.files_id));
        FilesUserCard {
            link,
            props,
            graphql_task: Some(GraphQLService::connect(&code_location!())),
            files_id_task: None,
            files: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        info!("{:?}", msg);
        let should_update = true;
        match msg {
            FilesUserCardMessage::FetchFilesById(files_id) => {

                if let Some(graphql_task) = self.graphql_task.as_mut() {
                    let vars = files_model::files_by_id::Variables {
                        files_id: files_id.0,
                    };

                    let task = files_model::FilesById::request(
                            graphql_task,
                            &self.link,
                            vars,
                            |response| {
                                FilesUserCardMessage::Files(response)
                            },
                    );
                    self.files_id_task = Some(task);
                }

            }
            FilesUserCardMessage::Files(files) => {
                self.files = files.clone().and_then(|data| data.files_by_pk)
            }
            FilesUserCardMessage::DeleteFiles(files_id) => {
                if let Some(on_files_delete) = &self.props.on_files_delete {
                    on_files_delete.emit(files_id);
                    self.link.send_message(FilesUserCardMessage::FetchFilesById(self.props.files.files_id));
                }
            }
        }
        should_update
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        info!("{:?} => {:?}", self.props, props);
        let mut should_render = false;

        if self.props.files.files_id != props.files.files_id {
            self.link.send_message(FilesUserCardMessage::FetchFilesById(props.files.files_id));
        }
        if self.props != props {
            self.props = props;
            should_render = true;
        } 
        
        should_render
    }

    fn view(&self) -> Html {
        let files_id = self.props.files.files_id;
        let on_delete_file = self.link.callback(move |_| FilesUserCardMessage::DeleteFiles(files_id));
        let files_profile = self
            .files
            .as_ref()
            .and_then(|files| files.files_profile.as_ref())
            .and_then(|files_profile| {
                Some(html! {
                    <div class="card-messages-view-files-user bg-white d-flex align-items-center justify-content-between px-3 mb-4 py-3">
                        <span class="d-flex align-items-center text-primary-blue-dark noir-bold is-size-16 lh-22">
                            <span class="me-3">
                                <i class="fas fa-paperclip"></i>
                            </span>
                            {&files_profile.title}
                        </span>
                        <div class="d-flex align-items-center">
                            <span class="text-brown noir-light is-size-16 lh-19 pe-4">{&files_profile.timestamp.format("%a %b-%e-%Y").to_string()}</span>
                            <button class="btn bg-white text-purple-gray" onclick=on_delete_file>
                                <i class="far fa-trash-alt fas fa-lg"></i>
                            </button>
                        </div>
                    </div>
                })
            }).unwrap_or(html! {
                <CardFilesListPlaceholder />
            });
        html! {
            // <CardFilesListPlaceholder />
            {files_profile}
        }
    }
}
