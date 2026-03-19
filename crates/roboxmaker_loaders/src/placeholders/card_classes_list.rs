use yew::prelude::*;

pub struct Msg;

pub struct CardClassesListPlaceholder;

impl Component for CardClassesListPlaceholder {
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