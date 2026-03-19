use log::*;
use std::vec;
use yew::prelude::*;
use code_location::code_location;
use yew::{html, Component, ComponentLink, Html, ShouldRender};

use roboxmaker_main::lang;
use roboxmaker_types::types::{GroupId, FilesId, AppRoute};
use roboxmaker_models::{user_model, school_model, files_model};
use roboxmaker_graphql::{GraphQLService, GraphQLTask, Request, RequestTask};


#[derive(Debug)]
pub enum FilesSelectOption {
    Files(FilesId),
    NewFiles,
}
pub struct FilesSelect {
    link: ComponentLink<Self>,
    props: FilesSelectProperties,
    graphql_task: Option<GraphQLTask>,
    files_task: Option<RequestTask>,
    show_modal_add_file: bool,
    files_list: Vec<files_model::files_by_author_id::FilesByAuthorIdFiles>,
}

#[derive(Debug, Properties, Clone, PartialEq)]
pub struct FilesSelectProperties {
    pub on_select: Callback<FilesSelectOption>,
    pub group_id: Option<GroupId>,
    pub on_app_route: Callback<AppRoute>,
    pub auth_school: Option<school_model::school_by_id::SchoolByIdSchoolByPk>,
    pub user_profile: Option<MyUserProfile>,
}

#[derive(Debug)]
pub enum FilesSelectMessage {
    SelectFiles(FilesSelectOption),
    FetchFilesByAuthorId,
    FilesList(Option<files_model::files_by_author_id::ResponseData>),
    ShowModal,
}

impl Component for FilesSelect {
    type Message = FilesSelectMessage;
    type Properties = FilesSelectProperties;

    fn create(ctx: &Context<Self>) -> Self {
        link().send_message(FilesSelectMessage::FetchFilesByAuthorId);
        FilesSelect {
            link,
            props,
            graphql_task: Some(GraphQLService::connect(&code_location!())),
            files_task: None,
            show_modal_add_file: false,
            files_list: vec![],
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        info!("{:?}", msg);
        let should_render = true;
        match msg {
            FilesSelectMessage::SelectFiles(select_option) => {
                self.show_modal_add_file = false;
                self.files_list = vec![];
                ctx.props().on_select.emit(select_option);
            }
            FilesSelectMessage::FetchFilesByAuthorId => {
                let user_id = ctx.props().auth_user.as_ref()
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
                                    FilesSelectMessage::FilesList(response)
                                },
                        );
                        self.files_task = Some(task);
                    }
                }
            }
            FilesSelectMessage::FilesList(files_list) => {
                self.files_list = files_list.clone().and_then(|data| Some(data.files)).unwrap_or(vec![])
            }
            FilesSelectMessage::ShowModal => {
                self.show_modal_add_file = !self.show_modal_add_file;
            }
        }
        should_render
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
        let maybe_new = self
            .props.auth_user
            .as_ref()
            .and_then(|data| data.user_by_pk.as_ref())
            .and_then(|user|{
                if user.user_staff.is_some() || user.user_teacher.is_some() || user.user_student.is_some() {
                    let on_select = ctx.link().callback(move |_| FilesSelectMessage::SelectFiles(FilesSelectOption::NewFiles));
                    Some(html! {
                        <a class="btn button-update-file d-flex align-items-center justify-content-center me-5" onmousedown=on_select>
                            <span class="text-white noir-bold is-size-16 lh-20 text-center">{lang::dict("Upload File...")}</span>
                        </a>
                    })
                } else {Some(html! {})}
            })
            .unwrap_or(html! {});
        let files = self
            .files_list
            .iter()
            .map(|files| {
                let files_id = FilesId(files.id);
                let on_select = ctx.link().callback(move |_| {FilesSelectMessage::SelectFiles(FilesSelectOption::Files(files_id))});
                let title = files.files_profile.clone().unwrap().title;
                html! {
                    <div class="m-4">
                        <div class="card vh-15">
                            <div class="module-message-universal line-clamp-message-universal p-2 h-80">
                                <span class="text-primary-blue-dark noir-bold is-size-18 lh-22">
                                    {&title}
                                </span>
                            </div>
                            <div class="card-body border-top d-flex px-5 py-2">
                                <a class="btn btn-outline-secondary btn-sm mx-auto" onmousedown=on_select>
                                    <span>
                                        {lang::dict("Add")}
                                    </span>
                                </a>
                            </div>
                        </div>
                    </div>
                }
            })
            .collect::<Html>();
        let show_modal = ctx.link().callback(|_| FilesSelectMessage::ShowModal);
        let class_search_modal = if self.show_modal_add_file {
            "modal fade show"
        } else {
            "modal fade"
        };
        let class_search_scroll = if self.show_modal_add_file {
            "display: block;"
        } else {
            "display: none;"
        };
        let modal_files = html! {
            <div class=class_search_modal id="exampleModalScrollable" tabindex="-1" aria-labelledby="exampleModalScrollableTitle" style=class_search_scroll aria-modal="true" role="dialog">
                <div class="modal-dialog modal-dialog-scrollable modal-xl">
                    <div class="modal-content">
                        <div class="modal-header">
                            <p class="modal-card-title text-center">{"Mis Archivos"}</p>
                            <a class="btn bg-purple-on ms-5" onclick=&show_modal>
                                <span class="text-white">
                                    <i class="fas fa-times"></i>
                                </span>
                            </a>
                        </div>
                        <div class="modal-body vh-100 d-flex flex-wrap">
                            {files}
                        </div>
                    </div>
                </div>
            </div>
        };
        html! {
            <>
                <div class="d-flex flex-wrap">
                    {maybe_new}
                    <a class="btn button-search-my-space d-flex align-items-center justify-content-center" onclick=&show_modal>
                        <span class="text-secondary-purple noir-bold is-size-16 lh-20">{lang::dict("Search in My Space...")}</span>
                    </a>
                </div>
                {modal_files}
            </>
        }
    }
}
