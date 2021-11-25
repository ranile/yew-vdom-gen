//! Contains the Yew fragment generation code

use crate::{Listener, VElement};
use yew::virtual_dom::{VList, VNode};

/// A Yew fragment.
///
/// Using the [html!][yew::html!] macro, this is created as follows:
/// ```rust
/// use yew::html;
///
/// html! {
///     <>
///         // children go here
///     </>
/// };
/// ```
pub struct Fragment {
    children: Vec<VNode>,
}

impl Fragment {
    pub(crate) fn new() -> Self {
        Self { children: vec![] }
    }

    pub(crate) fn new_with_children(children: Vec<VNode>) -> Self {
        Self { children }
    }
}

impl VElement for Fragment {
    fn children_mut(&mut self) -> &mut Vec<VNode> {
        &mut self.children
    }

    fn listeners_mut(&mut self) -> &mut Vec<Listener> {
        unimplemented!("fragment can't have listeners")
    }

    fn listener(self, _listener: Listener) -> Self {
        unimplemented!("fragment can't have listeners")
    }
}

impl Into<VList> for Fragment {
    fn into(self) -> VList {
        VList::with_children(self.children, None)
    }
}

impl Into<VNode> for Fragment {
    fn into(self) -> VNode {
        let v_list = VList::with_children(self.children, None);
        VNode::VList(v_list)
    }
}

/// Create a [Fragment] (`<>...</>`)
pub fn fragment() -> Fragment {
    Fragment::new()
}

/// Create a [Fragment] (`<>...</>`)
///
/// Any type that can be converted into [Html][yew::Html] can be passed as children.
/// Of course, that includes every type exported from this crate.
pub fn fragment_with_children<C, I>(c: C) -> Fragment
where
    C: IntoIterator<Item = I>,
    I: Into<VNode>,
{
    let children: Vec<VNode> = c.into_iter().map(|it| it.into()).collect();
    Fragment::new_with_children(children)
}
