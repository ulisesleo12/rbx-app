use log::*;
use std::vec;
use yew::prelude::*;
use code_location::code_location;
use yew::{html, Component, ComponentLink, Html, ShouldRender};

use roboxmaker_models::{user_model, school_model, files_model};
use roboxmaker_types::types::{GroupId, FilesId, ClassGroupFiles, AppRoute};
use roboxmaker_graphql::{GraphQLService, GraphQLTask, Request, RequestTask};

pub struct FilesCard {
    link: ComponentLink<Self>,
    props: FilesCardProps,
    graphql_task: Option<GraphQLTask>,
    files_id_task: Option<RequestTask>,
    files: Option<files_model::files_by_id::FilesByIdFilesByPk>,
}

#[derive(Debug, Properties, Clone, PartialEq)]
pub struct FilesCardProps {
    pub on_app_route: Callback<AppRoute>,
    pub user_profile: Option<MyUserProfile>,
    pub auth_school: Option<school_model::school_by_id::SchoolByIdSchoolByPk>,
    pub group_id: GroupId,
    pub files: ClassGroupFiles,
    pub on_files_delete: Option<Callback<FilesId>>,
}

#[derive(Debug)]
pub enum FilesCardMessage {
    FetchFilesById(FilesId),
    Files(Option<files_model::files_by_id::ResponseData>),
    DeleteFiles(FilesId),
}

impl Component for FilesCard {
    type Message = FilesCardMessage;
    type Properties = FilesCardProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        link.send_message(FilesCardMessage::FetchFilesById(props.files.files_id));
        FilesCard {
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
            FilesCardMessage::FetchFilesById(files_id) => {
                if let Some(graphql_task) = self.graphql_task.as_mut() {
                    let vars = files_model::files_by_id::Variables {
                        files_id: files_id.0,
                    };

                    let task = files_model::FilesById::request(
                            graphql_task,
                            &self.link,
                            vars,
                            |response| {
                                FilesCardMessage::Files(response)
                            },
                    );
                    self.files_id_task = Some(task);
                }
            }
            FilesCardMessage::Files(files) => {
                self.files = files.clone().and_then(|data| data.files_by_pk)
            }
            FilesCardMessage::DeleteFiles(files_id) => {
                if let Some(on_files_delete) = &self.props.on_files_delete {
                    on_files_delete.emit(files_id);
                    self.link.send_message(FilesCardMessage::FetchFilesById(files_id));
                }
            }
        }
        should_update
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        info!("{:?} => {:?}", self.props, props);
        let mut should_render = false;

        if self.props.files.files_id != props.files.files_id {
            self.link.send_message(FilesCardMessage::FetchFilesById(self.props.files.files_id));
        }

        if self.props != props {
            self.props = props;
            should_render = true;
        } 

        should_render
    }

    fn view(&self) -> Html {
        let files_id = self.props.files.files_id;
        let on_delete_file = self.link.callback(move |_| FilesCardMessage::DeleteFiles(files_id));
        let files_profile = self
            .files
            .as_ref()
            .and_then(|files| files.files_profile.as_ref())
            .and_then(|files_profile| {
                Some(html! {
                    <div class="card-activity-view bg-white d-flex align-items-center justify-content-between mb-4 p-4">
                        <span class="text-primary-blue-dark noir-bold is-size-18 lh-22">
                            <i class="fas fa-paperclip"></i>
                            <span class="ps-2">{&files_profile.title}</span>
                        </span>
                        <div class="pe-4 d-flex align-items-center flex-wrap">
                            <span class="text-gray-purple-two noir-light is-size-14 lh-17-2 pe-6">{&files_profile.timestamp.format("%a %b-%e-%Y").to_string()}</span>
                            <button class="btn btn-transparent">
                                <span class="text-gray-blue">
                                    <i class="far fa-save fas fa-lg"></i>
                                </span>
                            </button>
                            <button class="btn btn-outline-danger" onclick=on_delete_file>
                                <span class="is-size-14">
                                    <i class="far fa-trash-alt"></i>
                                </span>
                            </button>
                        </div>
                    </div>
                })
            }).unwrap_or(html! {});
        html! {
            <>
                {files_profile}
            </>
        }
    }
}
