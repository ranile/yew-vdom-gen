#![allow(clippy::from_over_into)]

use std::rc::Rc;

pub mod elements;
pub mod functions;
pub mod listeners;

pub type Listener = Rc<dyn yew::virtual_dom::Listener>;

pub mod prelude {
    pub use super::functions::*;
    pub use super::listeners::*;
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;
    use wasm_bindgen_test::*;
    use yew::prelude::*;
    use yew::services::ConsoleService;

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn it_works() {
        struct Component2;

        impl Component for Component2 {
            type Message = ();
            type Properties = ();

            fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
                Self
            }

            fn update(&mut self, _msg: Self::Message) -> ShouldRender {
                false
            }

            fn change(&mut self, _props: Self::Properties) -> ShouldRender {
                false
            }

            fn view(&self) -> Html {
                h1("test").into()
            }
        }

        struct Comp;

        impl Component for Comp {
            type Message = ();
            type Properties = ();

            fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
                Self
            }

            fn update(&mut self, _msg: Self::Message) -> ShouldRender {
                false
            }

            fn change(&mut self, _props: Self::Properties) -> ShouldRender {
                false
            }

            fn view(&self) -> Html {
                div()
                    .component::<Component2>(yew::props!(Component2::Properties {}))
                    .child(h2("test2"))
                    .listener(on_click(|_e| ConsoleService::log("test")))
                    .into()
            }
        }

        let element = yew::utils::document().get_element_by_id("output").unwrap();
        let app: App<Comp> = yew::App::new();
        app.mount(element.clone());

        assert_eq!(
            element.inner_html(),
            r#"<div><h1>test</h1><h2>test2</h2></div>"#
        )
    }
}
