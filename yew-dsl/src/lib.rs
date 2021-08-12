//! Construct [Yew](https://yew.rs)'s Virtual DOM using ergonomic Rust idioms.
//!
//! The API is based around functions and makes no use of macros, like [`html!`][yew::html!]
//! which allows it to provide great IDE support. IntelliSense is your friend when using this library.
//!
//! # Usage
//!
//! ```rust
//! use yew::prelude::*;
//! use yew_dsl::prelude::*;
//!
//! # macro_rules! log {
//! # ($($tt:tt)*) => {{}}
//! # }
//! #
//! struct Component1;
//! impl Component for Component1 {
//! #    type Message = ();
//! #    type Properties = ();
//! #    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self { Self }
//! #    fn update(&mut self, _msg: Self::Message) -> ShouldRender { false }
//! #    fn change(&mut self, _props: Self::Properties) -> ShouldRender { false }
//!     // ...
//!     fn view(&self) -> Html {
//!         h1("Heading ").into()
//!     }
//! }
//!
//! struct Component2;
//! impl Component for Component2 {
//! #    type Message = ();
//! #    type Properties = ();
//! #    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self { Self }
//! #    fn update(&mut self, _msg: Self::Message) -> ShouldRender { false }
//! #    fn change(&mut self, _props: Self::Properties) -> ShouldRender { false }
//!     // ...
//!     fn view(&self) -> Html {
//!         div()
//!             .component::<Component1>(yew::props!(Component1::Properties {}))
//!             .child(h2("test2"))
//!             .listener(on_click(|_e| log!("test")))
//!             .into()
//!     }
//! }
//! ```

#![allow(clippy::from_over_into)]

use std::rc::Rc;

/// Houses all the [HTML elements](https://developer.mozilla.org/en-US/docs/Web/HTML/Element)
pub mod elements;

/// Functions for creating elements.
///
/// Most of these functions are just call `new` function of their respective element struct.
/// However some of them, like [`p`][functions::p], takes an argument to be set as it's
/// inner text node.
pub mod functions;

/// Event Listeners that can be attached using element's `listener` method
pub mod listeners;

/// A listener that can be attached to an element.
///
/// This type is returned from [listener functions][listeners] and passed to
/// element's `listener` method
///
/// # Example
///
/// ```rust
/// # use yew_dsl::prelude::*;
/// button("text")
///     .listener(on_click(|_event| { /* Click Handler */ }));
/// ```
pub type Listener = Rc<dyn yew::virtual_dom::Listener>;

pub mod prelude {
    //! A list of types which are useful for using the library.
    //! Unless you have name conflicts, it is recommended to add `yew_dsl::prelude::*;` import.

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
