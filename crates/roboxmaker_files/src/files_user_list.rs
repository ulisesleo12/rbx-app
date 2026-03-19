use log::*;
use std::vec;
use uuid::Uuid;
use yew::prelude::*;
use code_location::code_location;
use crate::files_user_card::FilesUserCard;
use yew::{html, Component, ComponentLink, Html, ShouldRender};

use roboxmaker_models::{user_model, school_model, files_model};
use roboxmaker_graphql::{GraphQLService, GraphQLTask, Request, RequestTask};
use roboxmaker_types::types::{GroupId, FilesId, ClassesId, AppRoute, ClassGroupFiles};

pub struct FilesUserList {
    link: ComponentLink<Self>,
    props: FilesUserListProperties,
    graphql_task: Option<GraphQLTask>,
    files_add_task: Option<RequestTask>,
}

#[derive(Debug, Properties, Clone, PartialEq)]
pub struct FilesUserListProperties {
    pub files: Vec<ClassGroupFiles>,
    pub user_profile: Option<MyUserProfile>,
    pub auth_school: Option<school_model::school_by_id::SchoolByIdSchoolByPk>,
    pub on_app_route: Callback<AppRoute>,
    pub group_id: GroupId,
    pub classes_id: Option<ClassesId>,
    pub on_list_change: Option<Callback<()>>,
    pub inventory_group_id: Uuid,
}

#[derive(Debug)]
pub enum FilesUserListMessage {
    AddFiles(FilesId),
    RemoveFiles(FilesId),
    CreateFiles,
    FilesAdded(Option<FilesId>),
    FilesRemoved(Option<FilesId>),
}

impl Component for FilesUserList {
    type Message = FilesUserListMessage;
    type Properties = FilesUserListProperties;

    fn create(ctx: &Context<Self>) -> Self {
        FilesUserList {
            link,
            props,
            graphql_task: Some(GraphQLService::connect(&code_location!())),
            files_add_task: None,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        info!("{:?}", msg);
        let mut should_update = true;
        match msg {
            FilesUserListMessage::AddFiles(files_id) => {
                if let Some(graphql_task) = self.graphql_task.as_mut() {
                    if let Some(classes_id) = ctx.props().classes_id {
                        let vars = files_model::files_group_add::Variables {
                            group_id: ctx.props().group_id.0,
                            classes_id: classes_id.0,
                            files_id: files_id.0,
                        };
                        let task = files_model::FilesGroupAdd::request(
                                graphql_task,
                                &self.link,
                                vars,
                                |response| {
                                    let files_id = if let Some(files) = response {
                                        files.insert_files_group_one.clone().and_then(|data| Some(FilesId(data.files_id)))
                                    } else {
                                        None
                                    };
                                    FilesUserListMessage::FilesAdded(files_id)
                                },
                        );
                        self.files_add_task = Some(task);
                    }

                }
            }
            FilesUserListMessage::RemoveFiles(files_id) => {
                if let Some(graphql_task) = self.graphql_task.as_mut() {
                    if let Some(classes_id) = ctx.props().classes_id {
                        let vars = files_model::files_group_delete::Variables {
                            group_id: ctx.props().group_id.0,
                            classes_id: classes_id.0,
                            files_id: files_id.0,
                        };
                        let task = files_model::FilesGroupDelete::request(
                                graphql_task,
                                &self.link,
                                vars,
                                |response| {
                                    let files_id = if let Some(response) = response {
                                        if response.delete_files_group.clone().and_then(|data| Some(data.returning)).unwrap_or(vec![]).len() > 0 {
                                            Some(FilesId(response.delete_files_group.clone().and_then(|data| Some(data.returning)).unwrap_or(vec![])[0].files_id))
                                        } else {
                                            None
                                        }
                                    } else {
                                        None
                                    };
                                    FilesUserListMessage::FilesAdded(files_id)
                                },
                        );
                        self.files_add_task = Some(task);
                    }

                }
            }
            FilesUserListMessage::CreateFiles => {
                let group_id = ctx.props().group_id;
                let local = chrono::Local::now().naive_local();
                
                if let Some(graphql_task) = self.graphql_task.as_mut() {
                    if let Some(classes_id) = ctx.props().classes_id {
                        let vars = files_model::files_group_create::Variables {
                            title: String::from("File"),
                            classes_id: classes_id.0,
                            group_id: group_id.0,
                            inventory_group_id: ctx.props().inventory_group_id,
                            files_id: Uuid::new_v4(),
                            timestamp: local,
                        };
                        let task = files_model::FilesGroupCreate::request(
                                graphql_task,
                                &self.link,
                                vars,
                                |response| {
                                    let files_id = if let Some(files) = response {
                                        files.insert_files_group_one.clone().and_then(|data| Some(FilesId(data.files_id)))
                                    } else {
                                        None
                                    };
                                    FilesUserListMessage::FilesAdded(files_id)
                                },
                        );
                        self.files_add_task = Some(task);
                    }

                }
            }
            FilesUserListMessage::FilesAdded(files_id) => {
                if let Some(files_id) = files_id {
                    ctx.props().files.push(ClassGroupFiles { files_id });
                    if let Some(on_list_change) = &ctx.props().on_list_change {
                        on_list_change.emit(());
                    }
                } else {
                    should_update = true;
                }
            }
            FilesUserListMessage::FilesRemoved(files_id) => {
                if let Some(files_id) = files_id {
                    ctx.props().files.retain(|u| u.files_id != files_id);
                    if let Some(on_list_change) = &ctx.props().on_list_change {
                        on_list_change.emit(());
                    }
                } else {
                    should_update = true;
                }
            }
        }
        should_update
    }

    fn changed(&mut self, ctx: &Context<Self>, old_props: &Self::Properties) -> bool {
        info!("{:?} => {:?}", ctx.props(), old_props);
        let mut should_render = false;

        if self.props != props {
            self.props = props;
            should_render = true;
        } 
        
        should_render
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let auth_user = ctx.props().auth_user.clone();
        let maybe_files = |files_id: &ClassGroupFiles| {
            let on_files_delete = {
                let callback = ctx.link().callback(|files_id| FilesUserListMessage::RemoveFiles(files_id));
                Some(callback)
            };
            html! {
                <FilesUserCard auth_user=auth_user.clone() 
                    files=files_id.clone()
                    on_app_route=ctx.props().on_app_route.clone()
                    on_files_delete=on_files_delete
                    on_list_change=ctx.props().on_list_change.clone()
                    group_id=ctx.props().group_id />
            }
        };

        let maybe_files = if ctx.props().files.len() > 0 {
            html! {
                <>
                    // { maybe_files_add }
                    // <div class="py-5 mt-2"><span class="text-primary-blue-dark noir-bold is-size-14 lh-18">{lang::dict("File List")}</span></div>
                    {                        
                        ctx.props().files
                        .iter()
                        .map(|files_id| {
                        maybe_files(files_id)
                        }).collect::<Html>()
                    }
                </>
            }
        } else {
            html! {
                <div class="text-center">
                    // { maybe_files_add }
                    <div>
                        <p class="is-size-7-mobile is-size-5-tablet is-size-4-desktop">{"No hay Archivos"}</p>
                    </div>
                </div>
            }
        };
        html! { maybe_files }
    }
}