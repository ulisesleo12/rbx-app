use uuid::Uuid;
use yew_router::Switch;
use std::{fmt, str::FromStr};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, Default)]
pub struct SchoolId(pub Uuid);

impl FromStr for SchoolId {
    type Err = uuid::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(SchoolId(Uuid::from_str(s)?))
    }
}

impl fmt::Display for SchoolId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FilesId(pub Uuid);

impl FromStr for FilesId {
    type Err = uuid::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(FilesId(Uuid::parse_str(s)?))
    }
}

impl fmt::Display for FilesId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ActivityId(pub Uuid);

impl FromStr for ActivityId {
    type Err = uuid::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(ActivityId(Uuid::parse_str(s)?))
    }
}

impl fmt::Display for ActivityId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}



#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct GroupId(pub Uuid);

impl FromStr for GroupId {
    type Err = uuid::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(GroupId(Uuid::from_str(s)?))
    }
}

impl fmt::Display for GroupId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

pub fn gen_private_group_id(name: &str, uuids: Vec<&Uuid>) -> GroupId {
    let uuid_type = Uuid::parse_str(name).unwrap();
    let name_vec = uuids
        .iter()
        .map(|uuid| uuid.to_string())
        .collect::<Vec<String>>()
        .join("");
    GroupId(Uuid::new_v5(&uuid_type, name_vec.as_bytes()))
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ClassesId(pub Uuid);

impl FromStr for ClassesId {
    type Err = uuid::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(ClassesId(Uuid::from_str(s)?))
    }
}

impl fmt::Display for ClassesId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct UserId(pub Uuid);

impl FromStr for UserId {
    type Err = uuid::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(UserId(Uuid::from_str(s)?))
    }
}

impl fmt::Display for UserId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RobotId(pub Uuid);

impl FromStr for RobotId {
    type Err = uuid::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(RobotId(Uuid::from_str(s)?))
    }
}

impl fmt::Display for RobotId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LessonId(pub Uuid);

impl FromStr for LessonId {
    type Err = uuid::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(LessonId(Uuid::from_str(s)?))
    }
}

impl fmt::Display for LessonId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ResourceId(pub Uuid);

impl FromStr for ResourceId {
    type Err = uuid::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(ResourceId(Uuid::from_str(s)?))
    }
}

impl fmt::Display for ResourceId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PostId(pub Uuid);

impl FromStr for PostId {
    type Err = uuid::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(PostId(Uuid::from_str(s)?))
    }
}

impl fmt::Display for PostId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MeetId(pub Uuid);

impl FromStr for MeetId {
    type Err = uuid::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(MeetId(Uuid::parse_str(s)?))
    }
}

impl fmt::Display for MeetId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct WhiteboardId(pub Uuid);

impl FromStr for WhiteboardId {
    type Err = uuid::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(WhiteboardId(Uuid::parse_str(s)?))
    }
}

impl fmt::Display for WhiteboardId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MeetingsId(pub Uuid);

impl FromStr for MeetingsId {
    type Err = uuid::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(MeetingsId(Uuid::from_str(s)?))
    }
}

impl fmt::Display for MeetingsId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MessageId(pub Uuid);

impl FromStr for MessageId {
    type Err = uuid::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(MessageId(Uuid::parse_str(s)?))
    }
}

impl fmt::Display for MessageId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ClassGroupCategory {
    Posts,
    Members,
    Robots,
    Lessons,
    // Classes,
    TeacherResources,
    Quizzes,
}

impl FromStr for ClassGroupCategory {
    type Err = String;

    fn from_str(input: &str) -> Result<ClassGroupCategory, Self::Err> {
        match input {
            "posts" => Ok(ClassGroupCategory::Posts),
            "members" => Ok(ClassGroupCategory::Members),
            "robots" => Ok(ClassGroupCategory::Robots),
            "lessons" => Ok(ClassGroupCategory::Lessons),
            // "classes" => Ok(ClassGroupCategory::Classes),
            "resources" => Ok(ClassGroupCategory::TeacherResources),
            "quizzes" => Ok(ClassGroupCategory::Quizzes),
            _ => Err(String::from("Parsing ClassGroupCategory")),
        }
    }
}

impl fmt::Display for ClassGroupCategory {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let category = match self {
            ClassGroupCategory::Posts => "posts",
            ClassGroupCategory::Members => "members",
            ClassGroupCategory::Robots => "robots",
            ClassGroupCategory::Lessons => "lessons",
            // ClassGroupCategory::Classes => "classes",
            ClassGroupCategory::TeacherResources => "resources",
            ClassGroupCategory::Quizzes => "quizzes",
        };
        write!(f, "{}", category)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PageMode {
    View,
    Edit,
}

impl FromStr for PageMode {
    type Err = String;

    fn from_str(input: &str) -> Result<PageMode, Self::Err> {
        match input {
            "view" => Ok(PageMode::View),
            "edit" => Ok(PageMode::Edit),
            _ => Err(String::from("Parsing PageMode")),
        }
    }
}

impl fmt::Display for PageMode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let page_mode = match self {
            PageMode::View => "view",
            PageMode::Edit => "edit",
        };
        write!(f, "{}", page_mode)
    }
}


#[derive(Debug, Clone, PartialEq)]
pub struct ClassGroupPost {
    pub post_id: PostId,
    pub published: bool,
    pub archived: bool,
    pub class_name: String,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct ClassGroupFiles {
    pub files_id: FilesId,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ClassGroupMember {
    pub user_id: UserId,
    pub student: bool,
    pub teacher: bool,
    pub staff: bool,
    pub view_profile: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ClassGroupRobot {
    pub robot_id: RobotId,
    pub enabled: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ClassGroupLesson {
    pub lesson_id: LessonId,
    pub send_to_grade: bool,
    pub archived: bool,
    pub created_staff: bool,
    pub created_teacher: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ClassGroupClasses {
    pub classes_id: ClassesId,
    pub published: bool,
    pub archived: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ClassGroupMeetings {
    pub meetings_id: MeetingsId,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ClassGroup {
    pub class_name: String,
    pub group_id: GroupId,
    pub school_id: SchoolId,
    pub posts: Vec<ClassGroupPost>,
    pub members: Vec<ClassGroupMember>,
    pub robots: Vec<ClassGroupRobot>,
    pub lessons: Vec<ClassGroupLesson>,
    pub classes: Vec<ClassGroupClasses>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct NewClassGroup {
    pub class_name: String,
    pub group_id: GroupId,
    pub school_id: SchoolId,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DataSchool {
    pub name: String,
    pub inventory_id: Uuid,
    pub school_id: SchoolId,
    pub display_list_meetings: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ClassGroupTest {
    pub posts: Vec<ClassGroupPost>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ClassGroupMeet {
    pub class_name: String,
    pub group_id: GroupId,
    pub meetings: Vec<ClassGroupMeetings>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ClassGroupCategoryHome {
    Posts,
    Members,
    Robots,
    Classes,
    Meetings,
}

impl FromStr for ClassGroupCategoryHome {
    type Err = String;

    fn from_str(input: &str) -> Result<ClassGroupCategoryHome, Self::Err> {
        match input {
            "posts" => Ok(ClassGroupCategoryHome::Posts),
            "members" => Ok(ClassGroupCategoryHome::Members),
            "robots" => Ok(ClassGroupCategoryHome::Robots),
            "classes" => Ok(ClassGroupCategoryHome::Classes),
            "meetings" => Ok(ClassGroupCategoryHome::Meetings),
            _ => Err(String::from("Parsing ClassGroupCategoryHome")),
        }
    }
}

impl fmt::Display for ClassGroupCategoryHome {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let category = match self {
            ClassGroupCategoryHome::Posts => "posts",
            ClassGroupCategoryHome::Members => "members",
            ClassGroupCategoryHome::Robots => "robots",
            ClassGroupCategoryHome::Classes => "classes",
            ClassGroupCategoryHome::Meetings => "meetings",
        };
        write!(f, "{}", category)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ClassGroupHome {
    pub class_name: String,
    pub group_id: GroupId,
    pub school_id: SchoolId,
    pub posts: Vec<ClassGroupPost>,
    pub members: Vec<ClassGroupMember>,
    pub robots: Vec<ClassGroupRobot>,
    pub classes: Vec<ClassGroupClasses>,
    pub meetings: Vec<ClassGroupMeetings>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ClassGroups {
    pub class_name: String,
    pub group_id: GroupId,
}

#[derive(Debug, Clone, PartialEq)]
pub enum IconShowSidebarRight {
    ArrowRight,
    ArrowLeft,
}

#[derive(Debug, Clone)]
pub enum LoadSearchFound {
    Found,
    NotFound,
}

#[derive(Debug, Clone)]
pub enum LoadSearch {
    Static,
    Load(LoadSearchFound),
}

#[derive(Debug, Clone)]
pub enum LoadFullScreenFound {
    Found,
    NotFound,
}

#[derive(Debug, Clone)]
pub enum LoadFullScreen {
    Loading,
    Load(LoadFullScreenFound),
}

#[derive(Debug, Clone)]
pub enum LoadResponseFound {
    Found,
    NotFound,
}

#[derive(Debug, Clone)]
pub enum LoadResponse {
    Loading,
    Load(LoadResponseFound),
}

#[derive(Debug, Switch, Clone)]
pub enum AppRoute {
    #[to = "/school/{school_id}/group/{group_id}/{category}"]
    SchoolGroupSection(SchoolId, GroupId, ClassGroupCategory),
    #[to = "/degree/{school_id}/member/{user_id}/{category}"]
    GroupSectionStudent(SchoolId, UserId, ClassGroupCategory),
    #[to = "/user/{user_id}/"]
    MySpace(UserId),
    #[to = "/robot/{robot_id}/group/{group_id}/user/{user_id}"]
    Robot(RobotId, GroupId, UserId),
    #[to = "/lesson_edit/{SchoolId}/group/{group_id}/lesson/{lesson_id}"]
    Lesson(SchoolId, GroupId, LessonId),
    #[to = "/lesson_view/{SchoolId}/group/{group_id}/lesson/{lesson_id}"]
    LessonView(SchoolId, GroupId, LessonId),
    #[to = "/post/{SchoolId}/group/{group_id}/post/{post_id}/{mode}"]
    Post(SchoolId, GroupId, PostId, PageMode),
    // #[to = "/classes_view/{SchoolId}/group/{group_id}/classes/{classes_id}"]
    // Classes(SchoolId, GroupId, ClassesId),
    #[to = "/resource_view/{SchoolId}/group/{group_id}/resource/{resource_id}"]
    Resource(SchoolId, GroupId, ResourceId),
    #[to = "/quizzes_view/{SchoolId}/group/{group_id}/quiz/{quiz_id}"]
    Quizzes(SchoolId, GroupId, Uuid),
    #[to = "/group/{id}/meet/{meetings_id}"]
    Meet(GroupId, MeetingsId),
    #[to = "/group/meet_direct/{id}"]
    MeetDirect(GroupId),
    #[to = "/whiteboard/{id}"]
    Whiteboard(WhiteboardId),
    #[to = "/login"]
    Login,
    #[to = "/list/meetings/schools/view"]
    Meetings,
    #[to = "/school/view"]
    Schools,
    #[to = "/grades/user/school/{id}/view"]
    GradesByUserId(SchoolId),
    #[to = "/list/grades/school/{id}/view"]
    GradesBySchoolId(SchoolId),
    #[to = "/panel_add_users"]
    PanelAddUsers,
    #[to = "/quiz-panel"]
    QuizzesPanel,
    #[to = "/home"]
    Home,
    #[to = "/"]
    Landing,
}


#[derive(Debug, Clone, PartialEq)]
pub struct MyUserProfile {
    pub email: String,
    pub full_name: String,
    pub pic_path: String,
    pub user_id: UserId,
    pub school_name: String,
    pub user_student: Option<UserId>,
    pub user_teacher: Option<UserId>,
    pub user_staff: Option<UserId>,
    pub license: String,
    pub group_member_id: GroupId,
}


#[derive(Debug, Clone, PartialEq)]
pub struct PostPageContent {
    pub title: String,
    pub content: String,
    pub timestamp: String,
    pub author_user_id: Uuid,
    pub author_full_name: String,
    pub author_pic_path: String,
    pub published: bool,
    pub archived: bool,
    pub class_name: String,
}

// #[derive(Debug, Clone, PartialEq)]
// pub struct PostPageContent {
//     pub title: String,
//     pub content: String,
//     pub timestamp: String,
//     pub author_user_id: Uuid,
//     pub author_full_name: String,
//     pub author_pic_path: String,
//     pub published: bool,
//     pub archived: bool,
//     pub class_name: String,
// }

#[derive(Debug, Clone, PartialEq)]
pub enum QuizResponderPage {
    None,
    Edit,
}


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Groups {
    pub class_name: String,
    pub group_id: GroupId,
    pub school_id: Uuid,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Schools {
    pub name: String,
    pub inventory_group: Uuid,
    pub school_id: SchoolId,
}