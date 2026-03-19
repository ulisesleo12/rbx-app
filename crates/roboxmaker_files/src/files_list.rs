use log::*;
use std::vec;
use uuid::Uuid;
use yew::prelude::*;
use code_location::code_location;
use crate::files_card::FilesCard;
use crate::files_select::FilesSelect;
use crate::files_select::FilesSelectOption;
use yew::{html, Component, ComponentLink, Html, ShouldRender};

use roboxmaker_main::lang;
use roboxmaker_models::{user_model, school_model, files_model};
use roboxmaker_graphql::{GraphQLService, GraphQLTask, Request, RequestTask};
use roboxmaker_types::types::{GroupId, FilesId, ClassesId, SchoolId, AppRoute, ClassGroupFiles};

pub struct FilesList {
    link: ComponentLink<Self>,
    props: FilesListProperties,
    graphql_task: Option<GraphQLTask>,
    inventory_id_task: Option<RequestTask>,
    files_add_task: Option<RequestTask>,
    inventory: Vec<school_model::inventory_group_id_by_school_id::InventoryGroupIdBySchoolIdSchoolGroup>,
}

#[derive(Debug, Properties, Clone, PartialEq)]
pub struct FilesListProperties {
    pub files: Vec<ClassGroupFiles>,
    pub user_profile: Option<MyUserProfile>,
    pub auth_school: Option<school_model::school_by_id::SchoolByIdSchoolByPk>,
    pub on_app_route: Callback<AppRoute>,
    pub classes_id: ClassesId,
    pub group_id: GroupId,
    pub school_id: SchoolId,
}

#[derive(Debug)]
pub enum FilesListMessage {
    FetchInventoryGroupId,
    Inventory(Option<school_model::inventory_group_id_by_school_id::ResponseData>),
    AddFiles(FilesId),
    RemoveFiles(FilesId),
    CreateFiles,
    FilesAdded(Option<FilesId>),
    FilesRemoved(Option<FilesId>),
}

impl Component for FilesList {
    type Message = FilesListMessage;
    type Properties = FilesListProperties;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        link.send_message(FilesListMessage::FetchInventoryGroupId);
        FilesList {
            link,
            props,
            graphql_task: Some(GraphQLService::connect(&code_location!())),
            inventory_id_task: None,
            files_add_task: None,
            inventory: vec![],
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        info!("{:?}", msg);
        let mut should_update = true;
        match msg {
            FilesListMessage::FetchInventoryGroupId => {
                if let Some(graphql_task) = self.graphql_task.as_mut() {
                    let vars = school_model::inventory_group_id_by_school_id::Variables {
                        school_id: self.props.school_id.0, 
                    };

                    let task = school_model::InventoryGroupIdBySchoolId::request(
                            graphql_task,
                            &self.link,
                            vars,
                            |response| {
                                FilesListMessage::Inventory(response)
                            },
                    );
                    self.inventory_id_task = Some(task);
                }
            }
            FilesListMessage::Inventory(inventory) => {
                self.inventory = inventory.clone().and_then(|data| Some(data.school_group)).unwrap_or(vec![])
            }
            FilesListMessage::AddFiles(files_id) => {
                if let Some(graphql_task) = self.graphql_task.as_mut() {
                    let vars = files_model::files_group_add::Variables {
                        group_id: self.props.group_id.0,
                        classes_id: self.props.classes_id.0,
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
                                FilesListMessage::FilesAdded(files_id)
                            },
                    );
                    self.files_add_task = Some(task);
                }
            }
            FilesListMessage::RemoveFiles(files_id) => {
                if let Some(graphql_task) = self.graphql_task.as_mut() {
                    let vars = files_model::files_group_delete::Variables {
                        group_id: self.props.group_id.0,
                        classes_id: self.props.classes_id.0,
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
                                FilesListMessage::FilesRemoved(files_id)
                            },
                    );
                    self.files_add_task = Some(task);
                }
            }
            FilesListMessage::CreateFiles => {
                let inventory_group_id = self.inventory.iter()
                    .nth(0)
                    .map(|data| data.inventory_group.clone().and_then(|data| Some(data.group_id))).unwrap_or(Some(Uuid::default()));
                let classes_id = self.props.classes_id;
                let group_id = self.props.group_id;
                let local = chrono::Local::now().naive_local();

                if let Some(graphql_task) = self.graphql_task.as_mut() {
                    if let Some(inventory_group_id) = inventory_group_id {
                        let vars = files_model::files_group_create::Variables {
                            title: String::from("File"),
                            classes_id: classes_id.0,
                            group_id: group_id.0,
                            inventory_group_id,
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
                                    FilesListMessage::FilesAdded(files_id)
                                },
                        );
                        self.files_add_task = Some(task);
                    }
                }
            }
            FilesListMessage::FilesAdded(files_id) => {
                if let Some(files_id) = files_id {
                    self.props.files.push(ClassGroupFiles { files_id });
                } else {
                    should_update = true;
                }
            }
            FilesListMessage::FilesRemoved(files_id) => {
                if let Some(files_id) = files_id {
                    self.props.files.retain(|u| u.files_id != files_id);
                } else {
                    should_update = true;
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
        let auth_user = self.props.auth_user.clone();
        let maybe_files = |files_id: &ClassGroupFiles| {
            let on_files_delete = {
                let callback = self.link.callback(|files_id| FilesListMessage::RemoveFiles(files_id));
                Some(callback)
            };
            html! {
                <FilesCard auth_user=auth_user.clone() 
                    files=files_id.clone()
                    on_app_route=self.props.on_app_route.clone()
                    on_files_delete=on_files_delete
                    group_id=self.props.group_id />
            }
        };

        let maybe_files_add = self
            .props
            .auth_user
            .as_ref()
            .and_then(|data| data.user_by_pk.as_ref())
            .and_then(|auth_user| {
                let on_select = self.link.callback(|select_option| match select_option {
                    FilesSelectOption::Files(files_id) => FilesListMessage::AddFiles(files_id),
                    FilesSelectOption::NewFiles => FilesListMessage::CreateFiles,
                });
                if auth_user.user_staff.is_some() || auth_user.user_teacher.is_some() || auth_user.user_student.is_some() {
                    Some(html! {
                        <FilesSelect on_select=on_select 
                        group_id=Some(self.props.group_id)
                        auth_user=self.props.auth_user.clone()
                        on_app_route=self.props.on_app_route.clone() />
                    })
                } else {
                    Some(html! {})
                }
            })
            .unwrap_or(html! {});

        let maybe_files = if self.props.files.len() > 0 {
            html! {
                <>
                    { maybe_files_add }
                    <div class="py-5 mt-2"><span class="text-primary-blue-dark noir-bold is-size-14 lh-18">{lang::dict("File List")}</span></div>
                    {                        
                        self.props.files
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
                    { maybe_files_add }
                    <div>
                        <p class="is-size-7-mobile is-size-5-tablet is-size-4-desktop">{"No hay Archivos"}</p>
                    </div>
                </div>
            }
        };
        html! { maybe_files }
    }
}