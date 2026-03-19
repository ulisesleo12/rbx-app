use yew::prelude::*;

pub struct Msg;

pub struct CardClassesListPlaceholder;

impl Component for CardClassesListPlaceholder {
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
            <div class="card-classes-list is-loading-classes-list p-4 mb-5">
                <div class="title-classes-list"></div>
                <div class="options-classes-list">
                    <h2></h2>
                    <p></p>
                    <div class="menu-classes-list"></div>
                </div>
            </div>
        }
    }
}