use uuid::Uuid;
use serde_derive::{Serialize, Deserialize};


use roboxmaker_utils::functions::get_creation_date;
use roboxmaker_models::grade_model::get_class_group_by_group_id::{self, GetClassGroupByGroupIdClassProfile};
use roboxmaker_types::types::{PostProfile, ClassesProfile, RobotProfile, LessonProfile, MeetingsProfile, PostId, ClassesId, LessonId, RobotId, MeetingsId, MemberProfile, UserId};


#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ClassProfile {
    pub name: String,
    pub class_id: Uuid,
    pub section_id: Uuid,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct ClassProfileData {
    pub class_profile: Option<ClassProfile>,
    pub post_profile: Vec<PostProfile>,
    pub classes_profile: Vec<ClassesProfile>,
    pub lesson_profile: Vec<LessonProfile>,
    pub robot_profile: Vec<RobotProfile>,
    pub meets_profile: Vec<MeetingsProfile>,
    pub members_profile: Vec<MemberProfile>,
}


impl ClassProfileData {
    pub fn get_class_profile(class_data: GetClassGroupByGroupIdClassProfile) -> ClassProfileData {

        let naive = chrono::NaiveTime::from_hms_opt(23, 59, 59).unwrap();


        let  class_profile = Some( ClassProfile { 
            name: class_data.name.clone(), 
            class_id: class_data.class_id, 
            section_id: class_data.section_id 
        });

        let post_profile = class_data.class_post.iter().map(|item | {

            let time_fn = get_creation_date(item.post_profile.timestamp);
            let maybe_time_fn = get_creation_date(item.post_profile.timestamp);

            PostProfile {
                topic: item.post_profile.topic.clone(),
                timestamp: time_fn,
                maybe_timestamp: maybe_time_fn,
                post_id: PostId(item.post_profile.post_id),
                full_name: item.post_profile.author_profile.clone().and_then(|item| Some(item.full_name)).unwrap_or(String::default()),
                pic_path: item.post_profile.author_profile.clone().and_then(|item| Some(item.full_name)).unwrap_or(String::default()),
            }
        }).collect();

        let classes_profile = class_data.class_classes.iter().map(|item | {
            let time_fn = get_creation_date(item.classes_profile.timestamp);

            ClassesProfile {
                topic: item.classes_profile.topic.clone(), 
                timestamp: time_fn, 
                classes_id: ClassesId(item.classes_profile.classes_id) 
            }
        }).collect();

        let lesson_profile = class_data.class_lesson.iter().map(|item | {

            let time_fn = get_creation_date(item.lesson_profile.timestamp);
            let lesson_type = format!("{:?}", item.lesson_profile.lesson_type.clone().unwrap_or(get_class_group_by_group_id::RoboxLessonTypeEnum::Extra));

            LessonProfile { 
                title: item.lesson_profile.title.clone(), 
                timestamp: time_fn, 
                lesson_id: LessonId(item.lesson_profile.lesson_id), 
                full_name: item.lesson_profile.author_profile.clone().and_then(|author| Some(author.full_name)).unwrap_or(String::default()), 
                author_id: item.lesson_profile.author_profile.clone().and_then(|author| Some(author.user_id)).unwrap_or(Uuid::default()),  
                pic_path: item.lesson_profile.author_profile.clone().and_then(|author| author.pic_path).unwrap_or(String::default()), 
                lesson_type: lesson_type.to_uppercase(),
            }
        }).collect();

        let robot_profile = class_data.class_robot.iter().map(|item | {

            let time_fn = get_creation_date(item.robot_profile.timestamp);

            RobotProfile { 
                name: item.robot_profile.name.clone(), 
                timestamp: time_fn, 
                path: item.robot_profile.path.clone(), 
                robot_id: RobotId(item.robot_profile.robot_id) 
            }
        }).collect();

        let meets_profile = class_data.class_meet.iter().map(|item | {

            MeetingsProfile { 
                title: item.meetings_profile.title.clone(), 
                schedule_time: item.meetings_profile.schedule_time.format("%d-%m-%Y").to_string(), 
                start_of_meeting: item.meetings_profile.start_of_meeting.unwrap_or(naive).to_string(), 
                end_of_meeting: item.meetings_profile.end_of_meeting.unwrap_or(naive).to_string(), 
                meeting_id: MeetingsId(item.meetings_profile.meet_id), 
                author_name: item.meetings_profile.author_profile.clone().and_then(|author| Some(author.full_name)).unwrap_or(String::default()), 
                user_staff: item.meetings_profile.author_profile.clone().and_then(|user| user.user_staff).is_some(), 
                user_teacher: item.meetings_profile.author_profile.clone().and_then(|user| user.user_teacher).is_some() 
            }
        }).collect();

        let members_profile = class_data.class_users.iter().map(|item | {

            MemberProfile { 
                full_name: item.user_profile.full_name.clone(), 
                pic_path: item.user_profile.pic_path.clone().unwrap_or(String::new()), 
                user_id: UserId(item.user_profile.user_id), 
                user_staff: item.user_profile.user_staff.is_some(), 
                user_teacher: item.user_profile.user_teacher.is_some(), 
                user_student: item.user_profile.user_student.is_some() 
            }
        }).collect();
        

        ClassProfileData {
            class_profile,
            post_profile,
            classes_profile,
            lesson_profile,
            robot_profile,
            meets_profile,
            members_profile,
        }
    }
}