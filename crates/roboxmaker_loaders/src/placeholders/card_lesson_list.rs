use yew::prelude::*;

pub struct Msg;

pub struct CardLessonListPlaceholder;

impl Component for CardLessonListPlaceholder {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        true
    }

    fn changed(&mut self, _ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
        false
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div class="card-lesson-list is-loading-lesson-list p-4 mb-md-3 mb-lg-5 me-md-3 me-lg-5">
                <div class="content-lesson-list-header">
                    <div class="image-lesson-list-2"></div>
                    <div class="menu-lesson-list"></div>
                </div>
                <div class="content-lesson-list">
                    <div class="image-lesson-list"></div>
                    <h2></h2>
                    <p></p>
                </div>
            </div>
        }
    }
}