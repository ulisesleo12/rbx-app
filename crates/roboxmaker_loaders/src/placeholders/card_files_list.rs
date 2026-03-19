use yew::prelude::*;

pub struct Msg;

pub struct CardFilesListPlaceholder;

impl Component for CardFilesListPlaceholder {
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