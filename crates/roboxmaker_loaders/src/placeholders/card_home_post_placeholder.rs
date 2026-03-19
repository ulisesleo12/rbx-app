use yew::prelude::*;

pub struct Msg;

pub struct CardHomePostPlaceholder;

impl Component for CardHomePostPlaceholder {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        // Should only return "true" if new properties are different to
        // previously received properties.
        // This component has no properties so we will always return "false".
        false
    }

    fn view(&self) -> Html {
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