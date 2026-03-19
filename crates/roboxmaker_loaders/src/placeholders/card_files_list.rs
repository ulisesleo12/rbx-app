use yew::prelude::*;

pub struct Msg;

pub struct CardFilesListPlaceholder;

impl Component for CardFilesListPlaceholder {
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
            <div class="card-files-list is-loading-files-list p-4 mb-4 w-100">
                <div class="title-file">
                    <h3></h3>
                    <div class="title-files-list"></div>
                </div>
                <div class="options-files-list">
                    <h2></h2>
                    <p></p>
                </div>
            </div>
        }
    }
}