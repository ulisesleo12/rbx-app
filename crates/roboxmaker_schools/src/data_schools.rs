use log::*;
use yew::prelude::*;
use code_location::code_location;
use yew::{html, Component, ComponentLink, Html, ShouldRender};

use roboxmaker_models::school_model;
use roboxmaker_types::types::{SchoolId, AppRoute, MyUserProfile};
use roboxmaker_graphql::{GraphQLService, GraphQLTask, Request, RequestTask};

pub struct DataSchools {
    link: ComponentLink<Self>,
    props: DataSchoolsProps,
    graphql_task: Option<GraphQLTask>,
    staff_task: Option<RequestTask>,
    student_task: Option<RequestTask>,
    teacher_task: Option<RequestTask>,
    deggrees_task: Option<RequestTask>,
    staff: Vec<school_model::list_staff_by_school_id::ListStaffBySchoolIdUser>,
    student: Vec<school_model::list_student_by_school_id::ListStudentBySchoolIdUser>,
    teacher: Vec<school_model::list_teacher_by_school_id::ListTeacherBySchoolIdUser>,
    list_of_deggrees: Vec<school_model::deggrees_by_school_by_id::DeggreesBySchoolByIdClassGroup>,
}

#[derive(Debug, Properties, Clone, PartialEq)]
pub struct DataSchoolsProps {
    pub on_app_route: Callback<AppRoute>,
    pub user_profile: Option<MyUserProfile>,
    pub school_id: SchoolId,
}

#[derive(Debug)]
pub enum DataSchoolsMessage {
    FetchMembersData,
    DataDeggrees(Option<school_model::deggrees_by_school_by_id::ResponseData>),
    FetchDataStaff,
    DataUserStaff(Option<school_model::list_staff_by_school_id::ResponseData>),
    FetchDataStudent,
    DataUserStudent(Option<school_model::list_student_by_school_id::ResponseData>),
    FetchDataTeacher,
    DataUserTeacher(Option<school_model::list_teacher_by_school_id::ResponseData>),
}

impl Component for DataSchools {
    type Message = DataSchoolsMessage;
    type Properties = DataSchoolsProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        link.send_message(DataSchoolsMessage::FetchMembersData);
        link.send_message(DataSchoolsMessage::FetchDataStaff);
        link.send_message(DataSchoolsMessage::FetchDataStudent);
        link.send_message(DataSchoolsMessage::FetchDataTeacher);
        DataSchools { 
            link, 
            props, 
            graphql_task: Some(GraphQLService::connect(&code_location!())),
            staff_task: None,
            student_task: None,
            teacher_task: None,
            deggrees_task: None,
            staff: vec![],
            student: vec![],
            teacher: vec![],
            list_of_deggrees: vec![],
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        info!("{:?}", msg);
        let should_update = true;
        match msg {
            DataSchoolsMessage::FetchMembersData => {
                let school_id = self.props.school_id;
                if let Some(graphql_task) = self.graphql_task.as_mut() {

                    let vars = school_model::deggrees_by_school_by_id::Variables {
                        school_id: school_id.0,
                    };

                    let task = school_model::DeggreesBySchoolById::request(
                            graphql_task,
                            &self.link,
                            vars,
                            |response| {
                                DataSchoolsMessage::DataDeggrees(response)
                            },
                    );
                    self.deggrees_task = Some(task);
                }
            }
            DataSchoolsMessage::DataDeggrees(list_of_deggrees) => {
                self.list_of_deggrees = list_of_deggrees.clone().and_then(|data| Some(data.class_group)).unwrap_or(vec![]);
            }
            DataSchoolsMessage::FetchDataStaff => {
                let school_id = self.props.school_id;
                if let Some(graphql_task) = self.graphql_task.as_mut() {

                    let vars = school_model::list_staff_by_school_id::Variables {
                        school_id: school_id.0,
                    };

                    let task = school_model::ListStaffBySchoolId::request(
                            graphql_task,
                            &self.link,
                            vars,
                            |response| {
                                DataSchoolsMessage::DataUserStaff(response)
                            },
                    );
                    self.staff_task = Some(task);
                }
            }
            DataSchoolsMessage::DataUserStaff(staff) => {
                self.staff = staff.clone().and_then(|data| Some(data.user)).unwrap_or(vec![]);
            }
            DataSchoolsMessage::FetchDataStudent => {
                let school_id = self.props.school_id;
                if let Some(graphql_task) = self.graphql_task.as_mut() {

                    let vars = school_model::list_student_by_school_id::Variables {
                        school_id: school_id.0,
                    };

                    let task = school_model::ListStudentBySchoolId::request(
                            graphql_task,
                            &self.link,
                            vars,
                            |response| {
                                DataSchoolsMessage::DataUserStudent(response)
                            },
                    );
                    self.student_task = Some(task);
                }
            }
            DataSchoolsMessage::DataUserStudent(student) => {
                self.student = student.clone().and_then(|data| Some(data.user)).unwrap_or(vec![]);
            }
            DataSchoolsMessage::FetchDataTeacher => {
                let school_id = self.props.school_id;
                if let Some(graphql_task) = self.graphql_task.as_mut() {

                    let vars = school_model::list_teacher_by_school_id::Variables {
                        school_id: school_id.0,
                    };

                    let task = school_model::ListTeacherBySchoolId::request(
                            graphql_task,
                            &self.link,
                            vars,
                            |response| {
                                DataSchoolsMessage::DataUserTeacher(response)
                            },
                    );
                    self.teacher_task = Some(task);
                }
            }
            DataSchoolsMessage::DataUserTeacher(teacher) => {
                self.teacher = teacher.clone().and_then(|data| Some(data.user)).unwrap_or(vec![]);
            }
        }
        should_update
    }
    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        trace!("{:?} => {:?}", self.props, props);
        let mut should_render = false;
        
        if self.props != props {
            self.props = props;
            should_render = true;
        } 

        should_render
    }
    fn view(&self) -> Html {
        let number_deggrees = self 
            .list_of_deggrees
            .iter()
            .map(|class_group|class_group).len();
        let number_student = self
            .student
            .iter()
            .map(|data| data).len();
        let number_teacher = self
            .teacher
            .iter()
            .map(|data| data).len();
        let number_staff = self
            .staff
            .iter()
            .map(|data| data).len();
        html! {
            <>
                <div class="px-5 d-flex justify-content-between pb-2">
                    <div class="d-flex flex-column">
                        <span class="d-flex align-items-center pb-2">
                            <img src="/icons/graduation-2.svg" style="height: 12px;" />
                            <span class="text-purple-gray noir-regular is-size-14 lh-18 ps-2">{number_deggrees}{" Grados"}</span>
                        </span>
                        <span class="d-flex align-items-center pb-2">
                            <img src="/icons/user-class.svg" style="height: 18px;" />
                            <span class="text-purple-gray noir-regular is-size-14 lh-18 ps-2">{number_student}{" Alumnos"}</span>
                        </span>
                    </div>
                    <div class="d-flex flex-column">
                        <span class="d-flex align-items-center pb-2">
                            <img src="/icons/user-2.svg" style="height: 18px;" />
                            <span class="text-purple-gray noir-regular is-size-14 lh-18 ps-2">{number_teacher}{" Profesores"}</span>
                        </span>
                        <span class="text-purple-gray d-flex align-items-center pb-2">
                            <i class="fas fa-user-cog me-1"></i>
                            <span class="text-purple-gray noir-regular is-size-14 lh-18 ps-2">{number_staff}{" Soportes"}</span>
                        </span>
                    </div>
                </div>
            </>
        }
    }
}
