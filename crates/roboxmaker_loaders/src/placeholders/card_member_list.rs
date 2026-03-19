use yew::prelude::*;

pub struct Msg;

pub struct CardMemberListPlaceholder;

impl Component for CardMemberListPlaceholder {
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