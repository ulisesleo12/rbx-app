use log::*;
use std::vec;
use uuid::Uuid;
use yew::prelude::*;
use code_location::code_location;
use crate::files_user_list::FilesUserList;
use yew::{html, Component, ComponentLink, Html, ShouldRender};

use roboxmaker_models::{user_model, school_model, files_model};
use roboxmaker_types::types::{GroupId, FilesId, AppRoute, SchoolId, ClassGroupFiles};
use roboxmaker_graphql::{GraphQLService, GraphQLTask, Request, RequestTask};

pub struct UserFiles {
    link: ComponentLink<Self>,
    props: UserFilesProperties,
    graphql_task: Option<GraphQLTask>,
    inventory_id_task: Option<RequestTask>,
    files_task: Option<RequestTask>,
    files_list: Vec<ClassGroupFiles>,
    inventory: Vec<school_model::inventory_group_id_by_school_id::InventoryGroupIdBySchoolIdSchoolGroup>,
}

#[derive(Debug, Properties, Clone, PartialEq)]
pub struct UserFilesProperties {
    pub on_app_route: Callback<AppRoute>,
    pub user_profile: Option<MyUserProfile>,,
    pub auth_school: Option<school_model::school_by_id::SchoolByIdSchoolByPk>,
    pub group_id: GroupId,
    pub on_list_change: Callback<()>,
    pub school_id: Option<SchoolId>,
}

#[derive(Debug)]
pub enum UserFilesMessage {
    FetchFilesByAuthorId,
    FilesList(Option<files_model::files_by_author_id::ResponseData>),
    FetchInventoryGroupId,
    Inventory(Option<school_model::inventory_group_id_by_school_id::ResponseData>),
}

impl Component for UserFiles {
    type Message = UserFilesMessage;
    type Properties = UserFilesProperties;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        link.send_message(UserFilesMessage::FetchFilesByAuthorId);
        link.send_message(UserFilesMessage::FetchInventoryGroupId);
        UserFiles {
            link,
            props,
            graphql_task: Some(GraphQLService::connect(&code_location!())),
            inventory_id_task: None,
            files_task: None,
            files_list: vec![],
            inventory: vec![],
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        info!("{:?}", msg);
        let should_update = true;
        match msg {
            UserFilesMessage::FetchFilesByAuthorId => {
                let user_id = self.props.auth_user.as_ref()
                    .and_then(|data| data.user_by_pk.as_ref())
                    .and_then(|data| Some(data.id));

                if let Some(graphql_task) = self.graphql_task.as_mut() {
                    if user_id.is_some() {
                        let vars = files_model::files_by_author_id::Variables {
                            user_id: user_id.clone().unwrap(),
                        };
    
                        let task = files_model::FilesByAuthorId::request(
                                graphql_task,
                                &self.link,
                                vars,
                                |response| {
                                    UserFilesMessage::FilesList(response)
                                },
                        );
                        self.files_task = Some(task);
                    }
                }
            }
            UserFilesMessage::FilesList(files_list) => {
                self.files_list = files_list
                    .clone()
                    .and_then(|data| Some(data.files))
                    .unwrap_or(vec![])
                    .iter()
                    .map(|files| {
                        ClassGroupFiles {
                            files_id: FilesId(files.id),
                        }
                    }).collect();
            }
            UserFilesMessage::FetchInventoryGroupId => {
                if let Some(graphql_task) = self.graphql_task.as_mut() {
                    if self.props.school_id.is_some() {
                        let vars = school_model::inventory_group_id_by_school_id::Variables {
                            school_id: self.props.school_id.unwrap().0, 
                        };
    
                        let task = school_model::InventoryGroupIdBySchoolId::request(
                                graphql_task,
                                &self.link,
                                vars,
                                |response| {
                                    UserFilesMessage::Inventory(response)
                                },
                        );
                        self.inventory_id_task = Some(task);
                    }
                }
            }
            UserFilesMessage::Inventory(inventory) => {
                self.inventory = inventory.clone().and_then(|data| Some(data.school_group)).unwrap_or(vec![])
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
        let on_list_change = self.link.callback(|_| UserFilesMessage::FetchFilesByAuthorId);
        let files = self.files_list.iter().cloned().collect::<Vec<ClassGroupFiles>>();
        let files_list = self.inventory.iter().map(|inventory_group| {
            let inventory_group_id = inventory_group.inventory_group.clone().and_then(|data| Some(data.group_id)).unwrap_or(Uuid::default());
            html! {
                <FilesUserList files=files.clone()
                on_app_route=self.props.on_app_route.clone()
                auth_user=self.props.auth_user.clone()
                auth_school=self.props.auth_school.clone()
                group_id=self.props.group_id.clone()
                on_list_change=Some(on_list_change.clone())
                inventory_group_id=inventory_group_id />
            }
        }).collect::<Html>();
        html! {
            <>
                {files_list}
            </>
        }
    }
}
