use yew::prelude::*;

pub struct StructCounter {
    pub count: u32,
}

pub enum StructCounterMessage {
    ButtonClicked(u32),
}

impl Component for StructCounter {
    type Message = StructCounterMessage;

    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self { count: 0 }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            StructCounterMessage::ButtonClicked(i) => {
                self.count += i;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div>
                <button onclick={ctx.link().callback(|_| StructCounterMessage::ButtonClicked(1))}>{ "Increment" }</button>
                <p>{"Clicked: "} {self.count }</p>
            </div>
        }
    }
}
