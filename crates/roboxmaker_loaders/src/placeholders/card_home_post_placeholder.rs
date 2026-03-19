use yew::prelude::*;

pub struct Msg;

pub struct CardHomePostPlaceholder;

impl Component for CardHomePostPlaceholder {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        true
    }

    fn changed(&mut self, _ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
        // Should only return "true" if new properties are different to
        // previously received properties.
        // This component has no properties so we will always return "false".
        false
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div>
                <div class="skeleton" style=""></div>
                <div class="d-flex">
                    <div class="skeleton" style="line-height: 2rem"></div>
                </div>
            </div>
        }
    }
}