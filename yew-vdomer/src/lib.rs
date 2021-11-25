//! Construct [Yew](https://yew.rs)'s Virtual DOM using ergonomic Rust idioms.
//!
//! The API is based around functions and makes no use of macros, like [`html!`][yew::html!]
//! which allows it to provide great IDE support. IntelliSense is your friend when using this library.
//!
//! # Usage
//!
//! ```rust
//! use yew::prelude::*;
//! use yew_vdomer::prelude::*;
//! use gloo_console::log;
//!
//! struct Component1;
//! impl Component for Component1 {
//! #    type Message = ();
//! #    type Properties = ();
//! #    fn create(ctx: &Context<Self>) -> Self { Self }
//!     fn view(&self, ctx: &Context<Self>) -> Html {
//!         h1("Heading ").into()
//!     }
//! }
//!
//! struct Component2;
//! impl Component for Component2 {
//! #    type Message = ();
//! #    type Properties = ();
//! #    fn create(ctx: &Context<Self>) -> Self { Self }
//!     // ...
//!     fn view(&self, ctx: &Context<Self>) -> Html {
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
use yew::virtual_dom::{AttrValue, VComp, VNode, VTag, VText};
use yew::{Component, NodeRef};

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
/// use yew_vdomer::prelude::*;
///
/// button("text")
///     .listener(on_click(|_event| { /* Click Handler */ }));
/// ```

pub type Listener = Rc<dyn yew::virtual_dom::Listener>;

pub trait VElement: Sized {
    fn children_mut(&mut self) -> &mut Vec<VNode>;
    fn listeners_mut(&mut self) -> &mut Vec<Listener>;

    fn child(mut self, element: impl Into<VTag>) -> Self {
        self.children_mut().push(VNode::from(element.into()));
        self
    }

    fn component<C: Component>(mut self, props: C::Properties) -> Self {
        let props = Rc::<C::Properties>::new(props);
        self.children_mut().push(VNode::from(VComp::new::<C>(
            props,
            NodeRef::default(),
            None,
        )));
        self
    }

    fn text(mut self, text: impl Into<AttrValue>) -> Self {
        self.children_mut().push(VNode::from(VText::new(text)));
        self
    }
    fn listener(mut self, listener: Listener) -> Self {
        self.listeners_mut().push(listener);
        self
    }
}

pub struct Attribute {
    key: &'static str,
    value: AttrValue,
}

impl Attribute {
    pub fn new(key: &'static str, value: AttrValue) -> Self {
        Self { key, value }
    }
}

#[macro_export]
macro_rules! build_velement {
    ($ident:ident, $element_name:literal, [$($attr_ident:ident => $attr_name:literal;)+]) => {
        pub struct $ident {
            attributes: Vec<Attribute>,
            children: Vec<VNode>,
            listeners: Vec<Listener>,
        }
        impl $ident {
            pub(crate) fn new() -> Self {
                Self {
                    attributes: vec![],
                    children: vec![],
                    listeners: vec![],
                }
            }

            $(
                pub fn $attr_ident (mut self, value: AttrValue) -> Self {
                    self.attributes.push(Attribute::new($attr_name, value));
                    self
                }
            )+
        }

        impl VElement for $ident {
            fn children_mut(&mut self) -> &mut Vec<VNode> {
                &mut self.children
            }

            fn listeners_mut(&mut self) -> &mut Vec<Listener> {
                &mut self.listeners
            }
        }

        impl Into<VTag> for $ident {
            fn into(self) -> VTag {
                let mut vtag = VTag::new($element_name);
                for attr in self.attributes.into_iter() {
                    vtag.add_attribute(attr.key, attr.value)
                }
                vtag.add_children(self.children.into_iter());
                vtag.set_listener(self.listeners.into_iter().map(Some).collect::<Box<[_]>>());
                vtag
            }
        }

        impl Into<VNode> for $ident {
            fn into(self) -> VNode {
                let vtag: VTag = self.into();
                VNode::from(vtag)
            }
        }
    };
}

pub mod prelude {
    //! A list of types which are useful for using the library.
    //! Unless you have name conflicts, it is recommended to add `yew_dsl::prelude::*;` import.

    pub use super::functions::*;
    pub use super::listeners::*;
    pub use super::VElement;
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;
    use wasm_bindgen_test::*;
    use yew::prelude::*;
    use gloo_console::log;

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn it_works() {
        struct SomeComponent;

        impl Component for SomeComponent {
            type Message = ();
            type Properties = ();

            fn create(_ctx: &Context<Self>) -> Self { Self }

            fn view(&self, _ctx: &Context<Self>) -> Html {
                h1("test").into()
            }
        }

        struct App;

        impl Component for App {
            type Message = ();
            type Properties = ();

            fn create(_ctx: &Context<Self>) -> Self { Self }

            fn view(&self, _ctx: &Context<Self>) -> Html {
                div()
                    .component::<SomeComponent>(yew::props!(SomeComponent::Properties {}))
                    .child(h2("test2"))
                    .listener(on_click(|_e| log!("test")))
                    .into()
            }
        }

        let element = gloo_utils::document().get_element_by_id("output").unwrap();
        yew::start_app_in_element::<App>(element.clone());

        assert_eq!(
            element.inner_html(),
            r#"<div><h1>test</h1><h2>test2</h2></div>"#
        )
    }
}
