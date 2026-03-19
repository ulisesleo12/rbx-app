use yew::prelude::*;

pub struct Msg;

pub struct CardLessonListPlaceholder;

impl Component for CardLessonListPlaceholder {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
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