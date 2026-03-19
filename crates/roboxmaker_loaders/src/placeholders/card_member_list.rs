use yew::prelude::*;

pub struct Msg;

pub struct CardMemberListPlaceholder;

impl Component for CardMemberListPlaceholder {
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
            <div class="card-member-list is-loading-member-list p-4 mb-3 mb-lg-5 me-2 me-lg-5">
                <div class="content-member-list-header">
                    <div class="content-member-list">
                        <div class="image-member-list"></div>
                        <div class="txt-member-list">
                            <h2></h2>
                            <p></p>
                        </div>
                    </div>
                    <div class="menu-member-list"></div>
                </div>
            </div>
        }
    }
}