use yew::virtual_dom::{AttrValue, Attributes, PositionalAttr, VComp, VNode, VTag, VText};
use yew::{Component, NodeRef};
pub struct Html {
    attributes: Vec<PositionalAttr>,
    children: Vec<VNode>,
}

impl Html {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("accesskey", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("autocapitalize", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("class", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contextmenu", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("dir", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("draggable", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("hidden", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("id", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("itemprop", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("lang", value));
        self
    }

    pub fn manifest(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("manifest", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("slot", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("spellcheck", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("style", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("tabindex", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("translate", value));
        self
    }

    pub fn child(mut self, element: impl Into<VTag>) -> Self {
        self.children.push(VNode::from(element.into()));
        self
    }

    pub fn component<C: Component>(mut self, props: C::Properties) -> Self {
        self.children.push(VNode::from(VComp::new::<C>(
            props,
            NodeRef::default(),
            None,
        )));
        self
    }

    pub fn text(mut self, text: impl Into<AttrValue>) -> Self {
        self.children.push(VNode::from(VText::new(text)));
        self
    }
}

impl Into<VTag> for Html {
    fn into(self) -> VTag {
        let mut vtag = VTag::new("html");
        vtag.attributes = Attributes::Vec(self.attributes);
        vtag.add_children(self.children.into_iter());
        vtag
    }
}

impl Into<VNode> for Html {
    fn into(self) -> VNode {
        let vtag: VTag = self.into();
        VNode::from(vtag)
    }
}

pub struct Base {
    attributes: Vec<PositionalAttr>,
    children: Vec<VNode>,
}

impl Base {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("accesskey", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("autocapitalize", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("class", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contextmenu", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("dir", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("draggable", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("hidden", value));
        self
    }

    pub fn href(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("href", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("id", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("itemprop", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("lang", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("slot", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("spellcheck", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("style", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("tabindex", value));
        self
    }

    pub fn target(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("target", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("translate", value));
        self
    }

    pub fn child(mut self, element: impl Into<VTag>) -> Self {
        self.children.push(VNode::from(element.into()));
        self
    }

    pub fn component<C: Component>(mut self, props: C::Properties) -> Self {
        self.children.push(VNode::from(VComp::new::<C>(
            props,
            NodeRef::default(),
            None,
        )));
        self
    }

    pub fn text(mut self, text: impl Into<AttrValue>) -> Self {
        self.children.push(VNode::from(VText::new(text)));
        self
    }
}

impl Into<VTag> for Base {
    fn into(self) -> VTag {
        let mut vtag = VTag::new("base");
        vtag.attributes = Attributes::Vec(self.attributes);
        vtag.add_children(self.children.into_iter());
        vtag
    }
}

impl Into<VNode> for Base {
    fn into(self) -> VNode {
        let vtag: VTag = self.into();
        VNode::from(vtag)
    }
}

pub struct Head {
    attributes: Vec<PositionalAttr>,
    children: Vec<VNode>,
}

impl Head {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("accesskey", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("autocapitalize", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("class", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contextmenu", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("dir", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("draggable", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("hidden", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("id", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("itemprop", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("lang", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("slot", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("spellcheck", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("style", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("tabindex", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("translate", value));
        self
    }

    pub fn child(mut self, element: impl Into<VTag>) -> Self {
        self.children.push(VNode::from(element.into()));
        self
    }

    pub fn component<C: Component>(mut self, props: C::Properties) -> Self {
        self.children.push(VNode::from(VComp::new::<C>(
            props,
            NodeRef::default(),
            None,
        )));
        self
    }

    pub fn text(mut self, text: impl Into<AttrValue>) -> Self {
        self.children.push(VNode::from(VText::new(text)));
        self
    }
}

impl Into<VTag> for Head {
    fn into(self) -> VTag {
        let mut vtag = VTag::new("head");
        vtag.attributes = Attributes::Vec(self.attributes);
        vtag.add_children(self.children.into_iter());
        vtag
    }
}

impl Into<VNode> for Head {
    fn into(self) -> VNode {
        let vtag: VTag = self.into();
        VNode::from(vtag)
    }
}

pub struct Link {
    attributes: Vec<PositionalAttr>,
    children: Vec<VNode>,
}

impl Link {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("accesskey", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("autocapitalize", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("class", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contextmenu", value));
        self
    }

    pub fn crossorigin(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("crossorigin", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("dir", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("draggable", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("hidden", value));
        self
    }

    pub fn href(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("href", value));
        self
    }

    pub fn hreflang(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("hreflang", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("id", value));
        self
    }

    pub fn importance(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("importance", value));
        self
    }

    pub fn integrity(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("integrity", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("itemprop", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("lang", value));
        self
    }

    pub fn media(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("media", value));
        self
    }

    pub fn referrerpolicy(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("referrerpolicy", value));
        self
    }

    pub fn rel(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("rel", value));
        self
    }

    pub fn sizes(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("sizes", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("slot", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("spellcheck", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("style", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("tabindex", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("translate", value));
        self
    }

    pub fn child(mut self, element: impl Into<VTag>) -> Self {
        self.children.push(VNode::from(element.into()));
        self
    }

    pub fn component<C: Component>(mut self, props: C::Properties) -> Self {
        self.children.push(VNode::from(VComp::new::<C>(
            props,
            NodeRef::default(),
            None,
        )));
        self
    }

    pub fn text(mut self, text: impl Into<AttrValue>) -> Self {
        self.children.push(VNode::from(VText::new(text)));
        self
    }
}

impl Into<VTag> for Link {
    fn into(self) -> VTag {
        let mut vtag = VTag::new("link");
        vtag.attributes = Attributes::Vec(self.attributes);
        vtag.add_children(self.children.into_iter());
        vtag
    }
}

impl Into<VNode> for Link {
    fn into(self) -> VNode {
        let vtag: VTag = self.into();
        VNode::from(vtag)
    }
}

pub struct Meta {
    attributes: Vec<PositionalAttr>,
    children: Vec<VNode>,
}

impl Meta {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("accesskey", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("autocapitalize", value));
        self
    }

    pub fn charset(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("charset", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("class", value));
        self
    }

    pub fn content(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("content", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contextmenu", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("dir", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("draggable", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("hidden", value));
        self
    }

    pub fn http_equiv(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("http_equiv", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("id", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("itemprop", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("lang", value));
        self
    }

    pub fn name(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("name", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("slot", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("spellcheck", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("style", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("tabindex", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("translate", value));
        self
    }

    pub fn child(mut self, element: impl Into<VTag>) -> Self {
        self.children.push(VNode::from(element.into()));
        self
    }

    pub fn component<C: Component>(mut self, props: C::Properties) -> Self {
        self.children.push(VNode::from(VComp::new::<C>(
            props,
            NodeRef::default(),
            None,
        )));
        self
    }

    pub fn text(mut self, text: impl Into<AttrValue>) -> Self {
        self.children.push(VNode::from(VText::new(text)));
        self
    }
}

impl Into<VTag> for Meta {
    fn into(self) -> VTag {
        let mut vtag = VTag::new("meta");
        vtag.attributes = Attributes::Vec(self.attributes);
        vtag.add_children(self.children.into_iter());
        vtag
    }
}

impl Into<VNode> for Meta {
    fn into(self) -> VNode {
        let vtag: VTag = self.into();
        VNode::from(vtag)
    }
}

pub struct Style {
    attributes: Vec<PositionalAttr>,
    children: Vec<VNode>,
}

impl Style {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("accesskey", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("autocapitalize", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("class", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contextmenu", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("dir", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("draggable", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("hidden", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("id", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("itemprop", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("lang", value));
        self
    }

    pub fn media(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("media", value));
        self
    }

    pub fn scoped(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("scoped", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("slot", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("spellcheck", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("style", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("tabindex", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("translate", value));
        self
    }

    pub fn r#type(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("type", value));
        self
    }

    pub fn child(mut self, element: impl Into<VTag>) -> Self {
        self.children.push(VNode::from(element.into()));
        self
    }

    pub fn component<C: Component>(mut self, props: C::Properties) -> Self {
        self.children.push(VNode::from(VComp::new::<C>(
            props,
            NodeRef::default(),
            None,
        )));
        self
    }

    pub fn text(mut self, text: impl Into<AttrValue>) -> Self {
        self.children.push(VNode::from(VText::new(text)));
        self
    }
}

impl Into<VTag> for Style {
    fn into(self) -> VTag {
        let mut vtag = VTag::new("style");
        vtag.attributes = Attributes::Vec(self.attributes);
        vtag.add_children(self.children.into_iter());
        vtag
    }
}

impl Into<VNode> for Style {
    fn into(self) -> VNode {
        let vtag: VTag = self.into();
        VNode::from(vtag)
    }
}

pub struct Title {
    attributes: Vec<PositionalAttr>,
    children: Vec<VNode>,
}

impl Title {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("accesskey", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("autocapitalize", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("class", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contextmenu", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("dir", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("draggable", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("hidden", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("id", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("itemprop", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("lang", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("slot", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("spellcheck", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("style", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("tabindex", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("translate", value));
        self
    }

    pub fn child(mut self, element: impl Into<VTag>) -> Self {
        self.children.push(VNode::from(element.into()));
        self
    }

    pub fn component<C: Component>(mut self, props: C::Properties) -> Self {
        self.children.push(VNode::from(VComp::new::<C>(
            props,
            NodeRef::default(),
            None,
        )));
        self
    }

    pub fn text(mut self, text: impl Into<AttrValue>) -> Self {
        self.children.push(VNode::from(VText::new(text)));
        self
    }
}

impl Into<VTag> for Title {
    fn into(self) -> VTag {
        let mut vtag = VTag::new("title");
        vtag.attributes = Attributes::Vec(self.attributes);
        vtag.add_children(self.children.into_iter());
        vtag
    }
}

impl Into<VNode> for Title {
    fn into(self) -> VNode {
        let vtag: VTag = self.into();
        VNode::from(vtag)
    }
}

pub struct Body {
    attributes: Vec<PositionalAttr>,
    children: Vec<VNode>,
}

impl Body {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("accesskey", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("autocapitalize", value));
        self
    }

    pub fn background(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("background", value));
        self
    }

    pub fn bgcolor(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("bgcolor", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("class", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contextmenu", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("dir", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("draggable", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("hidden", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("id", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("itemprop", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("lang", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("slot", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("spellcheck", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("style", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("tabindex", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("translate", value));
        self
    }

    pub fn child(mut self, element: impl Into<VTag>) -> Self {
        self.children.push(VNode::from(element.into()));
        self
    }

    pub fn component<C: Component>(mut self, props: C::Properties) -> Self {
        self.children.push(VNode::from(VComp::new::<C>(
            props,
            NodeRef::default(),
            None,
        )));
        self
    }

    pub fn text(mut self, text: impl Into<AttrValue>) -> Self {
        self.children.push(VNode::from(VText::new(text)));
        self
    }
}

impl Into<VTag> for Body {
    fn into(self) -> VTag {
        let mut vtag = VTag::new("body");
        vtag.attributes = Attributes::Vec(self.attributes);
        vtag.add_children(self.children.into_iter());
        vtag
    }
}

impl Into<VNode> for Body {
    fn into(self) -> VNode {
        let vtag: VTag = self.into();
        VNode::from(vtag)
    }
}

pub struct Address {
    attributes: Vec<PositionalAttr>,
    children: Vec<VNode>,
}

impl Address {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("accesskey", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("autocapitalize", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("class", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contextmenu", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("dir", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("draggable", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("hidden", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("id", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("itemprop", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("lang", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("slot", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("spellcheck", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("style", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("tabindex", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("translate", value));
        self
    }

    pub fn child(mut self, element: impl Into<VTag>) -> Self {
        self.children.push(VNode::from(element.into()));
        self
    }

    pub fn component<C: Component>(mut self, props: C::Properties) -> Self {
        self.children.push(VNode::from(VComp::new::<C>(
            props,
            NodeRef::default(),
            None,
        )));
        self
    }

    pub fn text(mut self, text: impl Into<AttrValue>) -> Self {
        self.children.push(VNode::from(VText::new(text)));
        self
    }
}

impl Into<VTag> for Address {
    fn into(self) -> VTag {
        let mut vtag = VTag::new("address");
        vtag.attributes = Attributes::Vec(self.attributes);
        vtag.add_children(self.children.into_iter());
        vtag
    }
}

impl Into<VNode> for Address {
    fn into(self) -> VNode {
        let vtag: VTag = self.into();
        VNode::from(vtag)
    }
}

pub struct Article {
    attributes: Vec<PositionalAttr>,
    children: Vec<VNode>,
}

impl Article {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("accesskey", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("autocapitalize", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("class", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contextmenu", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("dir", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("draggable", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("hidden", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("id", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("itemprop", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("lang", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("slot", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("spellcheck", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("style", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("tabindex", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("translate", value));
        self
    }

    pub fn child(mut self, element: impl Into<VTag>) -> Self {
        self.children.push(VNode::from(element.into()));
        self
    }

    pub fn component<C: Component>(mut self, props: C::Properties) -> Self {
        self.children.push(VNode::from(VComp::new::<C>(
            props,
            NodeRef::default(),
            None,
        )));
        self
    }

    pub fn text(mut self, text: impl Into<AttrValue>) -> Self {
        self.children.push(VNode::from(VText::new(text)));
        self
    }
}

impl Into<VTag> for Article {
    fn into(self) -> VTag {
        let mut vtag = VTag::new("article");
        vtag.attributes = Attributes::Vec(self.attributes);
        vtag.add_children(self.children.into_iter());
        vtag
    }
}

impl Into<VNode> for Article {
    fn into(self) -> VNode {
        let vtag: VTag = self.into();
        VNode::from(vtag)
    }
}

pub struct Aside {
    attributes: Vec<PositionalAttr>,
    children: Vec<VNode>,
}

impl Aside {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("accesskey", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("autocapitalize", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("class", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contextmenu", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("dir", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("draggable", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("hidden", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("id", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("itemprop", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("lang", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("slot", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("spellcheck", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("style", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("tabindex", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("translate", value));
        self
    }

    pub fn child(mut self, element: impl Into<VTag>) -> Self {
        self.children.push(VNode::from(element.into()));
        self
    }

    pub fn component<C: Component>(mut self, props: C::Properties) -> Self {
        self.children.push(VNode::from(VComp::new::<C>(
            props,
            NodeRef::default(),
            None,
        )));
        self
    }

    pub fn text(mut self, text: impl Into<AttrValue>) -> Self {
        self.children.push(VNode::from(VText::new(text)));
        self
    }
}

impl Into<VTag> for Aside {
    fn into(self) -> VTag {
        let mut vtag = VTag::new("aside");
        vtag.attributes = Attributes::Vec(self.attributes);
        vtag.add_children(self.children.into_iter());
        vtag
    }
}

impl Into<VNode> for Aside {
    fn into(self) -> VNode {
        let vtag: VTag = self.into();
        VNode::from(vtag)
    }
}

pub struct Footer {
    attributes: Vec<PositionalAttr>,
    children: Vec<VNode>,
}

impl Footer {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("accesskey", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("autocapitalize", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("class", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contextmenu", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("dir", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("draggable", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("hidden", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("id", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("itemprop", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("lang", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("slot", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("spellcheck", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("style", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("tabindex", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("translate", value));
        self
    }

    pub fn child(mut self, element: impl Into<VTag>) -> Self {
        self.children.push(VNode::from(element.into()));
        self
    }

    pub fn component<C: Component>(mut self, props: C::Properties) -> Self {
        self.children.push(VNode::from(VComp::new::<C>(
            props,
            NodeRef::default(),
            None,
        )));
        self
    }

    pub fn text(mut self, text: impl Into<AttrValue>) -> Self {
        self.children.push(VNode::from(VText::new(text)));
        self
    }
}

impl Into<VTag> for Footer {
    fn into(self) -> VTag {
        let mut vtag = VTag::new("footer");
        vtag.attributes = Attributes::Vec(self.attributes);
        vtag.add_children(self.children.into_iter());
        vtag
    }
}

impl Into<VNode> for Footer {
    fn into(self) -> VNode {
        let vtag: VTag = self.into();
        VNode::from(vtag)
    }
}

pub struct Header {
    attributes: Vec<PositionalAttr>,
    children: Vec<VNode>,
}

impl Header {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("accesskey", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("autocapitalize", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("class", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contextmenu", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("dir", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("draggable", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("hidden", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("id", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("itemprop", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("lang", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("slot", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("spellcheck", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("style", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("tabindex", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("translate", value));
        self
    }

    pub fn child(mut self, element: impl Into<VTag>) -> Self {
        self.children.push(VNode::from(element.into()));
        self
    }

    pub fn component<C: Component>(mut self, props: C::Properties) -> Self {
        self.children.push(VNode::from(VComp::new::<C>(
            props,
            NodeRef::default(),
            None,
        )));
        self
    }

    pub fn text(mut self, text: impl Into<AttrValue>) -> Self {
        self.children.push(VNode::from(VText::new(text)));
        self
    }
}

impl Into<VTag> for Header {
    fn into(self) -> VTag {
        let mut vtag = VTag::new("header");
        vtag.attributes = Attributes::Vec(self.attributes);
        vtag.add_children(self.children.into_iter());
        vtag
    }
}

impl Into<VNode> for Header {
    fn into(self) -> VNode {
        let vtag: VTag = self.into();
        VNode::from(vtag)
    }
}

pub struct Main {
    attributes: Vec<PositionalAttr>,
    children: Vec<VNode>,
}

impl Main {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("accesskey", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("autocapitalize", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("class", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contextmenu", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("dir", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("draggable", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("hidden", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("id", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("itemprop", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("lang", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("slot", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("spellcheck", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("style", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("tabindex", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("translate", value));
        self
    }

    pub fn child(mut self, element: impl Into<VTag>) -> Self {
        self.children.push(VNode::from(element.into()));
        self
    }

    pub fn component<C: Component>(mut self, props: C::Properties) -> Self {
        self.children.push(VNode::from(VComp::new::<C>(
            props,
            NodeRef::default(),
            None,
        )));
        self
    }

    pub fn text(mut self, text: impl Into<AttrValue>) -> Self {
        self.children.push(VNode::from(VText::new(text)));
        self
    }
}

impl Into<VTag> for Main {
    fn into(self) -> VTag {
        let mut vtag = VTag::new("main");
        vtag.attributes = Attributes::Vec(self.attributes);
        vtag.add_children(self.children.into_iter());
        vtag
    }
}

impl Into<VNode> for Main {
    fn into(self) -> VNode {
        let vtag: VTag = self.into();
        VNode::from(vtag)
    }
}

pub struct Nav {
    attributes: Vec<PositionalAttr>,
    children: Vec<VNode>,
}

impl Nav {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("accesskey", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("autocapitalize", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("class", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contextmenu", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("dir", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("draggable", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("hidden", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("id", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("itemprop", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("lang", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("slot", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("spellcheck", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("style", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("tabindex", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("translate", value));
        self
    }

    pub fn child(mut self, element: impl Into<VTag>) -> Self {
        self.children.push(VNode::from(element.into()));
        self
    }

    pub fn component<C: Component>(mut self, props: C::Properties) -> Self {
        self.children.push(VNode::from(VComp::new::<C>(
            props,
            NodeRef::default(),
            None,
        )));
        self
    }

    pub fn text(mut self, text: impl Into<AttrValue>) -> Self {
        self.children.push(VNode::from(VText::new(text)));
        self
    }
}

impl Into<VTag> for Nav {
    fn into(self) -> VTag {
        let mut vtag = VTag::new("nav");
        vtag.attributes = Attributes::Vec(self.attributes);
        vtag.add_children(self.children.into_iter());
        vtag
    }
}

impl Into<VNode> for Nav {
    fn into(self) -> VNode {
        let vtag: VTag = self.into();
        VNode::from(vtag)
    }
}

pub struct Section {
    attributes: Vec<PositionalAttr>,
    children: Vec<VNode>,
}

impl Section {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("accesskey", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("autocapitalize", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("class", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contextmenu", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("dir", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("draggable", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("hidden", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("id", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("itemprop", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("lang", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("slot", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("spellcheck", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("style", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("tabindex", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("translate", value));
        self
    }

    pub fn child(mut self, element: impl Into<VTag>) -> Self {
        self.children.push(VNode::from(element.into()));
        self
    }

    pub fn component<C: Component>(mut self, props: C::Properties) -> Self {
        self.children.push(VNode::from(VComp::new::<C>(
            props,
            NodeRef::default(),
            None,
        )));
        self
    }

    pub fn text(mut self, text: impl Into<AttrValue>) -> Self {
        self.children.push(VNode::from(VText::new(text)));
        self
    }
}

impl Into<VTag> for Section {
    fn into(self) -> VTag {
        let mut vtag = VTag::new("section");
        vtag.attributes = Attributes::Vec(self.attributes);
        vtag.add_children(self.children.into_iter());
        vtag
    }
}

impl Into<VNode> for Section {
    fn into(self) -> VNode {
        let vtag: VTag = self.into();
        VNode::from(vtag)
    }
}

pub struct Blockquote {
    attributes: Vec<PositionalAttr>,
    children: Vec<VNode>,
}

impl Blockquote {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("accesskey", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("autocapitalize", value));
        self
    }

    pub fn cite(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("cite", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("class", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contextmenu", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("dir", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("draggable", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("hidden", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("id", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("itemprop", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("lang", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("slot", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("spellcheck", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("style", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("tabindex", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("translate", value));
        self
    }

    pub fn child(mut self, element: impl Into<VTag>) -> Self {
        self.children.push(VNode::from(element.into()));
        self
    }

    pub fn component<C: Component>(mut self, props: C::Properties) -> Self {
        self.children.push(VNode::from(VComp::new::<C>(
            props,
            NodeRef::default(),
            None,
        )));
        self
    }

    pub fn text(mut self, text: impl Into<AttrValue>) -> Self {
        self.children.push(VNode::from(VText::new(text)));
        self
    }
}

impl Into<VTag> for Blockquote {
    fn into(self) -> VTag {
        let mut vtag = VTag::new("blockquote");
        vtag.attributes = Attributes::Vec(self.attributes);
        vtag.add_children(self.children.into_iter());
        vtag
    }
}

impl Into<VNode> for Blockquote {
    fn into(self) -> VNode {
        let vtag: VTag = self.into();
        VNode::from(vtag)
    }
}

pub struct Dd {
    attributes: Vec<PositionalAttr>,
    children: Vec<VNode>,
}

impl Dd {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("accesskey", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("autocapitalize", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("class", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contextmenu", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("dir", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("draggable", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("hidden", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("id", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("itemprop", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("lang", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("slot", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("spellcheck", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("style", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("tabindex", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("translate", value));
        self
    }

    pub fn child(mut self, element: impl Into<VTag>) -> Self {
        self.children.push(VNode::from(element.into()));
        self
    }

    pub fn component<C: Component>(mut self, props: C::Properties) -> Self {
        self.children.push(VNode::from(VComp::new::<C>(
            props,
            NodeRef::default(),
            None,
        )));
        self
    }

    pub fn text(mut self, text: impl Into<AttrValue>) -> Self {
        self.children.push(VNode::from(VText::new(text)));
        self
    }
}

impl Into<VTag> for Dd {
    fn into(self) -> VTag {
        let mut vtag = VTag::new("dd");
        vtag.attributes = Attributes::Vec(self.attributes);
        vtag.add_children(self.children.into_iter());
        vtag
    }
}

impl Into<VNode> for Dd {
    fn into(self) -> VNode {
        let vtag: VTag = self.into();
        VNode::from(vtag)
    }
}

pub struct Div {
    attributes: Vec<PositionalAttr>,
    children: Vec<VNode>,
}

impl Div {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("accesskey", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("autocapitalize", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("class", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contextmenu", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("dir", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("draggable", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("hidden", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("id", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("itemprop", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("lang", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("slot", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("spellcheck", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("style", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("tabindex", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("translate", value));
        self
    }

    pub fn child(mut self, element: impl Into<VTag>) -> Self {
        self.children.push(VNode::from(element.into()));
        self
    }

    pub fn component<C: Component>(mut self, props: C::Properties) -> Self {
        self.children.push(VNode::from(VComp::new::<C>(
            props,
            NodeRef::default(),
            None,
        )));
        self
    }

    pub fn text(mut self, text: impl Into<AttrValue>) -> Self {
        self.children.push(VNode::from(VText::new(text)));
        self
    }
}

impl Into<VTag> for Div {
    fn into(self) -> VTag {
        let mut vtag = VTag::new("div");
        vtag.attributes = Attributes::Vec(self.attributes);
        vtag.add_children(self.children.into_iter());
        vtag
    }
}

impl Into<VNode> for Div {
    fn into(self) -> VNode {
        let vtag: VTag = self.into();
        VNode::from(vtag)
    }
}

pub struct Dl {
    attributes: Vec<PositionalAttr>,
    children: Vec<VNode>,
}

impl Dl {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("accesskey", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("autocapitalize", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("class", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contextmenu", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("dir", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("draggable", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("hidden", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("id", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("itemprop", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("lang", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("slot", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("spellcheck", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("style", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("tabindex", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("translate", value));
        self
    }

    pub fn child(mut self, element: impl Into<VTag>) -> Self {
        self.children.push(VNode::from(element.into()));
        self
    }

    pub fn component<C: Component>(mut self, props: C::Properties) -> Self {
        self.children.push(VNode::from(VComp::new::<C>(
            props,
            NodeRef::default(),
            None,
        )));
        self
    }

    pub fn text(mut self, text: impl Into<AttrValue>) -> Self {
        self.children.push(VNode::from(VText::new(text)));
        self
    }
}

impl Into<VTag> for Dl {
    fn into(self) -> VTag {
        let mut vtag = VTag::new("dl");
        vtag.attributes = Attributes::Vec(self.attributes);
        vtag.add_children(self.children.into_iter());
        vtag
    }
}

impl Into<VNode> for Dl {
    fn into(self) -> VNode {
        let vtag: VTag = self.into();
        VNode::from(vtag)
    }
}

pub struct Dt {
    attributes: Vec<PositionalAttr>,
    children: Vec<VNode>,
}

impl Dt {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("accesskey", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("autocapitalize", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("class", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contextmenu", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("dir", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("draggable", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("hidden", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("id", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("itemprop", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("lang", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("slot", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("spellcheck", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("style", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("tabindex", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("translate", value));
        self
    }

    pub fn child(mut self, element: impl Into<VTag>) -> Self {
        self.children.push(VNode::from(element.into()));
        self
    }

    pub fn component<C: Component>(mut self, props: C::Properties) -> Self {
        self.children.push(VNode::from(VComp::new::<C>(
            props,
            NodeRef::default(),
            None,
        )));
        self
    }

    pub fn text(mut self, text: impl Into<AttrValue>) -> Self {
        self.children.push(VNode::from(VText::new(text)));
        self
    }
}

impl Into<VTag> for Dt {
    fn into(self) -> VTag {
        let mut vtag = VTag::new("dt");
        vtag.attributes = Attributes::Vec(self.attributes);
        vtag.add_children(self.children.into_iter());
        vtag
    }
}

impl Into<VNode> for Dt {
    fn into(self) -> VNode {
        let vtag: VTag = self.into();
        VNode::from(vtag)
    }
}

pub struct Figcaption {
    attributes: Vec<PositionalAttr>,
    children: Vec<VNode>,
}

impl Figcaption {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("accesskey", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("autocapitalize", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("class", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contextmenu", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("dir", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("draggable", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("hidden", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("id", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("itemprop", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("lang", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("slot", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("spellcheck", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("style", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("tabindex", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("translate", value));
        self
    }

    pub fn child(mut self, element: impl Into<VTag>) -> Self {
        self.children.push(VNode::from(element.into()));
        self
    }

    pub fn component<C: Component>(mut self, props: C::Properties) -> Self {
        self.children.push(VNode::from(VComp::new::<C>(
            props,
            NodeRef::default(),
            None,
        )));
        self
    }

    pub fn text(mut self, text: impl Into<AttrValue>) -> Self {
        self.children.push(VNode::from(VText::new(text)));
        self
    }
}

impl Into<VTag> for Figcaption {
    fn into(self) -> VTag {
        let mut vtag = VTag::new("figcaption");
        vtag.attributes = Attributes::Vec(self.attributes);
        vtag.add_children(self.children.into_iter());
        vtag
    }
}

impl Into<VNode> for Figcaption {
    fn into(self) -> VNode {
        let vtag: VTag = self.into();
        VNode::from(vtag)
    }
}

pub struct Figure {
    attributes: Vec<PositionalAttr>,
    children: Vec<VNode>,
}

impl Figure {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("accesskey", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("autocapitalize", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("class", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contextmenu", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("dir", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("draggable", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("hidden", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("id", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("itemprop", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("lang", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("slot", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("spellcheck", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("style", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("tabindex", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("translate", value));
        self
    }

    pub fn child(mut self, element: impl Into<VTag>) -> Self {
        self.children.push(VNode::from(element.into()));
        self
    }

    pub fn component<C: Component>(mut self, props: C::Properties) -> Self {
        self.children.push(VNode::from(VComp::new::<C>(
            props,
            NodeRef::default(),
            None,
        )));
        self
    }

    pub fn text(mut self, text: impl Into<AttrValue>) -> Self {
        self.children.push(VNode::from(VText::new(text)));
        self
    }
}

impl Into<VTag> for Figure {
    fn into(self) -> VTag {
        let mut vtag = VTag::new("figure");
        vtag.attributes = Attributes::Vec(self.attributes);
        vtag.add_children(self.children.into_iter());
        vtag
    }
}

impl Into<VNode> for Figure {
    fn into(self) -> VNode {
        let vtag: VTag = self.into();
        VNode::from(vtag)
    }
}

pub struct Hr {
    attributes: Vec<PositionalAttr>,
    children: Vec<VNode>,
}

impl Hr {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("accesskey", value));
        self
    }

    pub fn align(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("align", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("autocapitalize", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("class", value));
        self
    }

    pub fn color(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("color", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contextmenu", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("dir", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("draggable", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("hidden", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("id", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("itemprop", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("lang", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("slot", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("spellcheck", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("style", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("tabindex", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("translate", value));
        self
    }

    pub fn child(mut self, element: impl Into<VTag>) -> Self {
        self.children.push(VNode::from(element.into()));
        self
    }

    pub fn component<C: Component>(mut self, props: C::Properties) -> Self {
        self.children.push(VNode::from(VComp::new::<C>(
            props,
            NodeRef::default(),
            None,
        )));
        self
    }

    pub fn text(mut self, text: impl Into<AttrValue>) -> Self {
        self.children.push(VNode::from(VText::new(text)));
        self
    }
}

impl Into<VTag> for Hr {
    fn into(self) -> VTag {
        let mut vtag = VTag::new("hr");
        vtag.attributes = Attributes::Vec(self.attributes);
        vtag.add_children(self.children.into_iter());
        vtag
    }
}

impl Into<VNode> for Hr {
    fn into(self) -> VNode {
        let vtag: VTag = self.into();
        VNode::from(vtag)
    }
}

pub struct Li {
    attributes: Vec<PositionalAttr>,
    children: Vec<VNode>,
}

impl Li {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("accesskey", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("autocapitalize", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("class", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contextmenu", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("dir", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("draggable", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("hidden", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("id", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("itemprop", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("lang", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("slot", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("spellcheck", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("style", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("tabindex", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("translate", value));
        self
    }

    pub fn value(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("value", value));
        self
    }

    pub fn child(mut self, element: impl Into<VTag>) -> Self {
        self.children.push(VNode::from(element.into()));
        self
    }

    pub fn component<C: Component>(mut self, props: C::Properties) -> Self {
        self.children.push(VNode::from(VComp::new::<C>(
            props,
            NodeRef::default(),
            None,
        )));
        self
    }

    pub fn text(mut self, text: impl Into<AttrValue>) -> Self {
        self.children.push(VNode::from(VText::new(text)));
        self
    }
}

impl Into<VTag> for Li {
    fn into(self) -> VTag {
        let mut vtag = VTag::new("li");
        vtag.attributes = Attributes::Vec(self.attributes);
        vtag.add_children(self.children.into_iter());
        vtag
    }
}

impl Into<VNode> for Li {
    fn into(self) -> VNode {
        let vtag: VTag = self.into();
        VNode::from(vtag)
    }
}

pub struct Ol {
    attributes: Vec<PositionalAttr>,
    children: Vec<VNode>,
}

impl Ol {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("accesskey", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("autocapitalize", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("class", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contextmenu", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("dir", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("draggable", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("hidden", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("id", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("itemprop", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("lang", value));
        self
    }

    pub fn reversed(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("reversed", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("slot", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("spellcheck", value));
        self
    }

    pub fn start(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("start", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("style", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("tabindex", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("translate", value));
        self
    }

    pub fn child(mut self, element: impl Into<VTag>) -> Self {
        self.children.push(VNode::from(element.into()));
        self
    }

    pub fn component<C: Component>(mut self, props: C::Properties) -> Self {
        self.children.push(VNode::from(VComp::new::<C>(
            props,
            NodeRef::default(),
            None,
        )));
        self
    }

    pub fn text(mut self, text: impl Into<AttrValue>) -> Self {
        self.children.push(VNode::from(VText::new(text)));
        self
    }
}

impl Into<VTag> for Ol {
    fn into(self) -> VTag {
        let mut vtag = VTag::new("ol");
        vtag.attributes = Attributes::Vec(self.attributes);
        vtag.add_children(self.children.into_iter());
        vtag
    }
}

impl Into<VNode> for Ol {
    fn into(self) -> VNode {
        let vtag: VTag = self.into();
        VNode::from(vtag)
    }
}

pub struct P {
    attributes: Vec<PositionalAttr>,
    children: Vec<VNode>,
}

impl P {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("accesskey", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("autocapitalize", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("class", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contextmenu", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("dir", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("draggable", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("hidden", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("id", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("itemprop", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("lang", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("slot", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("spellcheck", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("style", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("tabindex", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("translate", value));
        self
    }

    pub fn child(mut self, element: impl Into<VTag>) -> Self {
        self.children.push(VNode::from(element.into()));
        self
    }

    pub fn component<C: Component>(mut self, props: C::Properties) -> Self {
        self.children.push(VNode::from(VComp::new::<C>(
            props,
            NodeRef::default(),
            None,
        )));
        self
    }

    pub fn text(mut self, text: impl Into<AttrValue>) -> Self {
        self.children.push(VNode::from(VText::new(text)));
        self
    }
}

impl Into<VTag> for P {
    fn into(self) -> VTag {
        let mut vtag = VTag::new("p");
        vtag.attributes = Attributes::Vec(self.attributes);
        vtag.add_children(self.children.into_iter());
        vtag
    }
}

impl Into<VNode> for P {
    fn into(self) -> VNode {
        let vtag: VTag = self.into();
        VNode::from(vtag)
    }
}

pub struct Pre {
    attributes: Vec<PositionalAttr>,
    children: Vec<VNode>,
}

impl Pre {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("accesskey", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("autocapitalize", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("class", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contextmenu", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("dir", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("draggable", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("hidden", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("id", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("itemprop", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("lang", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("slot", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("spellcheck", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("style", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("tabindex", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("translate", value));
        self
    }

    pub fn child(mut self, element: impl Into<VTag>) -> Self {
        self.children.push(VNode::from(element.into()));
        self
    }

    pub fn component<C: Component>(mut self, props: C::Properties) -> Self {
        self.children.push(VNode::from(VComp::new::<C>(
            props,
            NodeRef::default(),
            None,
        )));
        self
    }

    pub fn text(mut self, text: impl Into<AttrValue>) -> Self {
        self.children.push(VNode::from(VText::new(text)));
        self
    }
}

impl Into<VTag> for Pre {
    fn into(self) -> VTag {
        let mut vtag = VTag::new("pre");
        vtag.attributes = Attributes::Vec(self.attributes);
        vtag.add_children(self.children.into_iter());
        vtag
    }
}

impl Into<VNode> for Pre {
    fn into(self) -> VNode {
        let vtag: VTag = self.into();
        VNode::from(vtag)
    }
}

pub struct Ul {
    attributes: Vec<PositionalAttr>,
    children: Vec<VNode>,
}

impl Ul {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("accesskey", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("autocapitalize", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("class", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contextmenu", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("dir", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("draggable", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("hidden", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("id", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("itemprop", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("lang", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("slot", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("spellcheck", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("style", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("tabindex", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("translate", value));
        self
    }

    pub fn child(mut self, element: impl Into<VTag>) -> Self {
        self.children.push(VNode::from(element.into()));
        self
    }

    pub fn component<C: Component>(mut self, props: C::Properties) -> Self {
        self.children.push(VNode::from(VComp::new::<C>(
            props,
            NodeRef::default(),
            None,
        )));
        self
    }

    pub fn text(mut self, text: impl Into<AttrValue>) -> Self {
        self.children.push(VNode::from(VText::new(text)));
        self
    }
}

impl Into<VTag> for Ul {
    fn into(self) -> VTag {
        let mut vtag = VTag::new("ul");
        vtag.attributes = Attributes::Vec(self.attributes);
        vtag.add_children(self.children.into_iter());
        vtag
    }
}

impl Into<VNode> for Ul {
    fn into(self) -> VNode {
        let vtag: VTag = self.into();
        VNode::from(vtag)
    }
}

pub struct A {
    attributes: Vec<PositionalAttr>,
    children: Vec<VNode>,
}

impl A {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("accesskey", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("autocapitalize", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("class", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contextmenu", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("dir", value));
        self
    }

    pub fn download(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("download", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("draggable", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("hidden", value));
        self
    }

    pub fn href(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("href", value));
        self
    }

    pub fn hreflang(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("hreflang", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("id", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("itemprop", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("lang", value));
        self
    }

    pub fn media(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("media", value));
        self
    }

    pub fn ping(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("ping", value));
        self
    }

    pub fn referrerpolicy(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("referrerpolicy", value));
        self
    }

    pub fn rel(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("rel", value));
        self
    }

    pub fn shape(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("shape", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("slot", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("spellcheck", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("style", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("tabindex", value));
        self
    }

    pub fn target(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("target", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("translate", value));
        self
    }

    pub fn child(mut self, element: impl Into<VTag>) -> Self {
        self.children.push(VNode::from(element.into()));
        self
    }

    pub fn component<C: Component>(mut self, props: C::Properties) -> Self {
        self.children.push(VNode::from(VComp::new::<C>(
            props,
            NodeRef::default(),
            None,
        )));
        self
    }

    pub fn text(mut self, text: impl Into<AttrValue>) -> Self {
        self.children.push(VNode::from(VText::new(text)));
        self
    }
}

impl Into<VTag> for A {
    fn into(self) -> VTag {
        let mut vtag = VTag::new("a");
        vtag.attributes = Attributes::Vec(self.attributes);
        vtag.add_children(self.children.into_iter());
        vtag
    }
}

impl Into<VNode> for A {
    fn into(self) -> VNode {
        let vtag: VTag = self.into();
        VNode::from(vtag)
    }
}

pub struct Abbr {
    attributes: Vec<PositionalAttr>,
    children: Vec<VNode>,
}

impl Abbr {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("accesskey", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("autocapitalize", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("class", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contextmenu", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("dir", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("draggable", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("hidden", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("id", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("itemprop", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("lang", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("slot", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("spellcheck", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("style", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("tabindex", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("translate", value));
        self
    }

    pub fn child(mut self, element: impl Into<VTag>) -> Self {
        self.children.push(VNode::from(element.into()));
        self
    }

    pub fn component<C: Component>(mut self, props: C::Properties) -> Self {
        self.children.push(VNode::from(VComp::new::<C>(
            props,
            NodeRef::default(),
            None,
        )));
        self
    }

    pub fn text(mut self, text: impl Into<AttrValue>) -> Self {
        self.children.push(VNode::from(VText::new(text)));
        self
    }
}

impl Into<VTag> for Abbr {
    fn into(self) -> VTag {
        let mut vtag = VTag::new("abbr");
        vtag.attributes = Attributes::Vec(self.attributes);
        vtag.add_children(self.children.into_iter());
        vtag
    }
}

impl Into<VNode> for Abbr {
    fn into(self) -> VNode {
        let vtag: VTag = self.into();
        VNode::from(vtag)
    }
}

pub struct B {
    attributes: Vec<PositionalAttr>,
    children: Vec<VNode>,
}

impl B {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("accesskey", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("autocapitalize", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("class", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contextmenu", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("dir", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("draggable", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("hidden", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("id", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("itemprop", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("lang", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("slot", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("spellcheck", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("style", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("tabindex", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("translate", value));
        self
    }

    pub fn child(mut self, element: impl Into<VTag>) -> Self {
        self.children.push(VNode::from(element.into()));
        self
    }

    pub fn component<C: Component>(mut self, props: C::Properties) -> Self {
        self.children.push(VNode::from(VComp::new::<C>(
            props,
            NodeRef::default(),
            None,
        )));
        self
    }

    pub fn text(mut self, text: impl Into<AttrValue>) -> Self {
        self.children.push(VNode::from(VText::new(text)));
        self
    }
}

impl Into<VTag> for B {
    fn into(self) -> VTag {
        let mut vtag = VTag::new("b");
        vtag.attributes = Attributes::Vec(self.attributes);
        vtag.add_children(self.children.into_iter());
        vtag
    }
}

impl Into<VNode> for B {
    fn into(self) -> VNode {
        let vtag: VTag = self.into();
        VNode::from(vtag)
    }
}

pub struct Bdi {
    attributes: Vec<PositionalAttr>,
    children: Vec<VNode>,
}

impl Bdi {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("accesskey", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("autocapitalize", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("class", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contextmenu", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("dir", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("draggable", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("hidden", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("id", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("itemprop", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("lang", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("slot", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("spellcheck", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("style", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("tabindex", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("translate", value));
        self
    }

    pub fn child(mut self, element: impl Into<VTag>) -> Self {
        self.children.push(VNode::from(element.into()));
        self
    }

    pub fn component<C: Component>(mut self, props: C::Properties) -> Self {
        self.children.push(VNode::from(VComp::new::<C>(
            props,
            NodeRef::default(),
            None,
        )));
        self
    }

    pub fn text(mut self, text: impl Into<AttrValue>) -> Self {
        self.children.push(VNode::from(VText::new(text)));
        self
    }
}

impl Into<VTag> for Bdi {
    fn into(self) -> VTag {
        let mut vtag = VTag::new("bdi");
        vtag.attributes = Attributes::Vec(self.attributes);
        vtag.add_children(self.children.into_iter());
        vtag
    }
}

impl Into<VNode> for Bdi {
    fn into(self) -> VNode {
        let vtag: VTag = self.into();
        VNode::from(vtag)
    }
}

pub struct Bdo {
    attributes: Vec<PositionalAttr>,
    children: Vec<VNode>,
}

impl Bdo {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("accesskey", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("autocapitalize", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("class", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contextmenu", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("dir", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("draggable", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("hidden", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("id", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("itemprop", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("lang", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("slot", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("spellcheck", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("style", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("tabindex", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("translate", value));
        self
    }

    pub fn child(mut self, element: impl Into<VTag>) -> Self {
        self.children.push(VNode::from(element.into()));
        self
    }

    pub fn component<C: Component>(mut self, props: C::Properties) -> Self {
        self.children.push(VNode::from(VComp::new::<C>(
            props,
            NodeRef::default(),
            None,
        )));
        self
    }

    pub fn text(mut self, text: impl Into<AttrValue>) -> Self {
        self.children.push(VNode::from(VText::new(text)));
        self
    }
}

impl Into<VTag> for Bdo {
    fn into(self) -> VTag {
        let mut vtag = VTag::new("bdo");
        vtag.attributes = Attributes::Vec(self.attributes);
        vtag.add_children(self.children.into_iter());
        vtag
    }
}

impl Into<VNode> for Bdo {
    fn into(self) -> VNode {
        let vtag: VTag = self.into();
        VNode::from(vtag)
    }
}

pub struct Br {
    attributes: Vec<PositionalAttr>,
    children: Vec<VNode>,
}

impl Br {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("accesskey", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("autocapitalize", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("class", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contextmenu", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("dir", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("draggable", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("hidden", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("id", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("itemprop", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("lang", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("slot", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("spellcheck", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("style", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("tabindex", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("translate", value));
        self
    }

    pub fn child(mut self, element: impl Into<VTag>) -> Self {
        self.children.push(VNode::from(element.into()));
        self
    }

    pub fn component<C: Component>(mut self, props: C::Properties) -> Self {
        self.children.push(VNode::from(VComp::new::<C>(
            props,
            NodeRef::default(),
            None,
        )));
        self
    }

    pub fn text(mut self, text: impl Into<AttrValue>) -> Self {
        self.children.push(VNode::from(VText::new(text)));
        self
    }
}

impl Into<VTag> for Br {
    fn into(self) -> VTag {
        let mut vtag = VTag::new("br");
        vtag.attributes = Attributes::Vec(self.attributes);
        vtag.add_children(self.children.into_iter());
        vtag
    }
}

impl Into<VNode> for Br {
    fn into(self) -> VNode {
        let vtag: VTag = self.into();
        VNode::from(vtag)
    }
}

pub struct Cite {
    attributes: Vec<PositionalAttr>,
    children: Vec<VNode>,
}

impl Cite {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("accesskey", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("autocapitalize", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("class", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contextmenu", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("dir", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("draggable", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("hidden", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("id", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("itemprop", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("lang", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("slot", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("spellcheck", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("style", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("tabindex", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("translate", value));
        self
    }

    pub fn child(mut self, element: impl Into<VTag>) -> Self {
        self.children.push(VNode::from(element.into()));
        self
    }

    pub fn component<C: Component>(mut self, props: C::Properties) -> Self {
        self.children.push(VNode::from(VComp::new::<C>(
            props,
            NodeRef::default(),
            None,
        )));
        self
    }

    pub fn text(mut self, text: impl Into<AttrValue>) -> Self {
        self.children.push(VNode::from(VText::new(text)));
        self
    }
}

impl Into<VTag> for Cite {
    fn into(self) -> VTag {
        let mut vtag = VTag::new("cite");
        vtag.attributes = Attributes::Vec(self.attributes);
        vtag.add_children(self.children.into_iter());
        vtag
    }
}

impl Into<VNode> for Cite {
    fn into(self) -> VNode {
        let vtag: VTag = self.into();
        VNode::from(vtag)
    }
}

pub struct Code {
    attributes: Vec<PositionalAttr>,
    children: Vec<VNode>,
}

impl Code {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("accesskey", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("autocapitalize", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("class", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contextmenu", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("dir", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("draggable", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("hidden", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("id", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("itemprop", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("lang", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("slot", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("spellcheck", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("style", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("tabindex", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("translate", value));
        self
    }

    pub fn child(mut self, element: impl Into<VTag>) -> Self {
        self.children.push(VNode::from(element.into()));
        self
    }

    pub fn component<C: Component>(mut self, props: C::Properties) -> Self {
        self.children.push(VNode::from(VComp::new::<C>(
            props,
            NodeRef::default(),
            None,
        )));
        self
    }

    pub fn text(mut self, text: impl Into<AttrValue>) -> Self {
        self.children.push(VNode::from(VText::new(text)));
        self
    }
}

impl Into<VTag> for Code {
    fn into(self) -> VTag {
        let mut vtag = VTag::new("code");
        vtag.attributes = Attributes::Vec(self.attributes);
        vtag.add_children(self.children.into_iter());
        vtag
    }
}

impl Into<VNode> for Code {
    fn into(self) -> VNode {
        let vtag: VTag = self.into();
        VNode::from(vtag)
    }
}

pub struct Data {
    attributes: Vec<PositionalAttr>,
    children: Vec<VNode>,
}

impl Data {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("accesskey", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("autocapitalize", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("class", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contextmenu", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("dir", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("draggable", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("hidden", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("id", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("itemprop", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("lang", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("slot", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("spellcheck", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("style", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("tabindex", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("translate", value));
        self
    }

    pub fn value(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("value", value));
        self
    }

    pub fn child(mut self, element: impl Into<VTag>) -> Self {
        self.children.push(VNode::from(element.into()));
        self
    }

    pub fn component<C: Component>(mut self, props: C::Properties) -> Self {
        self.children.push(VNode::from(VComp::new::<C>(
            props,
            NodeRef::default(),
            None,
        )));
        self
    }

    pub fn text(mut self, text: impl Into<AttrValue>) -> Self {
        self.children.push(VNode::from(VText::new(text)));
        self
    }
}

impl Into<VTag> for Data {
    fn into(self) -> VTag {
        let mut vtag = VTag::new("data");
        vtag.attributes = Attributes::Vec(self.attributes);
        vtag.add_children(self.children.into_iter());
        vtag
    }
}

impl Into<VNode> for Data {
    fn into(self) -> VNode {
        let vtag: VTag = self.into();
        VNode::from(vtag)
    }
}

pub struct Dfn {
    attributes: Vec<PositionalAttr>,
    children: Vec<VNode>,
}

impl Dfn {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("accesskey", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("autocapitalize", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("class", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contextmenu", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("dir", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("draggable", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("hidden", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("id", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("itemprop", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("lang", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("slot", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("spellcheck", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("style", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("tabindex", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("translate", value));
        self
    }

    pub fn child(mut self, element: impl Into<VTag>) -> Self {
        self.children.push(VNode::from(element.into()));
        self
    }

    pub fn component<C: Component>(mut self, props: C::Properties) -> Self {
        self.children.push(VNode::from(VComp::new::<C>(
            props,
            NodeRef::default(),
            None,
        )));
        self
    }

    pub fn text(mut self, text: impl Into<AttrValue>) -> Self {
        self.children.push(VNode::from(VText::new(text)));
        self
    }
}

impl Into<VTag> for Dfn {
    fn into(self) -> VTag {
        let mut vtag = VTag::new("dfn");
        vtag.attributes = Attributes::Vec(self.attributes);
        vtag.add_children(self.children.into_iter());
        vtag
    }
}

impl Into<VNode> for Dfn {
    fn into(self) -> VNode {
        let vtag: VTag = self.into();
        VNode::from(vtag)
    }
}

pub struct Em {
    attributes: Vec<PositionalAttr>,
    children: Vec<VNode>,
}

impl Em {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("accesskey", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("autocapitalize", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("class", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contextmenu", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("dir", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("draggable", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("hidden", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("id", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("itemprop", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("lang", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("slot", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("spellcheck", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("style", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("tabindex", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("translate", value));
        self
    }

    pub fn child(mut self, element: impl Into<VTag>) -> Self {
        self.children.push(VNode::from(element.into()));
        self
    }

    pub fn component<C: Component>(mut self, props: C::Properties) -> Self {
        self.children.push(VNode::from(VComp::new::<C>(
            props,
            NodeRef::default(),
            None,
        )));
        self
    }

    pub fn text(mut self, text: impl Into<AttrValue>) -> Self {
        self.children.push(VNode::from(VText::new(text)));
        self
    }
}

impl Into<VTag> for Em {
    fn into(self) -> VTag {
        let mut vtag = VTag::new("em");
        vtag.attributes = Attributes::Vec(self.attributes);
        vtag.add_children(self.children.into_iter());
        vtag
    }
}

impl Into<VNode> for Em {
    fn into(self) -> VNode {
        let vtag: VTag = self.into();
        VNode::from(vtag)
    }
}

pub struct I {
    attributes: Vec<PositionalAttr>,
    children: Vec<VNode>,
}

impl I {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("accesskey", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("autocapitalize", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("class", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contextmenu", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("dir", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("draggable", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("hidden", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("id", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("itemprop", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("lang", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("slot", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("spellcheck", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("style", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("tabindex", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("translate", value));
        self
    }

    pub fn child(mut self, element: impl Into<VTag>) -> Self {
        self.children.push(VNode::from(element.into()));
        self
    }

    pub fn component<C: Component>(mut self, props: C::Properties) -> Self {
        self.children.push(VNode::from(VComp::new::<C>(
            props,
            NodeRef::default(),
            None,
        )));
        self
    }

    pub fn text(mut self, text: impl Into<AttrValue>) -> Self {
        self.children.push(VNode::from(VText::new(text)));
        self
    }
}

impl Into<VTag> for I {
    fn into(self) -> VTag {
        let mut vtag = VTag::new("i");
        vtag.attributes = Attributes::Vec(self.attributes);
        vtag.add_children(self.children.into_iter());
        vtag
    }
}

impl Into<VNode> for I {
    fn into(self) -> VNode {
        let vtag: VTag = self.into();
        VNode::from(vtag)
    }
}

pub struct Kbd {
    attributes: Vec<PositionalAttr>,
    children: Vec<VNode>,
}

impl Kbd {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("accesskey", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("autocapitalize", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("class", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contextmenu", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("dir", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("draggable", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("hidden", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("id", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("itemprop", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("lang", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("slot", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("spellcheck", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("style", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("tabindex", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("translate", value));
        self
    }

    pub fn child(mut self, element: impl Into<VTag>) -> Self {
        self.children.push(VNode::from(element.into()));
        self
    }

    pub fn component<C: Component>(mut self, props: C::Properties) -> Self {
        self.children.push(VNode::from(VComp::new::<C>(
            props,
            NodeRef::default(),
            None,
        )));
        self
    }

    pub fn text(mut self, text: impl Into<AttrValue>) -> Self {
        self.children.push(VNode::from(VText::new(text)));
        self
    }
}

impl Into<VTag> for Kbd {
    fn into(self) -> VTag {
        let mut vtag = VTag::new("kbd");
        vtag.attributes = Attributes::Vec(self.attributes);
        vtag.add_children(self.children.into_iter());
        vtag
    }
}

impl Into<VNode> for Kbd {
    fn into(self) -> VNode {
        let vtag: VTag = self.into();
        VNode::from(vtag)
    }
}

pub struct Mark {
    attributes: Vec<PositionalAttr>,
    children: Vec<VNode>,
}

impl Mark {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("accesskey", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("autocapitalize", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("class", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contextmenu", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("dir", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("draggable", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("hidden", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("id", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("itemprop", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("lang", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("slot", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("spellcheck", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("style", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("tabindex", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("translate", value));
        self
    }

    pub fn child(mut self, element: impl Into<VTag>) -> Self {
        self.children.push(VNode::from(element.into()));
        self
    }

    pub fn component<C: Component>(mut self, props: C::Properties) -> Self {
        self.children.push(VNode::from(VComp::new::<C>(
            props,
            NodeRef::default(),
            None,
        )));
        self
    }

    pub fn text(mut self, text: impl Into<AttrValue>) -> Self {
        self.children.push(VNode::from(VText::new(text)));
        self
    }
}

impl Into<VTag> for Mark {
    fn into(self) -> VTag {
        let mut vtag = VTag::new("mark");
        vtag.attributes = Attributes::Vec(self.attributes);
        vtag.add_children(self.children.into_iter());
        vtag
    }
}

impl Into<VNode> for Mark {
    fn into(self) -> VNode {
        let vtag: VTag = self.into();
        VNode::from(vtag)
    }
}

pub struct Q {
    attributes: Vec<PositionalAttr>,
    children: Vec<VNode>,
}

impl Q {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("accesskey", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("autocapitalize", value));
        self
    }

    pub fn cite(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("cite", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("class", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contextmenu", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("dir", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("draggable", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("hidden", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("id", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("itemprop", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("lang", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("slot", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("spellcheck", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("style", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("tabindex", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("translate", value));
        self
    }

    pub fn child(mut self, element: impl Into<VTag>) -> Self {
        self.children.push(VNode::from(element.into()));
        self
    }

    pub fn component<C: Component>(mut self, props: C::Properties) -> Self {
        self.children.push(VNode::from(VComp::new::<C>(
            props,
            NodeRef::default(),
            None,
        )));
        self
    }

    pub fn text(mut self, text: impl Into<AttrValue>) -> Self {
        self.children.push(VNode::from(VText::new(text)));
        self
    }
}

impl Into<VTag> for Q {
    fn into(self) -> VTag {
        let mut vtag = VTag::new("q");
        vtag.attributes = Attributes::Vec(self.attributes);
        vtag.add_children(self.children.into_iter());
        vtag
    }
}

impl Into<VNode> for Q {
    fn into(self) -> VNode {
        let vtag: VTag = self.into();
        VNode::from(vtag)
    }
}

pub struct Rp {
    attributes: Vec<PositionalAttr>,
    children: Vec<VNode>,
}

impl Rp {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("accesskey", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("autocapitalize", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("class", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contextmenu", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("dir", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("draggable", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("hidden", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("id", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("itemprop", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("lang", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("slot", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("spellcheck", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("style", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("tabindex", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("translate", value));
        self
    }

    pub fn child(mut self, element: impl Into<VTag>) -> Self {
        self.children.push(VNode::from(element.into()));
        self
    }

    pub fn component<C: Component>(mut self, props: C::Properties) -> Self {
        self.children.push(VNode::from(VComp::new::<C>(
            props,
            NodeRef::default(),
            None,
        )));
        self
    }

    pub fn text(mut self, text: impl Into<AttrValue>) -> Self {
        self.children.push(VNode::from(VText::new(text)));
        self
    }
}

impl Into<VTag> for Rp {
    fn into(self) -> VTag {
        let mut vtag = VTag::new("rp");
        vtag.attributes = Attributes::Vec(self.attributes);
        vtag.add_children(self.children.into_iter());
        vtag
    }
}

impl Into<VNode> for Rp {
    fn into(self) -> VNode {
        let vtag: VTag = self.into();
        VNode::from(vtag)
    }
}

pub struct Rt {
    attributes: Vec<PositionalAttr>,
    children: Vec<VNode>,
}

impl Rt {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("accesskey", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("autocapitalize", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("class", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contextmenu", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("dir", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("draggable", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("hidden", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("id", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("itemprop", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("lang", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("slot", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("spellcheck", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("style", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("tabindex", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("translate", value));
        self
    }

    pub fn child(mut self, element: impl Into<VTag>) -> Self {
        self.children.push(VNode::from(element.into()));
        self
    }

    pub fn component<C: Component>(mut self, props: C::Properties) -> Self {
        self.children.push(VNode::from(VComp::new::<C>(
            props,
            NodeRef::default(),
            None,
        )));
        self
    }

    pub fn text(mut self, text: impl Into<AttrValue>) -> Self {
        self.children.push(VNode::from(VText::new(text)));
        self
    }
}

impl Into<VTag> for Rt {
    fn into(self) -> VTag {
        let mut vtag = VTag::new("rt");
        vtag.attributes = Attributes::Vec(self.attributes);
        vtag.add_children(self.children.into_iter());
        vtag
    }
}

impl Into<VNode> for Rt {
    fn into(self) -> VNode {
        let vtag: VTag = self.into();
        VNode::from(vtag)
    }
}

pub struct Ruby {
    attributes: Vec<PositionalAttr>,
    children: Vec<VNode>,
}

impl Ruby {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("accesskey", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("autocapitalize", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("class", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contextmenu", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("dir", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("draggable", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("hidden", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("id", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("itemprop", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("lang", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("slot", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("spellcheck", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("style", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("tabindex", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("translate", value));
        self
    }

    pub fn child(mut self, element: impl Into<VTag>) -> Self {
        self.children.push(VNode::from(element.into()));
        self
    }

    pub fn component<C: Component>(mut self, props: C::Properties) -> Self {
        self.children.push(VNode::from(VComp::new::<C>(
            props,
            NodeRef::default(),
            None,
        )));
        self
    }

    pub fn text(mut self, text: impl Into<AttrValue>) -> Self {
        self.children.push(VNode::from(VText::new(text)));
        self
    }
}

impl Into<VTag> for Ruby {
    fn into(self) -> VTag {
        let mut vtag = VTag::new("ruby");
        vtag.attributes = Attributes::Vec(self.attributes);
        vtag.add_children(self.children.into_iter());
        vtag
    }
}

impl Into<VNode> for Ruby {
    fn into(self) -> VNode {
        let vtag: VTag = self.into();
        VNode::from(vtag)
    }
}

pub struct S {
    attributes: Vec<PositionalAttr>,
    children: Vec<VNode>,
}

impl S {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("accesskey", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("autocapitalize", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("class", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contextmenu", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("dir", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("draggable", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("hidden", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("id", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("itemprop", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("lang", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("slot", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("spellcheck", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("style", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("tabindex", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("translate", value));
        self
    }

    pub fn child(mut self, element: impl Into<VTag>) -> Self {
        self.children.push(VNode::from(element.into()));
        self
    }

    pub fn component<C: Component>(mut self, props: C::Properties) -> Self {
        self.children.push(VNode::from(VComp::new::<C>(
            props,
            NodeRef::default(),
            None,
        )));
        self
    }

    pub fn text(mut self, text: impl Into<AttrValue>) -> Self {
        self.children.push(VNode::from(VText::new(text)));
        self
    }
}

impl Into<VTag> for S {
    fn into(self) -> VTag {
        let mut vtag = VTag::new("s");
        vtag.attributes = Attributes::Vec(self.attributes);
        vtag.add_children(self.children.into_iter());
        vtag
    }
}

impl Into<VNode> for S {
    fn into(self) -> VNode {
        let vtag: VTag = self.into();
        VNode::from(vtag)
    }
}

pub struct Samp {
    attributes: Vec<PositionalAttr>,
    children: Vec<VNode>,
}

impl Samp {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("accesskey", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("autocapitalize", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("class", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contextmenu", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("dir", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("draggable", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("hidden", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("id", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("itemprop", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("lang", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("slot", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("spellcheck", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("style", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("tabindex", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("translate", value));
        self
    }

    pub fn child(mut self, element: impl Into<VTag>) -> Self {
        self.children.push(VNode::from(element.into()));
        self
    }

    pub fn component<C: Component>(mut self, props: C::Properties) -> Self {
        self.children.push(VNode::from(VComp::new::<C>(
            props,
            NodeRef::default(),
            None,
        )));
        self
    }

    pub fn text(mut self, text: impl Into<AttrValue>) -> Self {
        self.children.push(VNode::from(VText::new(text)));
        self
    }
}

impl Into<VTag> for Samp {
    fn into(self) -> VTag {
        let mut vtag = VTag::new("samp");
        vtag.attributes = Attributes::Vec(self.attributes);
        vtag.add_children(self.children.into_iter());
        vtag
    }
}

impl Into<VNode> for Samp {
    fn into(self) -> VNode {
        let vtag: VTag = self.into();
        VNode::from(vtag)
    }
}

pub struct Small {
    attributes: Vec<PositionalAttr>,
    children: Vec<VNode>,
}

impl Small {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("accesskey", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("autocapitalize", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("class", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contextmenu", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("dir", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("draggable", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("hidden", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("id", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("itemprop", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("lang", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("slot", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("spellcheck", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("style", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("tabindex", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("translate", value));
        self
    }

    pub fn child(mut self, element: impl Into<VTag>) -> Self {
        self.children.push(VNode::from(element.into()));
        self
    }

    pub fn component<C: Component>(mut self, props: C::Properties) -> Self {
        self.children.push(VNode::from(VComp::new::<C>(
            props,
            NodeRef::default(),
            None,
        )));
        self
    }

    pub fn text(mut self, text: impl Into<AttrValue>) -> Self {
        self.children.push(VNode::from(VText::new(text)));
        self
    }
}

impl Into<VTag> for Small {
    fn into(self) -> VTag {
        let mut vtag = VTag::new("small");
        vtag.attributes = Attributes::Vec(self.attributes);
        vtag.add_children(self.children.into_iter());
        vtag
    }
}

impl Into<VNode> for Small {
    fn into(self) -> VNode {
        let vtag: VTag = self.into();
        VNode::from(vtag)
    }
}

pub struct Span {
    attributes: Vec<PositionalAttr>,
    children: Vec<VNode>,
}

impl Span {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("accesskey", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("autocapitalize", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("class", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contextmenu", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("dir", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("draggable", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("hidden", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("id", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("itemprop", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("lang", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("slot", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("spellcheck", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("style", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("tabindex", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("translate", value));
        self
    }

    pub fn child(mut self, element: impl Into<VTag>) -> Self {
        self.children.push(VNode::from(element.into()));
        self
    }

    pub fn component<C: Component>(mut self, props: C::Properties) -> Self {
        self.children.push(VNode::from(VComp::new::<C>(
            props,
            NodeRef::default(),
            None,
        )));
        self
    }

    pub fn text(mut self, text: impl Into<AttrValue>) -> Self {
        self.children.push(VNode::from(VText::new(text)));
        self
    }
}

impl Into<VTag> for Span {
    fn into(self) -> VTag {
        let mut vtag = VTag::new("span");
        vtag.attributes = Attributes::Vec(self.attributes);
        vtag.add_children(self.children.into_iter());
        vtag
    }
}

impl Into<VNode> for Span {
    fn into(self) -> VNode {
        let vtag: VTag = self.into();
        VNode::from(vtag)
    }
}

pub struct Strong {
    attributes: Vec<PositionalAttr>,
    children: Vec<VNode>,
}

impl Strong {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("accesskey", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("autocapitalize", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("class", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contextmenu", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("dir", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("draggable", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("hidden", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("id", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("itemprop", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("lang", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("slot", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("spellcheck", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("style", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("tabindex", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("translate", value));
        self
    }

    pub fn child(mut self, element: impl Into<VTag>) -> Self {
        self.children.push(VNode::from(element.into()));
        self
    }

    pub fn component<C: Component>(mut self, props: C::Properties) -> Self {
        self.children.push(VNode::from(VComp::new::<C>(
            props,
            NodeRef::default(),
            None,
        )));
        self
    }

    pub fn text(mut self, text: impl Into<AttrValue>) -> Self {
        self.children.push(VNode::from(VText::new(text)));
        self
    }
}

impl Into<VTag> for Strong {
    fn into(self) -> VTag {
        let mut vtag = VTag::new("strong");
        vtag.attributes = Attributes::Vec(self.attributes);
        vtag.add_children(self.children.into_iter());
        vtag
    }
}

impl Into<VNode> for Strong {
    fn into(self) -> VNode {
        let vtag: VTag = self.into();
        VNode::from(vtag)
    }
}

pub struct Sub {
    attributes: Vec<PositionalAttr>,
    children: Vec<VNode>,
}

impl Sub {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("accesskey", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("autocapitalize", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("class", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contextmenu", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("dir", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("draggable", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("hidden", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("id", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("itemprop", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("lang", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("slot", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("spellcheck", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("style", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("tabindex", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("translate", value));
        self
    }

    pub fn child(mut self, element: impl Into<VTag>) -> Self {
        self.children.push(VNode::from(element.into()));
        self
    }

    pub fn component<C: Component>(mut self, props: C::Properties) -> Self {
        self.children.push(VNode::from(VComp::new::<C>(
            props,
            NodeRef::default(),
            None,
        )));
        self
    }

    pub fn text(mut self, text: impl Into<AttrValue>) -> Self {
        self.children.push(VNode::from(VText::new(text)));
        self
    }
}

impl Into<VTag> for Sub {
    fn into(self) -> VTag {
        let mut vtag = VTag::new("sub");
        vtag.attributes = Attributes::Vec(self.attributes);
        vtag.add_children(self.children.into_iter());
        vtag
    }
}

impl Into<VNode> for Sub {
    fn into(self) -> VNode {
        let vtag: VTag = self.into();
        VNode::from(vtag)
    }
}

pub struct Sup {
    attributes: Vec<PositionalAttr>,
    children: Vec<VNode>,
}

impl Sup {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("accesskey", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("autocapitalize", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("class", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contextmenu", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("dir", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("draggable", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("hidden", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("id", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("itemprop", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("lang", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("slot", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("spellcheck", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("style", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("tabindex", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("translate", value));
        self
    }

    pub fn child(mut self, element: impl Into<VTag>) -> Self {
        self.children.push(VNode::from(element.into()));
        self
    }

    pub fn component<C: Component>(mut self, props: C::Properties) -> Self {
        self.children.push(VNode::from(VComp::new::<C>(
            props,
            NodeRef::default(),
            None,
        )));
        self
    }

    pub fn text(mut self, text: impl Into<AttrValue>) -> Self {
        self.children.push(VNode::from(VText::new(text)));
        self
    }
}

impl Into<VTag> for Sup {
    fn into(self) -> VTag {
        let mut vtag = VTag::new("sup");
        vtag.attributes = Attributes::Vec(self.attributes);
        vtag.add_children(self.children.into_iter());
        vtag
    }
}

impl Into<VNode> for Sup {
    fn into(self) -> VNode {
        let vtag: VTag = self.into();
        VNode::from(vtag)
    }
}

pub struct Time {
    attributes: Vec<PositionalAttr>,
    children: Vec<VNode>,
}

impl Time {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("accesskey", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("autocapitalize", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("class", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contextmenu", value));
        self
    }

    pub fn datetime(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("datetime", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("dir", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("draggable", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("hidden", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("id", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("itemprop", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("lang", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("slot", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("spellcheck", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("style", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("tabindex", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("translate", value));
        self
    }

    pub fn child(mut self, element: impl Into<VTag>) -> Self {
        self.children.push(VNode::from(element.into()));
        self
    }

    pub fn component<C: Component>(mut self, props: C::Properties) -> Self {
        self.children.push(VNode::from(VComp::new::<C>(
            props,
            NodeRef::default(),
            None,
        )));
        self
    }

    pub fn text(mut self, text: impl Into<AttrValue>) -> Self {
        self.children.push(VNode::from(VText::new(text)));
        self
    }
}

impl Into<VTag> for Time {
    fn into(self) -> VTag {
        let mut vtag = VTag::new("time");
        vtag.attributes = Attributes::Vec(self.attributes);
        vtag.add_children(self.children.into_iter());
        vtag
    }
}

impl Into<VNode> for Time {
    fn into(self) -> VNode {
        let vtag: VTag = self.into();
        VNode::from(vtag)
    }
}

pub struct U {
    attributes: Vec<PositionalAttr>,
    children: Vec<VNode>,
}

impl U {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("accesskey", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("autocapitalize", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("class", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contextmenu", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("dir", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("draggable", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("hidden", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("id", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("itemprop", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("lang", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("slot", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("spellcheck", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("style", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("tabindex", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("translate", value));
        self
    }

    pub fn child(mut self, element: impl Into<VTag>) -> Self {
        self.children.push(VNode::from(element.into()));
        self
    }

    pub fn component<C: Component>(mut self, props: C::Properties) -> Self {
        self.children.push(VNode::from(VComp::new::<C>(
            props,
            NodeRef::default(),
            None,
        )));
        self
    }

    pub fn text(mut self, text: impl Into<AttrValue>) -> Self {
        self.children.push(VNode::from(VText::new(text)));
        self
    }
}

impl Into<VTag> for U {
    fn into(self) -> VTag {
        let mut vtag = VTag::new("u");
        vtag.attributes = Attributes::Vec(self.attributes);
        vtag.add_children(self.children.into_iter());
        vtag
    }
}

impl Into<VNode> for U {
    fn into(self) -> VNode {
        let vtag: VTag = self.into();
        VNode::from(vtag)
    }
}

pub struct Var {
    attributes: Vec<PositionalAttr>,
    children: Vec<VNode>,
}

impl Var {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("accesskey", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("autocapitalize", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("class", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contextmenu", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("dir", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("draggable", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("hidden", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("id", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("itemprop", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("lang", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("slot", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("spellcheck", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("style", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("tabindex", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("translate", value));
        self
    }

    pub fn child(mut self, element: impl Into<VTag>) -> Self {
        self.children.push(VNode::from(element.into()));
        self
    }

    pub fn component<C: Component>(mut self, props: C::Properties) -> Self {
        self.children.push(VNode::from(VComp::new::<C>(
            props,
            NodeRef::default(),
            None,
        )));
        self
    }

    pub fn text(mut self, text: impl Into<AttrValue>) -> Self {
        self.children.push(VNode::from(VText::new(text)));
        self
    }
}

impl Into<VTag> for Var {
    fn into(self) -> VTag {
        let mut vtag = VTag::new("var");
        vtag.attributes = Attributes::Vec(self.attributes);
        vtag.add_children(self.children.into_iter());
        vtag
    }
}

impl Into<VNode> for Var {
    fn into(self) -> VNode {
        let vtag: VTag = self.into();
        VNode::from(vtag)
    }
}

pub struct Wbr {
    attributes: Vec<PositionalAttr>,
    children: Vec<VNode>,
}

impl Wbr {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("accesskey", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("autocapitalize", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("class", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contextmenu", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("dir", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("draggable", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("hidden", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("id", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("itemprop", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("lang", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("slot", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("spellcheck", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("style", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("tabindex", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("translate", value));
        self
    }

    pub fn child(mut self, element: impl Into<VTag>) -> Self {
        self.children.push(VNode::from(element.into()));
        self
    }

    pub fn component<C: Component>(mut self, props: C::Properties) -> Self {
        self.children.push(VNode::from(VComp::new::<C>(
            props,
            NodeRef::default(),
            None,
        )));
        self
    }

    pub fn text(mut self, text: impl Into<AttrValue>) -> Self {
        self.children.push(VNode::from(VText::new(text)));
        self
    }
}

impl Into<VTag> for Wbr {
    fn into(self) -> VTag {
        let mut vtag = VTag::new("wbr");
        vtag.attributes = Attributes::Vec(self.attributes);
        vtag.add_children(self.children.into_iter());
        vtag
    }
}

impl Into<VNode> for Wbr {
    fn into(self) -> VNode {
        let vtag: VTag = self.into();
        VNode::from(vtag)
    }
}

pub struct Area {
    attributes: Vec<PositionalAttr>,
    children: Vec<VNode>,
}

impl Area {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("accesskey", value));
        self
    }

    pub fn alt(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("alt", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("autocapitalize", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("class", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contextmenu", value));
        self
    }

    pub fn coords(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("coords", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("dir", value));
        self
    }

    pub fn download(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("download", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("draggable", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("hidden", value));
        self
    }

    pub fn href(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("href", value));
        self
    }

    pub fn hreflang(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("hreflang", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("id", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("itemprop", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("lang", value));
        self
    }

    pub fn media(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("media", value));
        self
    }

    pub fn ping(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("ping", value));
        self
    }

    pub fn referrerpolicy(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("referrerpolicy", value));
        self
    }

    pub fn rel(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("rel", value));
        self
    }

    pub fn shape(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("shape", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("slot", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("spellcheck", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("style", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("tabindex", value));
        self
    }

    pub fn target(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("target", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("translate", value));
        self
    }

    pub fn child(mut self, element: impl Into<VTag>) -> Self {
        self.children.push(VNode::from(element.into()));
        self
    }

    pub fn component<C: Component>(mut self, props: C::Properties) -> Self {
        self.children.push(VNode::from(VComp::new::<C>(
            props,
            NodeRef::default(),
            None,
        )));
        self
    }

    pub fn text(mut self, text: impl Into<AttrValue>) -> Self {
        self.children.push(VNode::from(VText::new(text)));
        self
    }
}

impl Into<VTag> for Area {
    fn into(self) -> VTag {
        let mut vtag = VTag::new("area");
        vtag.attributes = Attributes::Vec(self.attributes);
        vtag.add_children(self.children.into_iter());
        vtag
    }
}

impl Into<VNode> for Area {
    fn into(self) -> VNode {
        let vtag: VTag = self.into();
        VNode::from(vtag)
    }
}

pub struct Audio {
    attributes: Vec<PositionalAttr>,
    children: Vec<VNode>,
}

impl Audio {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("accesskey", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("autocapitalize", value));
        self
    }

    pub fn autoplay(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("autoplay", value));
        self
    }

    pub fn buffered(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("buffered", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("class", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contextmenu", value));
        self
    }

    pub fn controls(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("controls", value));
        self
    }

    pub fn crossorigin(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("crossorigin", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("dir", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("draggable", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("hidden", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("id", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("itemprop", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("lang", value));
        self
    }

    pub fn r#loop(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("loop", value));
        self
    }

    pub fn muted(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("muted", value));
        self
    }

    pub fn preload(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("preload", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("slot", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("spellcheck", value));
        self
    }

    pub fn src(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("src", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("style", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("tabindex", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("translate", value));
        self
    }

    pub fn child(mut self, element: impl Into<VTag>) -> Self {
        self.children.push(VNode::from(element.into()));
        self
    }

    pub fn component<C: Component>(mut self, props: C::Properties) -> Self {
        self.children.push(VNode::from(VComp::new::<C>(
            props,
            NodeRef::default(),
            None,
        )));
        self
    }

    pub fn text(mut self, text: impl Into<AttrValue>) -> Self {
        self.children.push(VNode::from(VText::new(text)));
        self
    }
}

impl Into<VTag> for Audio {
    fn into(self) -> VTag {
        let mut vtag = VTag::new("audio");
        vtag.attributes = Attributes::Vec(self.attributes);
        vtag.add_children(self.children.into_iter());
        vtag
    }
}

impl Into<VNode> for Audio {
    fn into(self) -> VNode {
        let vtag: VTag = self.into();
        VNode::from(vtag)
    }
}

pub struct Img {
    attributes: Vec<PositionalAttr>,
    children: Vec<VNode>,
}

impl Img {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("accesskey", value));
        self
    }

    pub fn align(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("align", value));
        self
    }

    pub fn alt(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("alt", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("autocapitalize", value));
        self
    }

    pub fn border(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("border", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("class", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contextmenu", value));
        self
    }

    pub fn crossorigin(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("crossorigin", value));
        self
    }

    pub fn decoding(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("decoding", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("dir", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("draggable", value));
        self
    }

    pub fn height(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("height", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("hidden", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("id", value));
        self
    }

    pub fn importance(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("importance", value));
        self
    }

    pub fn intrinsicsize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("intrinsicsize", value));
        self
    }

    pub fn ismap(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("ismap", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("itemprop", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("lang", value));
        self
    }

    pub fn loading(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("loading", value));
        self
    }

    pub fn referrerpolicy(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("referrerpolicy", value));
        self
    }

    pub fn sizes(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("sizes", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("slot", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("spellcheck", value));
        self
    }

    pub fn src(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("src", value));
        self
    }

    pub fn srcset(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("srcset", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("style", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("tabindex", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("translate", value));
        self
    }

    pub fn usemap(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("usemap", value));
        self
    }

    pub fn width(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("width", value));
        self
    }

    pub fn child(mut self, element: impl Into<VTag>) -> Self {
        self.children.push(VNode::from(element.into()));
        self
    }

    pub fn component<C: Component>(mut self, props: C::Properties) -> Self {
        self.children.push(VNode::from(VComp::new::<C>(
            props,
            NodeRef::default(),
            None,
        )));
        self
    }

    pub fn text(mut self, text: impl Into<AttrValue>) -> Self {
        self.children.push(VNode::from(VText::new(text)));
        self
    }
}

impl Into<VTag> for Img {
    fn into(self) -> VTag {
        let mut vtag = VTag::new("img");
        vtag.attributes = Attributes::Vec(self.attributes);
        vtag.add_children(self.children.into_iter());
        vtag
    }
}

impl Into<VNode> for Img {
    fn into(self) -> VNode {
        let vtag: VTag = self.into();
        VNode::from(vtag)
    }
}

pub struct Map {
    attributes: Vec<PositionalAttr>,
    children: Vec<VNode>,
}

impl Map {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("accesskey", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("autocapitalize", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("class", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contextmenu", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("dir", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("draggable", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("hidden", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("id", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("itemprop", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("lang", value));
        self
    }

    pub fn name(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("name", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("slot", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("spellcheck", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("style", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("tabindex", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("translate", value));
        self
    }

    pub fn child(mut self, element: impl Into<VTag>) -> Self {
        self.children.push(VNode::from(element.into()));
        self
    }

    pub fn component<C: Component>(mut self, props: C::Properties) -> Self {
        self.children.push(VNode::from(VComp::new::<C>(
            props,
            NodeRef::default(),
            None,
        )));
        self
    }

    pub fn text(mut self, text: impl Into<AttrValue>) -> Self {
        self.children.push(VNode::from(VText::new(text)));
        self
    }
}

impl Into<VTag> for Map {
    fn into(self) -> VTag {
        let mut vtag = VTag::new("map");
        vtag.attributes = Attributes::Vec(self.attributes);
        vtag.add_children(self.children.into_iter());
        vtag
    }
}

impl Into<VNode> for Map {
    fn into(self) -> VNode {
        let vtag: VTag = self.into();
        VNode::from(vtag)
    }
}

pub struct Track {
    attributes: Vec<PositionalAttr>,
    children: Vec<VNode>,
}

impl Track {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("accesskey", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("autocapitalize", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("class", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contextmenu", value));
        self
    }

    pub fn default(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("default", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("dir", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("draggable", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("hidden", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("id", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("itemprop", value));
        self
    }

    pub fn kind(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("kind", value));
        self
    }

    pub fn label(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("label", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("lang", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("slot", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("spellcheck", value));
        self
    }

    pub fn src(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("src", value));
        self
    }

    pub fn srclang(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("srclang", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("style", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("tabindex", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("translate", value));
        self
    }

    pub fn child(mut self, element: impl Into<VTag>) -> Self {
        self.children.push(VNode::from(element.into()));
        self
    }

    pub fn component<C: Component>(mut self, props: C::Properties) -> Self {
        self.children.push(VNode::from(VComp::new::<C>(
            props,
            NodeRef::default(),
            None,
        )));
        self
    }

    pub fn text(mut self, text: impl Into<AttrValue>) -> Self {
        self.children.push(VNode::from(VText::new(text)));
        self
    }
}

impl Into<VTag> for Track {
    fn into(self) -> VTag {
        let mut vtag = VTag::new("track");
        vtag.attributes = Attributes::Vec(self.attributes);
        vtag.add_children(self.children.into_iter());
        vtag
    }
}

impl Into<VNode> for Track {
    fn into(self) -> VNode {
        let vtag: VTag = self.into();
        VNode::from(vtag)
    }
}

pub struct Video {
    attributes: Vec<PositionalAttr>,
    children: Vec<VNode>,
}

impl Video {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("accesskey", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("autocapitalize", value));
        self
    }

    pub fn autoplay(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("autoplay", value));
        self
    }

    pub fn buffered(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("buffered", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("class", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contextmenu", value));
        self
    }

    pub fn controls(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("controls", value));
        self
    }

    pub fn crossorigin(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("crossorigin", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("dir", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("draggable", value));
        self
    }

    pub fn height(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("height", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("hidden", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("id", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("itemprop", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("lang", value));
        self
    }

    pub fn r#loop(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("loop", value));
        self
    }

    pub fn muted(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("muted", value));
        self
    }

    pub fn poster(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("poster", value));
        self
    }

    pub fn preload(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("preload", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("slot", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("spellcheck", value));
        self
    }

    pub fn src(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("src", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("style", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("tabindex", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("translate", value));
        self
    }

    pub fn width(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("width", value));
        self
    }

    pub fn child(mut self, element: impl Into<VTag>) -> Self {
        self.children.push(VNode::from(element.into()));
        self
    }

    pub fn component<C: Component>(mut self, props: C::Properties) -> Self {
        self.children.push(VNode::from(VComp::new::<C>(
            props,
            NodeRef::default(),
            None,
        )));
        self
    }

    pub fn text(mut self, text: impl Into<AttrValue>) -> Self {
        self.children.push(VNode::from(VText::new(text)));
        self
    }
}

impl Into<VTag> for Video {
    fn into(self) -> VTag {
        let mut vtag = VTag::new("video");
        vtag.attributes = Attributes::Vec(self.attributes);
        vtag.add_children(self.children.into_iter());
        vtag
    }
}

impl Into<VNode> for Video {
    fn into(self) -> VNode {
        let vtag: VTag = self.into();
        VNode::from(vtag)
    }
}

pub struct Embed {
    attributes: Vec<PositionalAttr>,
    children: Vec<VNode>,
}

impl Embed {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("accesskey", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("autocapitalize", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("class", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contextmenu", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("dir", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("draggable", value));
        self
    }

    pub fn height(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("height", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("hidden", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("id", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("itemprop", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("lang", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("slot", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("spellcheck", value));
        self
    }

    pub fn src(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("src", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("style", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("tabindex", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("translate", value));
        self
    }

    pub fn r#type(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("type", value));
        self
    }

    pub fn width(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("width", value));
        self
    }

    pub fn child(mut self, element: impl Into<VTag>) -> Self {
        self.children.push(VNode::from(element.into()));
        self
    }

    pub fn component<C: Component>(mut self, props: C::Properties) -> Self {
        self.children.push(VNode::from(VComp::new::<C>(
            props,
            NodeRef::default(),
            None,
        )));
        self
    }

    pub fn text(mut self, text: impl Into<AttrValue>) -> Self {
        self.children.push(VNode::from(VText::new(text)));
        self
    }
}

impl Into<VTag> for Embed {
    fn into(self) -> VTag {
        let mut vtag = VTag::new("embed");
        vtag.attributes = Attributes::Vec(self.attributes);
        vtag.add_children(self.children.into_iter());
        vtag
    }
}

impl Into<VNode> for Embed {
    fn into(self) -> VNode {
        let vtag: VTag = self.into();
        VNode::from(vtag)
    }
}

pub struct Iframe {
    attributes: Vec<PositionalAttr>,
    children: Vec<VNode>,
}

impl Iframe {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("accesskey", value));
        self
    }

    pub fn align(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("align", value));
        self
    }

    pub fn allow(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("allow", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("autocapitalize", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("class", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contextmenu", value));
        self
    }

    pub fn csp(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("csp", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("dir", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("draggable", value));
        self
    }

    pub fn height(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("height", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("hidden", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("id", value));
        self
    }

    pub fn importance(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("importance", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("itemprop", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("lang", value));
        self
    }

    pub fn loading(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("loading", value));
        self
    }

    pub fn name(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("name", value));
        self
    }

    pub fn referrerpolicy(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("referrerpolicy", value));
        self
    }

    pub fn sandbox(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("sandbox", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("slot", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("spellcheck", value));
        self
    }

    pub fn src(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("src", value));
        self
    }

    pub fn srcdoc(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("srcdoc", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("style", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("tabindex", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("translate", value));
        self
    }

    pub fn width(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("width", value));
        self
    }

    pub fn child(mut self, element: impl Into<VTag>) -> Self {
        self.children.push(VNode::from(element.into()));
        self
    }

    pub fn component<C: Component>(mut self, props: C::Properties) -> Self {
        self.children.push(VNode::from(VComp::new::<C>(
            props,
            NodeRef::default(),
            None,
        )));
        self
    }

    pub fn text(mut self, text: impl Into<AttrValue>) -> Self {
        self.children.push(VNode::from(VText::new(text)));
        self
    }
}

impl Into<VTag> for Iframe {
    fn into(self) -> VTag {
        let mut vtag = VTag::new("iframe");
        vtag.attributes = Attributes::Vec(self.attributes);
        vtag.add_children(self.children.into_iter());
        vtag
    }
}

impl Into<VNode> for Iframe {
    fn into(self) -> VNode {
        let vtag: VTag = self.into();
        VNode::from(vtag)
    }
}

pub struct Object {
    attributes: Vec<PositionalAttr>,
    children: Vec<VNode>,
}

impl Object {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("accesskey", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("autocapitalize", value));
        self
    }

    pub fn border(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("border", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("class", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contextmenu", value));
        self
    }

    pub fn data(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("data", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("dir", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("draggable", value));
        self
    }

    pub fn form(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("form", value));
        self
    }

    pub fn height(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("height", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("hidden", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("id", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("itemprop", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("lang", value));
        self
    }

    pub fn name(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("name", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("slot", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("spellcheck", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("style", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("tabindex", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("translate", value));
        self
    }

    pub fn r#type(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("type", value));
        self
    }

    pub fn usemap(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("usemap", value));
        self
    }

    pub fn width(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("width", value));
        self
    }

    pub fn child(mut self, element: impl Into<VTag>) -> Self {
        self.children.push(VNode::from(element.into()));
        self
    }

    pub fn component<C: Component>(mut self, props: C::Properties) -> Self {
        self.children.push(VNode::from(VComp::new::<C>(
            props,
            NodeRef::default(),
            None,
        )));
        self
    }

    pub fn text(mut self, text: impl Into<AttrValue>) -> Self {
        self.children.push(VNode::from(VText::new(text)));
        self
    }
}

impl Into<VTag> for Object {
    fn into(self) -> VTag {
        let mut vtag = VTag::new("object");
        vtag.attributes = Attributes::Vec(self.attributes);
        vtag.add_children(self.children.into_iter());
        vtag
    }
}

impl Into<VNode> for Object {
    fn into(self) -> VNode {
        let vtag: VTag = self.into();
        VNode::from(vtag)
    }
}

pub struct Param {
    attributes: Vec<PositionalAttr>,
    children: Vec<VNode>,
}

impl Param {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("accesskey", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("autocapitalize", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("class", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contextmenu", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("dir", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("draggable", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("hidden", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("id", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("itemprop", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("lang", value));
        self
    }

    pub fn name(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("name", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("slot", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("spellcheck", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("style", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("tabindex", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("translate", value));
        self
    }

    pub fn value(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("value", value));
        self
    }

    pub fn child(mut self, element: impl Into<VTag>) -> Self {
        self.children.push(VNode::from(element.into()));
        self
    }

    pub fn component<C: Component>(mut self, props: C::Properties) -> Self {
        self.children.push(VNode::from(VComp::new::<C>(
            props,
            NodeRef::default(),
            None,
        )));
        self
    }

    pub fn text(mut self, text: impl Into<AttrValue>) -> Self {
        self.children.push(VNode::from(VText::new(text)));
        self
    }
}

impl Into<VTag> for Param {
    fn into(self) -> VTag {
        let mut vtag = VTag::new("param");
        vtag.attributes = Attributes::Vec(self.attributes);
        vtag.add_children(self.children.into_iter());
        vtag
    }
}

impl Into<VNode> for Param {
    fn into(self) -> VNode {
        let vtag: VTag = self.into();
        VNode::from(vtag)
    }
}

pub struct Picture {
    attributes: Vec<PositionalAttr>,
    children: Vec<VNode>,
}

impl Picture {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("accesskey", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("autocapitalize", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("class", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contextmenu", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("dir", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("draggable", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("hidden", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("id", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("itemprop", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("lang", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("slot", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("spellcheck", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("style", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("tabindex", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("translate", value));
        self
    }

    pub fn child(mut self, element: impl Into<VTag>) -> Self {
        self.children.push(VNode::from(element.into()));
        self
    }

    pub fn component<C: Component>(mut self, props: C::Properties) -> Self {
        self.children.push(VNode::from(VComp::new::<C>(
            props,
            NodeRef::default(),
            None,
        )));
        self
    }

    pub fn text(mut self, text: impl Into<AttrValue>) -> Self {
        self.children.push(VNode::from(VText::new(text)));
        self
    }
}

impl Into<VTag> for Picture {
    fn into(self) -> VTag {
        let mut vtag = VTag::new("picture");
        vtag.attributes = Attributes::Vec(self.attributes);
        vtag.add_children(self.children.into_iter());
        vtag
    }
}

impl Into<VNode> for Picture {
    fn into(self) -> VNode {
        let vtag: VTag = self.into();
        VNode::from(vtag)
    }
}

pub struct Portal {
    attributes: Vec<PositionalAttr>,
    children: Vec<VNode>,
}

impl Portal {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("accesskey", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("autocapitalize", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("class", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contextmenu", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("dir", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("draggable", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("hidden", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("id", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("itemprop", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("lang", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("slot", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("spellcheck", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("style", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("tabindex", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("translate", value));
        self
    }

    pub fn child(mut self, element: impl Into<VTag>) -> Self {
        self.children.push(VNode::from(element.into()));
        self
    }

    pub fn component<C: Component>(mut self, props: C::Properties) -> Self {
        self.children.push(VNode::from(VComp::new::<C>(
            props,
            NodeRef::default(),
            None,
        )));
        self
    }

    pub fn text(mut self, text: impl Into<AttrValue>) -> Self {
        self.children.push(VNode::from(VText::new(text)));
        self
    }
}

impl Into<VTag> for Portal {
    fn into(self) -> VTag {
        let mut vtag = VTag::new("portal");
        vtag.attributes = Attributes::Vec(self.attributes);
        vtag.add_children(self.children.into_iter());
        vtag
    }
}

impl Into<VNode> for Portal {
    fn into(self) -> VNode {
        let vtag: VTag = self.into();
        VNode::from(vtag)
    }
}

pub struct Source {
    attributes: Vec<PositionalAttr>,
    children: Vec<VNode>,
}

impl Source {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("accesskey", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("autocapitalize", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("class", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contextmenu", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("dir", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("draggable", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("hidden", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("id", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("itemprop", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("lang", value));
        self
    }

    pub fn media(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("media", value));
        self
    }

    pub fn sizes(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("sizes", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("slot", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("spellcheck", value));
        self
    }

    pub fn src(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("src", value));
        self
    }

    pub fn srcset(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("srcset", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("style", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("tabindex", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("translate", value));
        self
    }

    pub fn r#type(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("type", value));
        self
    }

    pub fn child(mut self, element: impl Into<VTag>) -> Self {
        self.children.push(VNode::from(element.into()));
        self
    }

    pub fn component<C: Component>(mut self, props: C::Properties) -> Self {
        self.children.push(VNode::from(VComp::new::<C>(
            props,
            NodeRef::default(),
            None,
        )));
        self
    }

    pub fn text(mut self, text: impl Into<AttrValue>) -> Self {
        self.children.push(VNode::from(VText::new(text)));
        self
    }
}

impl Into<VTag> for Source {
    fn into(self) -> VTag {
        let mut vtag = VTag::new("source");
        vtag.attributes = Attributes::Vec(self.attributes);
        vtag.add_children(self.children.into_iter());
        vtag
    }
}

impl Into<VNode> for Source {
    fn into(self) -> VNode {
        let vtag: VTag = self.into();
        VNode::from(vtag)
    }
}

pub struct Svg {
    attributes: Vec<PositionalAttr>,
    children: Vec<VNode>,
}

impl Svg {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("accesskey", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("autocapitalize", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("class", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contextmenu", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("dir", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("draggable", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("hidden", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("id", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("itemprop", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("lang", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("slot", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("spellcheck", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("style", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("tabindex", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("translate", value));
        self
    }

    pub fn child(mut self, element: impl Into<VTag>) -> Self {
        self.children.push(VNode::from(element.into()));
        self
    }

    pub fn component<C: Component>(mut self, props: C::Properties) -> Self {
        self.children.push(VNode::from(VComp::new::<C>(
            props,
            NodeRef::default(),
            None,
        )));
        self
    }

    pub fn text(mut self, text: impl Into<AttrValue>) -> Self {
        self.children.push(VNode::from(VText::new(text)));
        self
    }
}

impl Into<VTag> for Svg {
    fn into(self) -> VTag {
        let mut vtag = VTag::new("svg");
        vtag.attributes = Attributes::Vec(self.attributes);
        vtag.add_children(self.children.into_iter());
        vtag
    }
}

impl Into<VNode> for Svg {
    fn into(self) -> VNode {
        let vtag: VTag = self.into();
        VNode::from(vtag)
    }
}

pub struct Math {
    attributes: Vec<PositionalAttr>,
    children: Vec<VNode>,
}

impl Math {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("accesskey", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("autocapitalize", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("class", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contextmenu", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("dir", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("draggable", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("hidden", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("id", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("itemprop", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("lang", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("slot", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("spellcheck", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("style", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("tabindex", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("translate", value));
        self
    }

    pub fn child(mut self, element: impl Into<VTag>) -> Self {
        self.children.push(VNode::from(element.into()));
        self
    }

    pub fn component<C: Component>(mut self, props: C::Properties) -> Self {
        self.children.push(VNode::from(VComp::new::<C>(
            props,
            NodeRef::default(),
            None,
        )));
        self
    }

    pub fn text(mut self, text: impl Into<AttrValue>) -> Self {
        self.children.push(VNode::from(VText::new(text)));
        self
    }
}

impl Into<VTag> for Math {
    fn into(self) -> VTag {
        let mut vtag = VTag::new("math");
        vtag.attributes = Attributes::Vec(self.attributes);
        vtag.add_children(self.children.into_iter());
        vtag
    }
}

impl Into<VNode> for Math {
    fn into(self) -> VNode {
        let vtag: VTag = self.into();
        VNode::from(vtag)
    }
}

pub struct Canvas {
    attributes: Vec<PositionalAttr>,
    children: Vec<VNode>,
}

impl Canvas {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("accesskey", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("autocapitalize", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("class", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contextmenu", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("dir", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("draggable", value));
        self
    }

    pub fn height(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("height", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("hidden", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("id", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("itemprop", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("lang", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("slot", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("spellcheck", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("style", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("tabindex", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("translate", value));
        self
    }

    pub fn width(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("width", value));
        self
    }

    pub fn child(mut self, element: impl Into<VTag>) -> Self {
        self.children.push(VNode::from(element.into()));
        self
    }

    pub fn component<C: Component>(mut self, props: C::Properties) -> Self {
        self.children.push(VNode::from(VComp::new::<C>(
            props,
            NodeRef::default(),
            None,
        )));
        self
    }

    pub fn text(mut self, text: impl Into<AttrValue>) -> Self {
        self.children.push(VNode::from(VText::new(text)));
        self
    }
}

impl Into<VTag> for Canvas {
    fn into(self) -> VTag {
        let mut vtag = VTag::new("canvas");
        vtag.attributes = Attributes::Vec(self.attributes);
        vtag.add_children(self.children.into_iter());
        vtag
    }
}

impl Into<VNode> for Canvas {
    fn into(self) -> VNode {
        let vtag: VTag = self.into();
        VNode::from(vtag)
    }
}

pub struct Noscript {
    attributes: Vec<PositionalAttr>,
    children: Vec<VNode>,
}

impl Noscript {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("accesskey", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("autocapitalize", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("class", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contextmenu", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("dir", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("draggable", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("hidden", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("id", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("itemprop", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("lang", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("slot", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("spellcheck", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("style", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("tabindex", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("translate", value));
        self
    }

    pub fn child(mut self, element: impl Into<VTag>) -> Self {
        self.children.push(VNode::from(element.into()));
        self
    }

    pub fn component<C: Component>(mut self, props: C::Properties) -> Self {
        self.children.push(VNode::from(VComp::new::<C>(
            props,
            NodeRef::default(),
            None,
        )));
        self
    }

    pub fn text(mut self, text: impl Into<AttrValue>) -> Self {
        self.children.push(VNode::from(VText::new(text)));
        self
    }
}

impl Into<VTag> for Noscript {
    fn into(self) -> VTag {
        let mut vtag = VTag::new("noscript");
        vtag.attributes = Attributes::Vec(self.attributes);
        vtag.add_children(self.children.into_iter());
        vtag
    }
}

impl Into<VNode> for Noscript {
    fn into(self) -> VNode {
        let vtag: VTag = self.into();
        VNode::from(vtag)
    }
}

pub struct Script {
    attributes: Vec<PositionalAttr>,
    children: Vec<VNode>,
}

impl Script {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("accesskey", value));
        self
    }

    pub fn r#async(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("async", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("autocapitalize", value));
        self
    }

    pub fn charset(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("charset", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("class", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contextmenu", value));
        self
    }

    pub fn crossorigin(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("crossorigin", value));
        self
    }

    pub fn defer(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("defer", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("dir", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("draggable", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("hidden", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("id", value));
        self
    }

    pub fn importance(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("importance", value));
        self
    }

    pub fn integrity(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("integrity", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("itemprop", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("lang", value));
        self
    }

    pub fn language(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("language", value));
        self
    }

    pub fn referrerpolicy(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("referrerpolicy", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("slot", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("spellcheck", value));
        self
    }

    pub fn src(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("src", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("style", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("tabindex", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("translate", value));
        self
    }

    pub fn r#type(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("type", value));
        self
    }

    pub fn child(mut self, element: impl Into<VTag>) -> Self {
        self.children.push(VNode::from(element.into()));
        self
    }

    pub fn component<C: Component>(mut self, props: C::Properties) -> Self {
        self.children.push(VNode::from(VComp::new::<C>(
            props,
            NodeRef::default(),
            None,
        )));
        self
    }

    pub fn text(mut self, text: impl Into<AttrValue>) -> Self {
        self.children.push(VNode::from(VText::new(text)));
        self
    }
}

impl Into<VTag> for Script {
    fn into(self) -> VTag {
        let mut vtag = VTag::new("script");
        vtag.attributes = Attributes::Vec(self.attributes);
        vtag.add_children(self.children.into_iter());
        vtag
    }
}

impl Into<VNode> for Script {
    fn into(self) -> VNode {
        let vtag: VTag = self.into();
        VNode::from(vtag)
    }
}

pub struct Del {
    attributes: Vec<PositionalAttr>,
    children: Vec<VNode>,
}

impl Del {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("accesskey", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("autocapitalize", value));
        self
    }

    pub fn cite(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("cite", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("class", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contextmenu", value));
        self
    }

    pub fn datetime(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("datetime", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("dir", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("draggable", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("hidden", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("id", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("itemprop", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("lang", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("slot", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("spellcheck", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("style", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("tabindex", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("translate", value));
        self
    }

    pub fn child(mut self, element: impl Into<VTag>) -> Self {
        self.children.push(VNode::from(element.into()));
        self
    }

    pub fn component<C: Component>(mut self, props: C::Properties) -> Self {
        self.children.push(VNode::from(VComp::new::<C>(
            props,
            NodeRef::default(),
            None,
        )));
        self
    }

    pub fn text(mut self, text: impl Into<AttrValue>) -> Self {
        self.children.push(VNode::from(VText::new(text)));
        self
    }
}

impl Into<VTag> for Del {
    fn into(self) -> VTag {
        let mut vtag = VTag::new("del");
        vtag.attributes = Attributes::Vec(self.attributes);
        vtag.add_children(self.children.into_iter());
        vtag
    }
}

impl Into<VNode> for Del {
    fn into(self) -> VNode {
        let vtag: VTag = self.into();
        VNode::from(vtag)
    }
}

pub struct Ins {
    attributes: Vec<PositionalAttr>,
    children: Vec<VNode>,
}

impl Ins {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("accesskey", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("autocapitalize", value));
        self
    }

    pub fn cite(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("cite", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("class", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contextmenu", value));
        self
    }

    pub fn datetime(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("datetime", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("dir", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("draggable", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("hidden", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("id", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("itemprop", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("lang", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("slot", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("spellcheck", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("style", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("tabindex", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("translate", value));
        self
    }

    pub fn child(mut self, element: impl Into<VTag>) -> Self {
        self.children.push(VNode::from(element.into()));
        self
    }

    pub fn component<C: Component>(mut self, props: C::Properties) -> Self {
        self.children.push(VNode::from(VComp::new::<C>(
            props,
            NodeRef::default(),
            None,
        )));
        self
    }

    pub fn text(mut self, text: impl Into<AttrValue>) -> Self {
        self.children.push(VNode::from(VText::new(text)));
        self
    }
}

impl Into<VTag> for Ins {
    fn into(self) -> VTag {
        let mut vtag = VTag::new("ins");
        vtag.attributes = Attributes::Vec(self.attributes);
        vtag.add_children(self.children.into_iter());
        vtag
    }
}

impl Into<VNode> for Ins {
    fn into(self) -> VNode {
        let vtag: VTag = self.into();
        VNode::from(vtag)
    }
}

pub struct Caption {
    attributes: Vec<PositionalAttr>,
    children: Vec<VNode>,
}

impl Caption {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("accesskey", value));
        self
    }

    pub fn align(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("align", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("autocapitalize", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("class", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contextmenu", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("dir", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("draggable", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("hidden", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("id", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("itemprop", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("lang", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("slot", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("spellcheck", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("style", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("tabindex", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("translate", value));
        self
    }

    pub fn child(mut self, element: impl Into<VTag>) -> Self {
        self.children.push(VNode::from(element.into()));
        self
    }

    pub fn component<C: Component>(mut self, props: C::Properties) -> Self {
        self.children.push(VNode::from(VComp::new::<C>(
            props,
            NodeRef::default(),
            None,
        )));
        self
    }

    pub fn text(mut self, text: impl Into<AttrValue>) -> Self {
        self.children.push(VNode::from(VText::new(text)));
        self
    }
}

impl Into<VTag> for Caption {
    fn into(self) -> VTag {
        let mut vtag = VTag::new("caption");
        vtag.attributes = Attributes::Vec(self.attributes);
        vtag.add_children(self.children.into_iter());
        vtag
    }
}

impl Into<VNode> for Caption {
    fn into(self) -> VNode {
        let vtag: VTag = self.into();
        VNode::from(vtag)
    }
}

pub struct Col {
    attributes: Vec<PositionalAttr>,
    children: Vec<VNode>,
}

impl Col {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("accesskey", value));
        self
    }

    pub fn align(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("align", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("autocapitalize", value));
        self
    }

    pub fn bgcolor(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("bgcolor", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("class", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contextmenu", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("dir", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("draggable", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("hidden", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("id", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("itemprop", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("lang", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("slot", value));
        self
    }

    pub fn span(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("span", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("spellcheck", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("style", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("tabindex", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("translate", value));
        self
    }

    pub fn child(mut self, element: impl Into<VTag>) -> Self {
        self.children.push(VNode::from(element.into()));
        self
    }

    pub fn component<C: Component>(mut self, props: C::Properties) -> Self {
        self.children.push(VNode::from(VComp::new::<C>(
            props,
            NodeRef::default(),
            None,
        )));
        self
    }

    pub fn text(mut self, text: impl Into<AttrValue>) -> Self {
        self.children.push(VNode::from(VText::new(text)));
        self
    }
}

impl Into<VTag> for Col {
    fn into(self) -> VTag {
        let mut vtag = VTag::new("col");
        vtag.attributes = Attributes::Vec(self.attributes);
        vtag.add_children(self.children.into_iter());
        vtag
    }
}

impl Into<VNode> for Col {
    fn into(self) -> VNode {
        let vtag: VTag = self.into();
        VNode::from(vtag)
    }
}

pub struct Colgroup {
    attributes: Vec<PositionalAttr>,
    children: Vec<VNode>,
}

impl Colgroup {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("accesskey", value));
        self
    }

    pub fn align(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("align", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("autocapitalize", value));
        self
    }

    pub fn bgcolor(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("bgcolor", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("class", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contextmenu", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("dir", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("draggable", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("hidden", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("id", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("itemprop", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("lang", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("slot", value));
        self
    }

    pub fn span(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("span", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("spellcheck", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("style", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("tabindex", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("translate", value));
        self
    }

    pub fn child(mut self, element: impl Into<VTag>) -> Self {
        self.children.push(VNode::from(element.into()));
        self
    }

    pub fn component<C: Component>(mut self, props: C::Properties) -> Self {
        self.children.push(VNode::from(VComp::new::<C>(
            props,
            NodeRef::default(),
            None,
        )));
        self
    }

    pub fn text(mut self, text: impl Into<AttrValue>) -> Self {
        self.children.push(VNode::from(VText::new(text)));
        self
    }
}

impl Into<VTag> for Colgroup {
    fn into(self) -> VTag {
        let mut vtag = VTag::new("colgroup");
        vtag.attributes = Attributes::Vec(self.attributes);
        vtag.add_children(self.children.into_iter());
        vtag
    }
}

impl Into<VNode> for Colgroup {
    fn into(self) -> VNode {
        let vtag: VTag = self.into();
        VNode::from(vtag)
    }
}

pub struct Table {
    attributes: Vec<PositionalAttr>,
    children: Vec<VNode>,
}

impl Table {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("accesskey", value));
        self
    }

    pub fn align(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("align", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("autocapitalize", value));
        self
    }

    pub fn background(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("background", value));
        self
    }

    pub fn bgcolor(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("bgcolor", value));
        self
    }

    pub fn border(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("border", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("class", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contextmenu", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("dir", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("draggable", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("hidden", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("id", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("itemprop", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("lang", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("slot", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("spellcheck", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("style", value));
        self
    }

    pub fn summary(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("summary", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("tabindex", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("translate", value));
        self
    }

    pub fn child(mut self, element: impl Into<VTag>) -> Self {
        self.children.push(VNode::from(element.into()));
        self
    }

    pub fn component<C: Component>(mut self, props: C::Properties) -> Self {
        self.children.push(VNode::from(VComp::new::<C>(
            props,
            NodeRef::default(),
            None,
        )));
        self
    }

    pub fn text(mut self, text: impl Into<AttrValue>) -> Self {
        self.children.push(VNode::from(VText::new(text)));
        self
    }
}

impl Into<VTag> for Table {
    fn into(self) -> VTag {
        let mut vtag = VTag::new("table");
        vtag.attributes = Attributes::Vec(self.attributes);
        vtag.add_children(self.children.into_iter());
        vtag
    }
}

impl Into<VNode> for Table {
    fn into(self) -> VNode {
        let vtag: VTag = self.into();
        VNode::from(vtag)
    }
}

pub struct Tbody {
    attributes: Vec<PositionalAttr>,
    children: Vec<VNode>,
}

impl Tbody {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("accesskey", value));
        self
    }

    pub fn align(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("align", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("autocapitalize", value));
        self
    }

    pub fn bgcolor(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("bgcolor", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("class", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contextmenu", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("dir", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("draggable", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("hidden", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("id", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("itemprop", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("lang", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("slot", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("spellcheck", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("style", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("tabindex", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("translate", value));
        self
    }

    pub fn child(mut self, element: impl Into<VTag>) -> Self {
        self.children.push(VNode::from(element.into()));
        self
    }

    pub fn component<C: Component>(mut self, props: C::Properties) -> Self {
        self.children.push(VNode::from(VComp::new::<C>(
            props,
            NodeRef::default(),
            None,
        )));
        self
    }

    pub fn text(mut self, text: impl Into<AttrValue>) -> Self {
        self.children.push(VNode::from(VText::new(text)));
        self
    }
}

impl Into<VTag> for Tbody {
    fn into(self) -> VTag {
        let mut vtag = VTag::new("tbody");
        vtag.attributes = Attributes::Vec(self.attributes);
        vtag.add_children(self.children.into_iter());
        vtag
    }
}

impl Into<VNode> for Tbody {
    fn into(self) -> VNode {
        let vtag: VTag = self.into();
        VNode::from(vtag)
    }
}

pub struct Td {
    attributes: Vec<PositionalAttr>,
    children: Vec<VNode>,
}

impl Td {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("accesskey", value));
        self
    }

    pub fn align(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("align", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("autocapitalize", value));
        self
    }

    pub fn background(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("background", value));
        self
    }

    pub fn bgcolor(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("bgcolor", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("class", value));
        self
    }

    pub fn colspan(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("colspan", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contextmenu", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("dir", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("draggable", value));
        self
    }

    pub fn headers(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("headers", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("hidden", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("id", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("itemprop", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("lang", value));
        self
    }

    pub fn rowspan(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("rowspan", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("slot", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("spellcheck", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("style", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("tabindex", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("translate", value));
        self
    }

    pub fn child(mut self, element: impl Into<VTag>) -> Self {
        self.children.push(VNode::from(element.into()));
        self
    }

    pub fn component<C: Component>(mut self, props: C::Properties) -> Self {
        self.children.push(VNode::from(VComp::new::<C>(
            props,
            NodeRef::default(),
            None,
        )));
        self
    }

    pub fn text(mut self, text: impl Into<AttrValue>) -> Self {
        self.children.push(VNode::from(VText::new(text)));
        self
    }
}

impl Into<VTag> for Td {
    fn into(self) -> VTag {
        let mut vtag = VTag::new("td");
        vtag.attributes = Attributes::Vec(self.attributes);
        vtag.add_children(self.children.into_iter());
        vtag
    }
}

impl Into<VNode> for Td {
    fn into(self) -> VNode {
        let vtag: VTag = self.into();
        VNode::from(vtag)
    }
}

pub struct Tfoot {
    attributes: Vec<PositionalAttr>,
    children: Vec<VNode>,
}

impl Tfoot {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("accesskey", value));
        self
    }

    pub fn align(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("align", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("autocapitalize", value));
        self
    }

    pub fn bgcolor(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("bgcolor", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("class", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contextmenu", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("dir", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("draggable", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("hidden", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("id", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("itemprop", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("lang", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("slot", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("spellcheck", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("style", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("tabindex", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("translate", value));
        self
    }

    pub fn child(mut self, element: impl Into<VTag>) -> Self {
        self.children.push(VNode::from(element.into()));
        self
    }

    pub fn component<C: Component>(mut self, props: C::Properties) -> Self {
        self.children.push(VNode::from(VComp::new::<C>(
            props,
            NodeRef::default(),
            None,
        )));
        self
    }

    pub fn text(mut self, text: impl Into<AttrValue>) -> Self {
        self.children.push(VNode::from(VText::new(text)));
        self
    }
}

impl Into<VTag> for Tfoot {
    fn into(self) -> VTag {
        let mut vtag = VTag::new("tfoot");
        vtag.attributes = Attributes::Vec(self.attributes);
        vtag.add_children(self.children.into_iter());
        vtag
    }
}

impl Into<VNode> for Tfoot {
    fn into(self) -> VNode {
        let vtag: VTag = self.into();
        VNode::from(vtag)
    }
}

pub struct Th {
    attributes: Vec<PositionalAttr>,
    children: Vec<VNode>,
}

impl Th {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("accesskey", value));
        self
    }

    pub fn align(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("align", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("autocapitalize", value));
        self
    }

    pub fn background(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("background", value));
        self
    }

    pub fn bgcolor(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("bgcolor", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("class", value));
        self
    }

    pub fn colspan(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("colspan", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contextmenu", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("dir", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("draggable", value));
        self
    }

    pub fn headers(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("headers", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("hidden", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("id", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("itemprop", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("lang", value));
        self
    }

    pub fn rowspan(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("rowspan", value));
        self
    }

    pub fn scope(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("scope", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("slot", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("spellcheck", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("style", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("tabindex", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("translate", value));
        self
    }

    pub fn child(mut self, element: impl Into<VTag>) -> Self {
        self.children.push(VNode::from(element.into()));
        self
    }

    pub fn component<C: Component>(mut self, props: C::Properties) -> Self {
        self.children.push(VNode::from(VComp::new::<C>(
            props,
            NodeRef::default(),
            None,
        )));
        self
    }

    pub fn text(mut self, text: impl Into<AttrValue>) -> Self {
        self.children.push(VNode::from(VText::new(text)));
        self
    }
}

impl Into<VTag> for Th {
    fn into(self) -> VTag {
        let mut vtag = VTag::new("th");
        vtag.attributes = Attributes::Vec(self.attributes);
        vtag.add_children(self.children.into_iter());
        vtag
    }
}

impl Into<VNode> for Th {
    fn into(self) -> VNode {
        let vtag: VTag = self.into();
        VNode::from(vtag)
    }
}

pub struct Thead {
    attributes: Vec<PositionalAttr>,
    children: Vec<VNode>,
}

impl Thead {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("accesskey", value));
        self
    }

    pub fn align(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("align", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("autocapitalize", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("class", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contextmenu", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("dir", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("draggable", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("hidden", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("id", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("itemprop", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("lang", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("slot", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("spellcheck", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("style", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("tabindex", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("translate", value));
        self
    }

    pub fn child(mut self, element: impl Into<VTag>) -> Self {
        self.children.push(VNode::from(element.into()));
        self
    }

    pub fn component<C: Component>(mut self, props: C::Properties) -> Self {
        self.children.push(VNode::from(VComp::new::<C>(
            props,
            NodeRef::default(),
            None,
        )));
        self
    }

    pub fn text(mut self, text: impl Into<AttrValue>) -> Self {
        self.children.push(VNode::from(VText::new(text)));
        self
    }
}

impl Into<VTag> for Thead {
    fn into(self) -> VTag {
        let mut vtag = VTag::new("thead");
        vtag.attributes = Attributes::Vec(self.attributes);
        vtag.add_children(self.children.into_iter());
        vtag
    }
}

impl Into<VNode> for Thead {
    fn into(self) -> VNode {
        let vtag: VTag = self.into();
        VNode::from(vtag)
    }
}

pub struct Tr {
    attributes: Vec<PositionalAttr>,
    children: Vec<VNode>,
}

impl Tr {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("accesskey", value));
        self
    }

    pub fn align(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("align", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("autocapitalize", value));
        self
    }

    pub fn bgcolor(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("bgcolor", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("class", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contextmenu", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("dir", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("draggable", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("hidden", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("id", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("itemprop", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("lang", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("slot", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("spellcheck", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("style", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("tabindex", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("translate", value));
        self
    }

    pub fn child(mut self, element: impl Into<VTag>) -> Self {
        self.children.push(VNode::from(element.into()));
        self
    }

    pub fn component<C: Component>(mut self, props: C::Properties) -> Self {
        self.children.push(VNode::from(VComp::new::<C>(
            props,
            NodeRef::default(),
            None,
        )));
        self
    }

    pub fn text(mut self, text: impl Into<AttrValue>) -> Self {
        self.children.push(VNode::from(VText::new(text)));
        self
    }
}

impl Into<VTag> for Tr {
    fn into(self) -> VTag {
        let mut vtag = VTag::new("tr");
        vtag.attributes = Attributes::Vec(self.attributes);
        vtag.add_children(self.children.into_iter());
        vtag
    }
}

impl Into<VNode> for Tr {
    fn into(self) -> VNode {
        let vtag: VTag = self.into();
        VNode::from(vtag)
    }
}

pub struct Button {
    attributes: Vec<PositionalAttr>,
    children: Vec<VNode>,
}

impl Button {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("accesskey", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("autocapitalize", value));
        self
    }

    pub fn autofocus(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("autofocus", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("class", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contextmenu", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("dir", value));
        self
    }

    pub fn disabled(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("disabled", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("draggable", value));
        self
    }

    pub fn form(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("form", value));
        self
    }

    pub fn formaction(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("formaction", value));
        self
    }

    pub fn formenctype(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("formenctype", value));
        self
    }

    pub fn formmethod(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("formmethod", value));
        self
    }

    pub fn formnovalidate(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("formnovalidate", value));
        self
    }

    pub fn formtarget(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("formtarget", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("hidden", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("id", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("itemprop", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("lang", value));
        self
    }

    pub fn name(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("name", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("slot", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("spellcheck", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("style", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("tabindex", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("translate", value));
        self
    }

    pub fn r#type(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("type", value));
        self
    }

    pub fn value(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("value", value));
        self
    }

    pub fn child(mut self, element: impl Into<VTag>) -> Self {
        self.children.push(VNode::from(element.into()));
        self
    }

    pub fn component<C: Component>(mut self, props: C::Properties) -> Self {
        self.children.push(VNode::from(VComp::new::<C>(
            props,
            NodeRef::default(),
            None,
        )));
        self
    }

    pub fn text(mut self, text: impl Into<AttrValue>) -> Self {
        self.children.push(VNode::from(VText::new(text)));
        self
    }
}

impl Into<VTag> for Button {
    fn into(self) -> VTag {
        let mut vtag = VTag::new("button");
        vtag.attributes = Attributes::Vec(self.attributes);
        vtag.add_children(self.children.into_iter());
        vtag
    }
}

impl Into<VNode> for Button {
    fn into(self) -> VNode {
        let vtag: VTag = self.into();
        VNode::from(vtag)
    }
}

pub struct Datalist {
    attributes: Vec<PositionalAttr>,
    children: Vec<VNode>,
}

impl Datalist {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("accesskey", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("autocapitalize", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("class", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contextmenu", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("dir", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("draggable", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("hidden", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("id", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("itemprop", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("lang", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("slot", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("spellcheck", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("style", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("tabindex", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("translate", value));
        self
    }

    pub fn child(mut self, element: impl Into<VTag>) -> Self {
        self.children.push(VNode::from(element.into()));
        self
    }

    pub fn component<C: Component>(mut self, props: C::Properties) -> Self {
        self.children.push(VNode::from(VComp::new::<C>(
            props,
            NodeRef::default(),
            None,
        )));
        self
    }

    pub fn text(mut self, text: impl Into<AttrValue>) -> Self {
        self.children.push(VNode::from(VText::new(text)));
        self
    }
}

impl Into<VTag> for Datalist {
    fn into(self) -> VTag {
        let mut vtag = VTag::new("datalist");
        vtag.attributes = Attributes::Vec(self.attributes);
        vtag.add_children(self.children.into_iter());
        vtag
    }
}

impl Into<VNode> for Datalist {
    fn into(self) -> VNode {
        let vtag: VTag = self.into();
        VNode::from(vtag)
    }
}

pub struct Fieldset {
    attributes: Vec<PositionalAttr>,
    children: Vec<VNode>,
}

impl Fieldset {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("accesskey", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("autocapitalize", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("class", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contextmenu", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("dir", value));
        self
    }

    pub fn disabled(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("disabled", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("draggable", value));
        self
    }

    pub fn form(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("form", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("hidden", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("id", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("itemprop", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("lang", value));
        self
    }

    pub fn name(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("name", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("slot", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("spellcheck", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("style", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("tabindex", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("translate", value));
        self
    }

    pub fn child(mut self, element: impl Into<VTag>) -> Self {
        self.children.push(VNode::from(element.into()));
        self
    }

    pub fn component<C: Component>(mut self, props: C::Properties) -> Self {
        self.children.push(VNode::from(VComp::new::<C>(
            props,
            NodeRef::default(),
            None,
        )));
        self
    }

    pub fn text(mut self, text: impl Into<AttrValue>) -> Self {
        self.children.push(VNode::from(VText::new(text)));
        self
    }
}

impl Into<VTag> for Fieldset {
    fn into(self) -> VTag {
        let mut vtag = VTag::new("fieldset");
        vtag.attributes = Attributes::Vec(self.attributes);
        vtag.add_children(self.children.into_iter());
        vtag
    }
}

impl Into<VNode> for Fieldset {
    fn into(self) -> VNode {
        let vtag: VTag = self.into();
        VNode::from(vtag)
    }
}

pub struct Form {
    attributes: Vec<PositionalAttr>,
    children: Vec<VNode>,
}

impl Form {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accept(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("accept", value));
        self
    }

    pub fn accept_charset(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("accept_charset", value));
        self
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("accesskey", value));
        self
    }

    pub fn action(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("action", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("autocapitalize", value));
        self
    }

    pub fn autocomplete(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("autocomplete", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("class", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contextmenu", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("dir", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("draggable", value));
        self
    }

    pub fn enctype(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("enctype", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("hidden", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("id", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("itemprop", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("lang", value));
        self
    }

    pub fn method(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("method", value));
        self
    }

    pub fn name(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("name", value));
        self
    }

    pub fn novalidate(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("novalidate", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("slot", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("spellcheck", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("style", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("tabindex", value));
        self
    }

    pub fn target(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("target", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("translate", value));
        self
    }

    pub fn child(mut self, element: impl Into<VTag>) -> Self {
        self.children.push(VNode::from(element.into()));
        self
    }

    pub fn component<C: Component>(mut self, props: C::Properties) -> Self {
        self.children.push(VNode::from(VComp::new::<C>(
            props,
            NodeRef::default(),
            None,
        )));
        self
    }

    pub fn text(mut self, text: impl Into<AttrValue>) -> Self {
        self.children.push(VNode::from(VText::new(text)));
        self
    }
}

impl Into<VTag> for Form {
    fn into(self) -> VTag {
        let mut vtag = VTag::new("form");
        vtag.attributes = Attributes::Vec(self.attributes);
        vtag.add_children(self.children.into_iter());
        vtag
    }
}

impl Into<VNode> for Form {
    fn into(self) -> VNode {
        let vtag: VTag = self.into();
        VNode::from(vtag)
    }
}

pub struct Input {
    attributes: Vec<PositionalAttr>,
    children: Vec<VNode>,
}

impl Input {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accept(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("accept", value));
        self
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("accesskey", value));
        self
    }

    pub fn alt(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("alt", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("autocapitalize", value));
        self
    }

    pub fn autocomplete(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("autocomplete", value));
        self
    }

    pub fn autofocus(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("autofocus", value));
        self
    }

    pub fn capture(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("capture", value));
        self
    }

    pub fn checked(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("checked", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("class", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contextmenu", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("dir", value));
        self
    }

    pub fn dirname(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("dirname", value));
        self
    }

    pub fn disabled(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("disabled", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("draggable", value));
        self
    }

    pub fn form(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("form", value));
        self
    }

    pub fn formaction(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("formaction", value));
        self
    }

    pub fn formenctype(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("formenctype", value));
        self
    }

    pub fn formmethod(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("formmethod", value));
        self
    }

    pub fn formnovalidate(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("formnovalidate", value));
        self
    }

    pub fn formtarget(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("formtarget", value));
        self
    }

    pub fn height(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("height", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("hidden", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("id", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("itemprop", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("lang", value));
        self
    }

    pub fn list(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("list", value));
        self
    }

    pub fn max(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("max", value));
        self
    }

    pub fn maxlength(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("maxlength", value));
        self
    }

    pub fn minlength(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("minlength", value));
        self
    }

    pub fn min(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("min", value));
        self
    }

    pub fn multiple(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("multiple", value));
        self
    }

    pub fn name(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("name", value));
        self
    }

    pub fn pattern(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("pattern", value));
        self
    }

    pub fn placeholder(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("placeholder", value));
        self
    }

    pub fn readonly(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("readonly", value));
        self
    }

    pub fn required(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("required", value));
        self
    }

    pub fn size(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("size", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("slot", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("spellcheck", value));
        self
    }

    pub fn src(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("src", value));
        self
    }

    pub fn step(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("step", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("style", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("tabindex", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("translate", value));
        self
    }

    pub fn r#type(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("type", value));
        self
    }

    pub fn usemap(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("usemap", value));
        self
    }

    pub fn value(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("value", value));
        self
    }

    pub fn width(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("width", value));
        self
    }

    pub fn child(mut self, element: impl Into<VTag>) -> Self {
        self.children.push(VNode::from(element.into()));
        self
    }

    pub fn component<C: Component>(mut self, props: C::Properties) -> Self {
        self.children.push(VNode::from(VComp::new::<C>(
            props,
            NodeRef::default(),
            None,
        )));
        self
    }

    pub fn text(mut self, text: impl Into<AttrValue>) -> Self {
        self.children.push(VNode::from(VText::new(text)));
        self
    }
}

impl Into<VTag> for Input {
    fn into(self) -> VTag {
        let mut vtag = VTag::new("input");
        vtag.attributes = Attributes::Vec(self.attributes);
        vtag.add_children(self.children.into_iter());
        vtag
    }
}

impl Into<VNode> for Input {
    fn into(self) -> VNode {
        let vtag: VTag = self.into();
        VNode::from(vtag)
    }
}

pub struct Label {
    attributes: Vec<PositionalAttr>,
    children: Vec<VNode>,
}

impl Label {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("accesskey", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("autocapitalize", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("class", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contextmenu", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("dir", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("draggable", value));
        self
    }

    pub fn r#for(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("for", value));
        self
    }

    pub fn form(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("form", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("hidden", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("id", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("itemprop", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("lang", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("slot", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("spellcheck", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("style", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("tabindex", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("translate", value));
        self
    }

    pub fn child(mut self, element: impl Into<VTag>) -> Self {
        self.children.push(VNode::from(element.into()));
        self
    }

    pub fn component<C: Component>(mut self, props: C::Properties) -> Self {
        self.children.push(VNode::from(VComp::new::<C>(
            props,
            NodeRef::default(),
            None,
        )));
        self
    }

    pub fn text(mut self, text: impl Into<AttrValue>) -> Self {
        self.children.push(VNode::from(VText::new(text)));
        self
    }
}

impl Into<VTag> for Label {
    fn into(self) -> VTag {
        let mut vtag = VTag::new("label");
        vtag.attributes = Attributes::Vec(self.attributes);
        vtag.add_children(self.children.into_iter());
        vtag
    }
}

impl Into<VNode> for Label {
    fn into(self) -> VNode {
        let vtag: VTag = self.into();
        VNode::from(vtag)
    }
}

pub struct Legend {
    attributes: Vec<PositionalAttr>,
    children: Vec<VNode>,
}

impl Legend {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("accesskey", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("autocapitalize", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("class", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contextmenu", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("dir", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("draggable", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("hidden", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("id", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("itemprop", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("lang", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("slot", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("spellcheck", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("style", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("tabindex", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("translate", value));
        self
    }

    pub fn child(mut self, element: impl Into<VTag>) -> Self {
        self.children.push(VNode::from(element.into()));
        self
    }

    pub fn component<C: Component>(mut self, props: C::Properties) -> Self {
        self.children.push(VNode::from(VComp::new::<C>(
            props,
            NodeRef::default(),
            None,
        )));
        self
    }

    pub fn text(mut self, text: impl Into<AttrValue>) -> Self {
        self.children.push(VNode::from(VText::new(text)));
        self
    }
}

impl Into<VTag> for Legend {
    fn into(self) -> VTag {
        let mut vtag = VTag::new("legend");
        vtag.attributes = Attributes::Vec(self.attributes);
        vtag.add_children(self.children.into_iter());
        vtag
    }
}

impl Into<VNode> for Legend {
    fn into(self) -> VNode {
        let vtag: VTag = self.into();
        VNode::from(vtag)
    }
}

pub struct Meter {
    attributes: Vec<PositionalAttr>,
    children: Vec<VNode>,
}

impl Meter {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("accesskey", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("autocapitalize", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("class", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contextmenu", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("dir", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("draggable", value));
        self
    }

    pub fn form(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("form", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("hidden", value));
        self
    }

    pub fn high(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("high", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("id", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("itemprop", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("lang", value));
        self
    }

    pub fn low(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("low", value));
        self
    }

    pub fn max(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("max", value));
        self
    }

    pub fn min(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("min", value));
        self
    }

    pub fn optimum(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("optimum", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("slot", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("spellcheck", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("style", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("tabindex", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("translate", value));
        self
    }

    pub fn value(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("value", value));
        self
    }

    pub fn child(mut self, element: impl Into<VTag>) -> Self {
        self.children.push(VNode::from(element.into()));
        self
    }

    pub fn component<C: Component>(mut self, props: C::Properties) -> Self {
        self.children.push(VNode::from(VComp::new::<C>(
            props,
            NodeRef::default(),
            None,
        )));
        self
    }

    pub fn text(mut self, text: impl Into<AttrValue>) -> Self {
        self.children.push(VNode::from(VText::new(text)));
        self
    }
}

impl Into<VTag> for Meter {
    fn into(self) -> VTag {
        let mut vtag = VTag::new("meter");
        vtag.attributes = Attributes::Vec(self.attributes);
        vtag.add_children(self.children.into_iter());
        vtag
    }
}

impl Into<VNode> for Meter {
    fn into(self) -> VNode {
        let vtag: VTag = self.into();
        VNode::from(vtag)
    }
}

pub struct Optgroup {
    attributes: Vec<PositionalAttr>,
    children: Vec<VNode>,
}

impl Optgroup {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("accesskey", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("autocapitalize", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("class", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contextmenu", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("dir", value));
        self
    }

    pub fn disabled(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("disabled", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("draggable", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("hidden", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("id", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("itemprop", value));
        self
    }

    pub fn label(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("label", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("lang", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("slot", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("spellcheck", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("style", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("tabindex", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("translate", value));
        self
    }

    pub fn child(mut self, element: impl Into<VTag>) -> Self {
        self.children.push(VNode::from(element.into()));
        self
    }

    pub fn component<C: Component>(mut self, props: C::Properties) -> Self {
        self.children.push(VNode::from(VComp::new::<C>(
            props,
            NodeRef::default(),
            None,
        )));
        self
    }

    pub fn text(mut self, text: impl Into<AttrValue>) -> Self {
        self.children.push(VNode::from(VText::new(text)));
        self
    }
}

impl Into<VTag> for Optgroup {
    fn into(self) -> VTag {
        let mut vtag = VTag::new("optgroup");
        vtag.attributes = Attributes::Vec(self.attributes);
        vtag.add_children(self.children.into_iter());
        vtag
    }
}

impl Into<VNode> for Optgroup {
    fn into(self) -> VNode {
        let vtag: VTag = self.into();
        VNode::from(vtag)
    }
}

pub struct Option {
    attributes: Vec<PositionalAttr>,
    children: Vec<VNode>,
}

impl Option {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("accesskey", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("autocapitalize", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("class", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contextmenu", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("dir", value));
        self
    }

    pub fn disabled(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("disabled", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("draggable", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("hidden", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("id", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("itemprop", value));
        self
    }

    pub fn label(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("label", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("lang", value));
        self
    }

    pub fn selected(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("selected", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("slot", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("spellcheck", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("style", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("tabindex", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("translate", value));
        self
    }

    pub fn value(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("value", value));
        self
    }

    pub fn child(mut self, element: impl Into<VTag>) -> Self {
        self.children.push(VNode::from(element.into()));
        self
    }

    pub fn component<C: Component>(mut self, props: C::Properties) -> Self {
        self.children.push(VNode::from(VComp::new::<C>(
            props,
            NodeRef::default(),
            None,
        )));
        self
    }

    pub fn text(mut self, text: impl Into<AttrValue>) -> Self {
        self.children.push(VNode::from(VText::new(text)));
        self
    }
}

impl Into<VTag> for Option {
    fn into(self) -> VTag {
        let mut vtag = VTag::new("option");
        vtag.attributes = Attributes::Vec(self.attributes);
        vtag.add_children(self.children.into_iter());
        vtag
    }
}

impl Into<VNode> for Option {
    fn into(self) -> VNode {
        let vtag: VTag = self.into();
        VNode::from(vtag)
    }
}

pub struct Output {
    attributes: Vec<PositionalAttr>,
    children: Vec<VNode>,
}

impl Output {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("accesskey", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("autocapitalize", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("class", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contextmenu", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("dir", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("draggable", value));
        self
    }

    pub fn r#for(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("for", value));
        self
    }

    pub fn form(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("form", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("hidden", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("id", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("itemprop", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("lang", value));
        self
    }

    pub fn name(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("name", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("slot", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("spellcheck", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("style", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("tabindex", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("translate", value));
        self
    }

    pub fn child(mut self, element: impl Into<VTag>) -> Self {
        self.children.push(VNode::from(element.into()));
        self
    }

    pub fn component<C: Component>(mut self, props: C::Properties) -> Self {
        self.children.push(VNode::from(VComp::new::<C>(
            props,
            NodeRef::default(),
            None,
        )));
        self
    }

    pub fn text(mut self, text: impl Into<AttrValue>) -> Self {
        self.children.push(VNode::from(VText::new(text)));
        self
    }
}

impl Into<VTag> for Output {
    fn into(self) -> VTag {
        let mut vtag = VTag::new("output");
        vtag.attributes = Attributes::Vec(self.attributes);
        vtag.add_children(self.children.into_iter());
        vtag
    }
}

impl Into<VNode> for Output {
    fn into(self) -> VNode {
        let vtag: VTag = self.into();
        VNode::from(vtag)
    }
}

pub struct Progress {
    attributes: Vec<PositionalAttr>,
    children: Vec<VNode>,
}

impl Progress {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("accesskey", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("autocapitalize", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("class", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contextmenu", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("dir", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("draggable", value));
        self
    }

    pub fn form(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("form", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("hidden", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("id", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("itemprop", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("lang", value));
        self
    }

    pub fn max(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("max", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("slot", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("spellcheck", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("style", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("tabindex", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("translate", value));
        self
    }

    pub fn value(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("value", value));
        self
    }

    pub fn child(mut self, element: impl Into<VTag>) -> Self {
        self.children.push(VNode::from(element.into()));
        self
    }

    pub fn component<C: Component>(mut self, props: C::Properties) -> Self {
        self.children.push(VNode::from(VComp::new::<C>(
            props,
            NodeRef::default(),
            None,
        )));
        self
    }

    pub fn text(mut self, text: impl Into<AttrValue>) -> Self {
        self.children.push(VNode::from(VText::new(text)));
        self
    }
}

impl Into<VTag> for Progress {
    fn into(self) -> VTag {
        let mut vtag = VTag::new("progress");
        vtag.attributes = Attributes::Vec(self.attributes);
        vtag.add_children(self.children.into_iter());
        vtag
    }
}

impl Into<VNode> for Progress {
    fn into(self) -> VNode {
        let vtag: VTag = self.into();
        VNode::from(vtag)
    }
}

pub struct Select {
    attributes: Vec<PositionalAttr>,
    children: Vec<VNode>,
}

impl Select {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("accesskey", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("autocapitalize", value));
        self
    }

    pub fn autocomplete(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("autocomplete", value));
        self
    }

    pub fn autofocus(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("autofocus", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("class", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contextmenu", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("dir", value));
        self
    }

    pub fn disabled(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("disabled", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("draggable", value));
        self
    }

    pub fn form(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("form", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("hidden", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("id", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("itemprop", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("lang", value));
        self
    }

    pub fn multiple(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("multiple", value));
        self
    }

    pub fn name(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("name", value));
        self
    }

    pub fn required(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("required", value));
        self
    }

    pub fn size(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("size", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("slot", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("spellcheck", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("style", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("tabindex", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("translate", value));
        self
    }

    pub fn child(mut self, element: impl Into<VTag>) -> Self {
        self.children.push(VNode::from(element.into()));
        self
    }

    pub fn component<C: Component>(mut self, props: C::Properties) -> Self {
        self.children.push(VNode::from(VComp::new::<C>(
            props,
            NodeRef::default(),
            None,
        )));
        self
    }

    pub fn text(mut self, text: impl Into<AttrValue>) -> Self {
        self.children.push(VNode::from(VText::new(text)));
        self
    }
}

impl Into<VTag> for Select {
    fn into(self) -> VTag {
        let mut vtag = VTag::new("select");
        vtag.attributes = Attributes::Vec(self.attributes);
        vtag.add_children(self.children.into_iter());
        vtag
    }
}

impl Into<VNode> for Select {
    fn into(self) -> VNode {
        let vtag: VTag = self.into();
        VNode::from(vtag)
    }
}

pub struct Textarea {
    attributes: Vec<PositionalAttr>,
    children: Vec<VNode>,
}

impl Textarea {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("accesskey", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("autocapitalize", value));
        self
    }

    pub fn autocomplete(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("autocomplete", value));
        self
    }

    pub fn autofocus(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("autofocus", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("class", value));
        self
    }

    pub fn cols(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("cols", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contextmenu", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("dir", value));
        self
    }

    pub fn dirname(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("dirname", value));
        self
    }

    pub fn disabled(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("disabled", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("draggable", value));
        self
    }

    pub fn enterkeyhint(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("enterkeyhint", value));
        self
    }

    pub fn form(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("form", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("hidden", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("id", value));
        self
    }

    pub fn inputmode(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("inputmode", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("itemprop", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("lang", value));
        self
    }

    pub fn maxlength(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("maxlength", value));
        self
    }

    pub fn minlength(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("minlength", value));
        self
    }

    pub fn name(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("name", value));
        self
    }

    pub fn placeholder(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("placeholder", value));
        self
    }

    pub fn readonly(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("readonly", value));
        self
    }

    pub fn required(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("required", value));
        self
    }

    pub fn rows(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("rows", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("slot", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("spellcheck", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("style", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("tabindex", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("translate", value));
        self
    }

    pub fn wrap(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("wrap", value));
        self
    }

    pub fn child(mut self, element: impl Into<VTag>) -> Self {
        self.children.push(VNode::from(element.into()));
        self
    }

    pub fn component<C: Component>(mut self, props: C::Properties) -> Self {
        self.children.push(VNode::from(VComp::new::<C>(
            props,
            NodeRef::default(),
            None,
        )));
        self
    }

    pub fn text(mut self, text: impl Into<AttrValue>) -> Self {
        self.children.push(VNode::from(VText::new(text)));
        self
    }
}

impl Into<VTag> for Textarea {
    fn into(self) -> VTag {
        let mut vtag = VTag::new("textarea");
        vtag.attributes = Attributes::Vec(self.attributes);
        vtag.add_children(self.children.into_iter());
        vtag
    }
}

impl Into<VNode> for Textarea {
    fn into(self) -> VNode {
        let vtag: VTag = self.into();
        VNode::from(vtag)
    }
}

pub struct Details {
    attributes: Vec<PositionalAttr>,
    children: Vec<VNode>,
}

impl Details {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("accesskey", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("autocapitalize", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("class", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contextmenu", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("dir", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("draggable", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("hidden", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("id", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("itemprop", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("lang", value));
        self
    }

    pub fn open(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("open", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("slot", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("spellcheck", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("style", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("tabindex", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("translate", value));
        self
    }

    pub fn child(mut self, element: impl Into<VTag>) -> Self {
        self.children.push(VNode::from(element.into()));
        self
    }

    pub fn component<C: Component>(mut self, props: C::Properties) -> Self {
        self.children.push(VNode::from(VComp::new::<C>(
            props,
            NodeRef::default(),
            None,
        )));
        self
    }

    pub fn text(mut self, text: impl Into<AttrValue>) -> Self {
        self.children.push(VNode::from(VText::new(text)));
        self
    }
}

impl Into<VTag> for Details {
    fn into(self) -> VTag {
        let mut vtag = VTag::new("details");
        vtag.attributes = Attributes::Vec(self.attributes);
        vtag.add_children(self.children.into_iter());
        vtag
    }
}

impl Into<VNode> for Details {
    fn into(self) -> VNode {
        let vtag: VTag = self.into();
        VNode::from(vtag)
    }
}

pub struct Dialog {
    attributes: Vec<PositionalAttr>,
    children: Vec<VNode>,
}

impl Dialog {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("accesskey", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("autocapitalize", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("class", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contextmenu", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("dir", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("draggable", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("hidden", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("id", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("itemprop", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("lang", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("slot", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("spellcheck", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("style", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("tabindex", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("translate", value));
        self
    }

    pub fn child(mut self, element: impl Into<VTag>) -> Self {
        self.children.push(VNode::from(element.into()));
        self
    }

    pub fn component<C: Component>(mut self, props: C::Properties) -> Self {
        self.children.push(VNode::from(VComp::new::<C>(
            props,
            NodeRef::default(),
            None,
        )));
        self
    }

    pub fn text(mut self, text: impl Into<AttrValue>) -> Self {
        self.children.push(VNode::from(VText::new(text)));
        self
    }
}

impl Into<VTag> for Dialog {
    fn into(self) -> VTag {
        let mut vtag = VTag::new("dialog");
        vtag.attributes = Attributes::Vec(self.attributes);
        vtag.add_children(self.children.into_iter());
        vtag
    }
}

impl Into<VNode> for Dialog {
    fn into(self) -> VNode {
        let vtag: VTag = self.into();
        VNode::from(vtag)
    }
}

pub struct Menu {
    attributes: Vec<PositionalAttr>,
    children: Vec<VNode>,
}

impl Menu {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("accesskey", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("autocapitalize", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("class", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contextmenu", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("dir", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("draggable", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("hidden", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("id", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("itemprop", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("lang", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("slot", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("spellcheck", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("style", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("tabindex", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("translate", value));
        self
    }

    pub fn r#type(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("type", value));
        self
    }

    pub fn child(mut self, element: impl Into<VTag>) -> Self {
        self.children.push(VNode::from(element.into()));
        self
    }

    pub fn component<C: Component>(mut self, props: C::Properties) -> Self {
        self.children.push(VNode::from(VComp::new::<C>(
            props,
            NodeRef::default(),
            None,
        )));
        self
    }

    pub fn text(mut self, text: impl Into<AttrValue>) -> Self {
        self.children.push(VNode::from(VText::new(text)));
        self
    }
}

impl Into<VTag> for Menu {
    fn into(self) -> VTag {
        let mut vtag = VTag::new("menu");
        vtag.attributes = Attributes::Vec(self.attributes);
        vtag.add_children(self.children.into_iter());
        vtag
    }
}

impl Into<VNode> for Menu {
    fn into(self) -> VNode {
        let vtag: VTag = self.into();
        VNode::from(vtag)
    }
}

pub struct Summary {
    attributes: Vec<PositionalAttr>,
    children: Vec<VNode>,
}

impl Summary {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("accesskey", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("autocapitalize", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("class", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contextmenu", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("dir", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("draggable", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("hidden", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("id", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("itemprop", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("lang", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("slot", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("spellcheck", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("style", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("tabindex", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("translate", value));
        self
    }

    pub fn child(mut self, element: impl Into<VTag>) -> Self {
        self.children.push(VNode::from(element.into()));
        self
    }

    pub fn component<C: Component>(mut self, props: C::Properties) -> Self {
        self.children.push(VNode::from(VComp::new::<C>(
            props,
            NodeRef::default(),
            None,
        )));
        self
    }

    pub fn text(mut self, text: impl Into<AttrValue>) -> Self {
        self.children.push(VNode::from(VText::new(text)));
        self
    }
}

impl Into<VTag> for Summary {
    fn into(self) -> VTag {
        let mut vtag = VTag::new("summary");
        vtag.attributes = Attributes::Vec(self.attributes);
        vtag.add_children(self.children.into_iter());
        vtag
    }
}

impl Into<VNode> for Summary {
    fn into(self) -> VNode {
        let vtag: VTag = self.into();
        VNode::from(vtag)
    }
}

pub struct Slot {
    attributes: Vec<PositionalAttr>,
    children: Vec<VNode>,
}

impl Slot {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("accesskey", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("autocapitalize", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("class", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contextmenu", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("dir", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("draggable", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("hidden", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("id", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("itemprop", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("lang", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("slot", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("spellcheck", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("style", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("tabindex", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("translate", value));
        self
    }

    pub fn child(mut self, element: impl Into<VTag>) -> Self {
        self.children.push(VNode::from(element.into()));
        self
    }

    pub fn component<C: Component>(mut self, props: C::Properties) -> Self {
        self.children.push(VNode::from(VComp::new::<C>(
            props,
            NodeRef::default(),
            None,
        )));
        self
    }

    pub fn text(mut self, text: impl Into<AttrValue>) -> Self {
        self.children.push(VNode::from(VText::new(text)));
        self
    }
}

impl Into<VTag> for Slot {
    fn into(self) -> VTag {
        let mut vtag = VTag::new("slot");
        vtag.attributes = Attributes::Vec(self.attributes);
        vtag.add_children(self.children.into_iter());
        vtag
    }
}

impl Into<VNode> for Slot {
    fn into(self) -> VNode {
        let vtag: VTag = self.into();
        VNode::from(vtag)
    }
}

pub struct Template {
    attributes: Vec<PositionalAttr>,
    children: Vec<VNode>,
}

impl Template {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("accesskey", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("autocapitalize", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("class", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contextmenu", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("dir", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("draggable", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("hidden", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("id", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("itemprop", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("lang", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("slot", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("spellcheck", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("style", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("tabindex", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("translate", value));
        self
    }

    pub fn child(mut self, element: impl Into<VTag>) -> Self {
        self.children.push(VNode::from(element.into()));
        self
    }

    pub fn component<C: Component>(mut self, props: C::Properties) -> Self {
        self.children.push(VNode::from(VComp::new::<C>(
            props,
            NodeRef::default(),
            None,
        )));
        self
    }

    pub fn text(mut self, text: impl Into<AttrValue>) -> Self {
        self.children.push(VNode::from(VText::new(text)));
        self
    }
}

impl Into<VTag> for Template {
    fn into(self) -> VTag {
        let mut vtag = VTag::new("template");
        vtag.attributes = Attributes::Vec(self.attributes);
        vtag.add_children(self.children.into_iter());
        vtag
    }
}

impl Into<VNode> for Template {
    fn into(self) -> VNode {
        let vtag: VTag = self.into();
        VNode::from(vtag)
    }
}

pub struct Acronym {
    attributes: Vec<PositionalAttr>,
    children: Vec<VNode>,
}

impl Acronym {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("accesskey", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("autocapitalize", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("class", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contextmenu", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("dir", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("draggable", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("hidden", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("id", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("itemprop", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("lang", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("slot", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("spellcheck", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("style", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("tabindex", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("translate", value));
        self
    }

    pub fn child(mut self, element: impl Into<VTag>) -> Self {
        self.children.push(VNode::from(element.into()));
        self
    }

    pub fn component<C: Component>(mut self, props: C::Properties) -> Self {
        self.children.push(VNode::from(VComp::new::<C>(
            props,
            NodeRef::default(),
            None,
        )));
        self
    }

    pub fn text(mut self, text: impl Into<AttrValue>) -> Self {
        self.children.push(VNode::from(VText::new(text)));
        self
    }
}

impl Into<VTag> for Acronym {
    fn into(self) -> VTag {
        let mut vtag = VTag::new("acronym");
        vtag.attributes = Attributes::Vec(self.attributes);
        vtag.add_children(self.children.into_iter());
        vtag
    }
}

impl Into<VNode> for Acronym {
    fn into(self) -> VNode {
        let vtag: VTag = self.into();
        VNode::from(vtag)
    }
}

pub struct Applet {
    attributes: Vec<PositionalAttr>,
    children: Vec<VNode>,
}

impl Applet {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("accesskey", value));
        self
    }

    pub fn align(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("align", value));
        self
    }

    pub fn alt(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("alt", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("autocapitalize", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("class", value));
        self
    }

    pub fn code(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("code", value));
        self
    }

    pub fn codebase(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("codebase", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contextmenu", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("dir", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("draggable", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("hidden", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("id", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("itemprop", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("lang", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("slot", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("spellcheck", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("style", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("tabindex", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("translate", value));
        self
    }

    pub fn child(mut self, element: impl Into<VTag>) -> Self {
        self.children.push(VNode::from(element.into()));
        self
    }

    pub fn component<C: Component>(mut self, props: C::Properties) -> Self {
        self.children.push(VNode::from(VComp::new::<C>(
            props,
            NodeRef::default(),
            None,
        )));
        self
    }

    pub fn text(mut self, text: impl Into<AttrValue>) -> Self {
        self.children.push(VNode::from(VText::new(text)));
        self
    }
}

impl Into<VTag> for Applet {
    fn into(self) -> VTag {
        let mut vtag = VTag::new("applet");
        vtag.attributes = Attributes::Vec(self.attributes);
        vtag.add_children(self.children.into_iter());
        vtag
    }
}

impl Into<VNode> for Applet {
    fn into(self) -> VNode {
        let vtag: VTag = self.into();
        VNode::from(vtag)
    }
}

pub struct Basefont {
    attributes: Vec<PositionalAttr>,
    children: Vec<VNode>,
}

impl Basefont {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("accesskey", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("autocapitalize", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("class", value));
        self
    }

    pub fn color(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("color", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contextmenu", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("dir", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("draggable", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("hidden", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("id", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("itemprop", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("lang", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("slot", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("spellcheck", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("style", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("tabindex", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("translate", value));
        self
    }

    pub fn child(mut self, element: impl Into<VTag>) -> Self {
        self.children.push(VNode::from(element.into()));
        self
    }

    pub fn component<C: Component>(mut self, props: C::Properties) -> Self {
        self.children.push(VNode::from(VComp::new::<C>(
            props,
            NodeRef::default(),
            None,
        )));
        self
    }

    pub fn text(mut self, text: impl Into<AttrValue>) -> Self {
        self.children.push(VNode::from(VText::new(text)));
        self
    }
}

impl Into<VTag> for Basefont {
    fn into(self) -> VTag {
        let mut vtag = VTag::new("basefont");
        vtag.attributes = Attributes::Vec(self.attributes);
        vtag.add_children(self.children.into_iter());
        vtag
    }
}

impl Into<VNode> for Basefont {
    fn into(self) -> VNode {
        let vtag: VTag = self.into();
        VNode::from(vtag)
    }
}

pub struct Bgsound {
    attributes: Vec<PositionalAttr>,
    children: Vec<VNode>,
}

impl Bgsound {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("accesskey", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("autocapitalize", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("class", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contextmenu", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("dir", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("draggable", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("hidden", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("id", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("itemprop", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("lang", value));
        self
    }

    pub fn r#loop(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("loop", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("slot", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("spellcheck", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("style", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("tabindex", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("translate", value));
        self
    }

    pub fn child(mut self, element: impl Into<VTag>) -> Self {
        self.children.push(VNode::from(element.into()));
        self
    }

    pub fn component<C: Component>(mut self, props: C::Properties) -> Self {
        self.children.push(VNode::from(VComp::new::<C>(
            props,
            NodeRef::default(),
            None,
        )));
        self
    }

    pub fn text(mut self, text: impl Into<AttrValue>) -> Self {
        self.children.push(VNode::from(VText::new(text)));
        self
    }
}

impl Into<VTag> for Bgsound {
    fn into(self) -> VTag {
        let mut vtag = VTag::new("bgsound");
        vtag.attributes = Attributes::Vec(self.attributes);
        vtag.add_children(self.children.into_iter());
        vtag
    }
}

impl Into<VNode> for Bgsound {
    fn into(self) -> VNode {
        let vtag: VTag = self.into();
        VNode::from(vtag)
    }
}

pub struct Big {
    attributes: Vec<PositionalAttr>,
    children: Vec<VNode>,
}

impl Big {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("accesskey", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("autocapitalize", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("class", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contextmenu", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("dir", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("draggable", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("hidden", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("id", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("itemprop", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("lang", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("slot", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("spellcheck", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("style", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("tabindex", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("translate", value));
        self
    }

    pub fn child(mut self, element: impl Into<VTag>) -> Self {
        self.children.push(VNode::from(element.into()));
        self
    }

    pub fn component<C: Component>(mut self, props: C::Properties) -> Self {
        self.children.push(VNode::from(VComp::new::<C>(
            props,
            NodeRef::default(),
            None,
        )));
        self
    }

    pub fn text(mut self, text: impl Into<AttrValue>) -> Self {
        self.children.push(VNode::from(VText::new(text)));
        self
    }
}

impl Into<VTag> for Big {
    fn into(self) -> VTag {
        let mut vtag = VTag::new("big");
        vtag.attributes = Attributes::Vec(self.attributes);
        vtag.add_children(self.children.into_iter());
        vtag
    }
}

impl Into<VNode> for Big {
    fn into(self) -> VNode {
        let vtag: VTag = self.into();
        VNode::from(vtag)
    }
}

pub struct Blink {
    attributes: Vec<PositionalAttr>,
    children: Vec<VNode>,
}

impl Blink {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("accesskey", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("autocapitalize", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("class", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contextmenu", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("dir", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("draggable", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("hidden", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("id", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("itemprop", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("lang", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("slot", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("spellcheck", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("style", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("tabindex", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("translate", value));
        self
    }

    pub fn child(mut self, element: impl Into<VTag>) -> Self {
        self.children.push(VNode::from(element.into()));
        self
    }

    pub fn component<C: Component>(mut self, props: C::Properties) -> Self {
        self.children.push(VNode::from(VComp::new::<C>(
            props,
            NodeRef::default(),
            None,
        )));
        self
    }

    pub fn text(mut self, text: impl Into<AttrValue>) -> Self {
        self.children.push(VNode::from(VText::new(text)));
        self
    }
}

impl Into<VTag> for Blink {
    fn into(self) -> VTag {
        let mut vtag = VTag::new("blink");
        vtag.attributes = Attributes::Vec(self.attributes);
        vtag.add_children(self.children.into_iter());
        vtag
    }
}

impl Into<VNode> for Blink {
    fn into(self) -> VNode {
        let vtag: VTag = self.into();
        VNode::from(vtag)
    }
}

pub struct Center {
    attributes: Vec<PositionalAttr>,
    children: Vec<VNode>,
}

impl Center {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("accesskey", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("autocapitalize", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("class", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contextmenu", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("dir", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("draggable", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("hidden", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("id", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("itemprop", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("lang", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("slot", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("spellcheck", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("style", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("tabindex", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("translate", value));
        self
    }

    pub fn child(mut self, element: impl Into<VTag>) -> Self {
        self.children.push(VNode::from(element.into()));
        self
    }

    pub fn component<C: Component>(mut self, props: C::Properties) -> Self {
        self.children.push(VNode::from(VComp::new::<C>(
            props,
            NodeRef::default(),
            None,
        )));
        self
    }

    pub fn text(mut self, text: impl Into<AttrValue>) -> Self {
        self.children.push(VNode::from(VText::new(text)));
        self
    }
}

impl Into<VTag> for Center {
    fn into(self) -> VTag {
        let mut vtag = VTag::new("center");
        vtag.attributes = Attributes::Vec(self.attributes);
        vtag.add_children(self.children.into_iter());
        vtag
    }
}

impl Into<VNode> for Center {
    fn into(self) -> VNode {
        let vtag: VTag = self.into();
        VNode::from(vtag)
    }
}

pub struct Content {
    attributes: Vec<PositionalAttr>,
    children: Vec<VNode>,
}

impl Content {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("accesskey", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("autocapitalize", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("class", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contextmenu", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("dir", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("draggable", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("hidden", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("id", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("itemprop", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("lang", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("slot", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("spellcheck", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("style", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("tabindex", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("translate", value));
        self
    }

    pub fn child(mut self, element: impl Into<VTag>) -> Self {
        self.children.push(VNode::from(element.into()));
        self
    }

    pub fn component<C: Component>(mut self, props: C::Properties) -> Self {
        self.children.push(VNode::from(VComp::new::<C>(
            props,
            NodeRef::default(),
            None,
        )));
        self
    }

    pub fn text(mut self, text: impl Into<AttrValue>) -> Self {
        self.children.push(VNode::from(VText::new(text)));
        self
    }
}

impl Into<VTag> for Content {
    fn into(self) -> VTag {
        let mut vtag = VTag::new("content");
        vtag.attributes = Attributes::Vec(self.attributes);
        vtag.add_children(self.children.into_iter());
        vtag
    }
}

impl Into<VNode> for Content {
    fn into(self) -> VNode {
        let vtag: VTag = self.into();
        VNode::from(vtag)
    }
}

pub struct Dir {
    attributes: Vec<PositionalAttr>,
    children: Vec<VNode>,
}

impl Dir {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("accesskey", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("autocapitalize", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("class", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contextmenu", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("dir", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("draggable", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("hidden", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("id", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("itemprop", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("lang", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("slot", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("spellcheck", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("style", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("tabindex", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("translate", value));
        self
    }

    pub fn child(mut self, element: impl Into<VTag>) -> Self {
        self.children.push(VNode::from(element.into()));
        self
    }

    pub fn component<C: Component>(mut self, props: C::Properties) -> Self {
        self.children.push(VNode::from(VComp::new::<C>(
            props,
            NodeRef::default(),
            None,
        )));
        self
    }

    pub fn text(mut self, text: impl Into<AttrValue>) -> Self {
        self.children.push(VNode::from(VText::new(text)));
        self
    }
}

impl Into<VTag> for Dir {
    fn into(self) -> VTag {
        let mut vtag = VTag::new("dir");
        vtag.attributes = Attributes::Vec(self.attributes);
        vtag.add_children(self.children.into_iter());
        vtag
    }
}

impl Into<VNode> for Dir {
    fn into(self) -> VNode {
        let vtag: VTag = self.into();
        VNode::from(vtag)
    }
}

pub struct Font {
    attributes: Vec<PositionalAttr>,
    children: Vec<VNode>,
}

impl Font {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("accesskey", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("autocapitalize", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("class", value));
        self
    }

    pub fn color(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("color", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contextmenu", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("dir", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("draggable", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("hidden", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("id", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("itemprop", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("lang", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("slot", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("spellcheck", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("style", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("tabindex", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("translate", value));
        self
    }

    pub fn child(mut self, element: impl Into<VTag>) -> Self {
        self.children.push(VNode::from(element.into()));
        self
    }

    pub fn component<C: Component>(mut self, props: C::Properties) -> Self {
        self.children.push(VNode::from(VComp::new::<C>(
            props,
            NodeRef::default(),
            None,
        )));
        self
    }

    pub fn text(mut self, text: impl Into<AttrValue>) -> Self {
        self.children.push(VNode::from(VText::new(text)));
        self
    }
}

impl Into<VTag> for Font {
    fn into(self) -> VTag {
        let mut vtag = VTag::new("font");
        vtag.attributes = Attributes::Vec(self.attributes);
        vtag.add_children(self.children.into_iter());
        vtag
    }
}

impl Into<VNode> for Font {
    fn into(self) -> VNode {
        let vtag: VTag = self.into();
        VNode::from(vtag)
    }
}

pub struct Frame {
    attributes: Vec<PositionalAttr>,
    children: Vec<VNode>,
}

impl Frame {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("accesskey", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("autocapitalize", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("class", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contextmenu", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("dir", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("draggable", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("hidden", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("id", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("itemprop", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("lang", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("slot", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("spellcheck", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("style", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("tabindex", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("translate", value));
        self
    }

    pub fn child(mut self, element: impl Into<VTag>) -> Self {
        self.children.push(VNode::from(element.into()));
        self
    }

    pub fn component<C: Component>(mut self, props: C::Properties) -> Self {
        self.children.push(VNode::from(VComp::new::<C>(
            props,
            NodeRef::default(),
            None,
        )));
        self
    }

    pub fn text(mut self, text: impl Into<AttrValue>) -> Self {
        self.children.push(VNode::from(VText::new(text)));
        self
    }
}

impl Into<VTag> for Frame {
    fn into(self) -> VTag {
        let mut vtag = VTag::new("frame");
        vtag.attributes = Attributes::Vec(self.attributes);
        vtag.add_children(self.children.into_iter());
        vtag
    }
}

impl Into<VNode> for Frame {
    fn into(self) -> VNode {
        let vtag: VTag = self.into();
        VNode::from(vtag)
    }
}

pub struct Frameset {
    attributes: Vec<PositionalAttr>,
    children: Vec<VNode>,
}

impl Frameset {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("accesskey", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("autocapitalize", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("class", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contextmenu", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("dir", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("draggable", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("hidden", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("id", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("itemprop", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("lang", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("slot", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("spellcheck", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("style", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("tabindex", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("translate", value));
        self
    }

    pub fn child(mut self, element: impl Into<VTag>) -> Self {
        self.children.push(VNode::from(element.into()));
        self
    }

    pub fn component<C: Component>(mut self, props: C::Properties) -> Self {
        self.children.push(VNode::from(VComp::new::<C>(
            props,
            NodeRef::default(),
            None,
        )));
        self
    }

    pub fn text(mut self, text: impl Into<AttrValue>) -> Self {
        self.children.push(VNode::from(VText::new(text)));
        self
    }
}

impl Into<VTag> for Frameset {
    fn into(self) -> VTag {
        let mut vtag = VTag::new("frameset");
        vtag.attributes = Attributes::Vec(self.attributes);
        vtag.add_children(self.children.into_iter());
        vtag
    }
}

impl Into<VNode> for Frameset {
    fn into(self) -> VNode {
        let vtag: VTag = self.into();
        VNode::from(vtag)
    }
}

pub struct Hgroup {
    attributes: Vec<PositionalAttr>,
    children: Vec<VNode>,
}

impl Hgroup {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("accesskey", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("autocapitalize", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("class", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contextmenu", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("dir", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("draggable", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("hidden", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("id", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("itemprop", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("lang", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("slot", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("spellcheck", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("style", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("tabindex", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("translate", value));
        self
    }

    pub fn child(mut self, element: impl Into<VTag>) -> Self {
        self.children.push(VNode::from(element.into()));
        self
    }

    pub fn component<C: Component>(mut self, props: C::Properties) -> Self {
        self.children.push(VNode::from(VComp::new::<C>(
            props,
            NodeRef::default(),
            None,
        )));
        self
    }

    pub fn text(mut self, text: impl Into<AttrValue>) -> Self {
        self.children.push(VNode::from(VText::new(text)));
        self
    }
}

impl Into<VTag> for Hgroup {
    fn into(self) -> VTag {
        let mut vtag = VTag::new("hgroup");
        vtag.attributes = Attributes::Vec(self.attributes);
        vtag.add_children(self.children.into_iter());
        vtag
    }
}

impl Into<VNode> for Hgroup {
    fn into(self) -> VNode {
        let vtag: VTag = self.into();
        VNode::from(vtag)
    }
}

pub struct Image {
    attributes: Vec<PositionalAttr>,
    children: Vec<VNode>,
}

impl Image {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("accesskey", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("autocapitalize", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("class", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contextmenu", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("dir", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("draggable", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("hidden", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("id", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("itemprop", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("lang", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("slot", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("spellcheck", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("style", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("tabindex", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("translate", value));
        self
    }

    pub fn child(mut self, element: impl Into<VTag>) -> Self {
        self.children.push(VNode::from(element.into()));
        self
    }

    pub fn component<C: Component>(mut self, props: C::Properties) -> Self {
        self.children.push(VNode::from(VComp::new::<C>(
            props,
            NodeRef::default(),
            None,
        )));
        self
    }

    pub fn text(mut self, text: impl Into<AttrValue>) -> Self {
        self.children.push(VNode::from(VText::new(text)));
        self
    }
}

impl Into<VTag> for Image {
    fn into(self) -> VTag {
        let mut vtag = VTag::new("image");
        vtag.attributes = Attributes::Vec(self.attributes);
        vtag.add_children(self.children.into_iter());
        vtag
    }
}

impl Into<VNode> for Image {
    fn into(self) -> VNode {
        let vtag: VTag = self.into();
        VNode::from(vtag)
    }
}

pub struct Keygen {
    attributes: Vec<PositionalAttr>,
    children: Vec<VNode>,
}

impl Keygen {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("accesskey", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("autocapitalize", value));
        self
    }

    pub fn autofocus(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("autofocus", value));
        self
    }

    pub fn challenge(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("challenge", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("class", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contextmenu", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("dir", value));
        self
    }

    pub fn disabled(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("disabled", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("draggable", value));
        self
    }

    pub fn form(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("form", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("hidden", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("id", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("itemprop", value));
        self
    }

    pub fn keytype(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("keytype", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("lang", value));
        self
    }

    pub fn name(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("name", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("slot", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("spellcheck", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("style", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("tabindex", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("translate", value));
        self
    }

    pub fn child(mut self, element: impl Into<VTag>) -> Self {
        self.children.push(VNode::from(element.into()));
        self
    }

    pub fn component<C: Component>(mut self, props: C::Properties) -> Self {
        self.children.push(VNode::from(VComp::new::<C>(
            props,
            NodeRef::default(),
            None,
        )));
        self
    }

    pub fn text(mut self, text: impl Into<AttrValue>) -> Self {
        self.children.push(VNode::from(VText::new(text)));
        self
    }
}

impl Into<VTag> for Keygen {
    fn into(self) -> VTag {
        let mut vtag = VTag::new("keygen");
        vtag.attributes = Attributes::Vec(self.attributes);
        vtag.add_children(self.children.into_iter());
        vtag
    }
}

impl Into<VNode> for Keygen {
    fn into(self) -> VNode {
        let vtag: VTag = self.into();
        VNode::from(vtag)
    }
}

pub struct Marquee {
    attributes: Vec<PositionalAttr>,
    children: Vec<VNode>,
}

impl Marquee {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("accesskey", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("autocapitalize", value));
        self
    }

    pub fn bgcolor(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("bgcolor", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("class", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contextmenu", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("dir", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("draggable", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("hidden", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("id", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("itemprop", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("lang", value));
        self
    }

    pub fn r#loop(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("loop", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("slot", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("spellcheck", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("style", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("tabindex", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("translate", value));
        self
    }

    pub fn child(mut self, element: impl Into<VTag>) -> Self {
        self.children.push(VNode::from(element.into()));
        self
    }

    pub fn component<C: Component>(mut self, props: C::Properties) -> Self {
        self.children.push(VNode::from(VComp::new::<C>(
            props,
            NodeRef::default(),
            None,
        )));
        self
    }

    pub fn text(mut self, text: impl Into<AttrValue>) -> Self {
        self.children.push(VNode::from(VText::new(text)));
        self
    }
}

impl Into<VTag> for Marquee {
    fn into(self) -> VTag {
        let mut vtag = VTag::new("marquee");
        vtag.attributes = Attributes::Vec(self.attributes);
        vtag.add_children(self.children.into_iter());
        vtag
    }
}

impl Into<VNode> for Marquee {
    fn into(self) -> VNode {
        let vtag: VTag = self.into();
        VNode::from(vtag)
    }
}

pub struct Menuitem {
    attributes: Vec<PositionalAttr>,
    children: Vec<VNode>,
}

impl Menuitem {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("accesskey", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("autocapitalize", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("class", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contextmenu", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("dir", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("draggable", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("hidden", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("id", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("itemprop", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("lang", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("slot", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("spellcheck", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("style", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("tabindex", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("translate", value));
        self
    }

    pub fn child(mut self, element: impl Into<VTag>) -> Self {
        self.children.push(VNode::from(element.into()));
        self
    }

    pub fn component<C: Component>(mut self, props: C::Properties) -> Self {
        self.children.push(VNode::from(VComp::new::<C>(
            props,
            NodeRef::default(),
            None,
        )));
        self
    }

    pub fn text(mut self, text: impl Into<AttrValue>) -> Self {
        self.children.push(VNode::from(VText::new(text)));
        self
    }
}

impl Into<VTag> for Menuitem {
    fn into(self) -> VTag {
        let mut vtag = VTag::new("menuitem");
        vtag.attributes = Attributes::Vec(self.attributes);
        vtag.add_children(self.children.into_iter());
        vtag
    }
}

impl Into<VNode> for Menuitem {
    fn into(self) -> VNode {
        let vtag: VTag = self.into();
        VNode::from(vtag)
    }
}

pub struct Nobr {
    attributes: Vec<PositionalAttr>,
    children: Vec<VNode>,
}

impl Nobr {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("accesskey", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("autocapitalize", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("class", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contextmenu", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("dir", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("draggable", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("hidden", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("id", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("itemprop", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("lang", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("slot", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("spellcheck", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("style", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("tabindex", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("translate", value));
        self
    }

    pub fn child(mut self, element: impl Into<VTag>) -> Self {
        self.children.push(VNode::from(element.into()));
        self
    }

    pub fn component<C: Component>(mut self, props: C::Properties) -> Self {
        self.children.push(VNode::from(VComp::new::<C>(
            props,
            NodeRef::default(),
            None,
        )));
        self
    }

    pub fn text(mut self, text: impl Into<AttrValue>) -> Self {
        self.children.push(VNode::from(VText::new(text)));
        self
    }
}

impl Into<VTag> for Nobr {
    fn into(self) -> VTag {
        let mut vtag = VTag::new("nobr");
        vtag.attributes = Attributes::Vec(self.attributes);
        vtag.add_children(self.children.into_iter());
        vtag
    }
}

impl Into<VNode> for Nobr {
    fn into(self) -> VNode {
        let vtag: VTag = self.into();
        VNode::from(vtag)
    }
}

pub struct Noembed {
    attributes: Vec<PositionalAttr>,
    children: Vec<VNode>,
}

impl Noembed {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("accesskey", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("autocapitalize", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("class", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contextmenu", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("dir", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("draggable", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("hidden", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("id", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("itemprop", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("lang", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("slot", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("spellcheck", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("style", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("tabindex", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("translate", value));
        self
    }

    pub fn child(mut self, element: impl Into<VTag>) -> Self {
        self.children.push(VNode::from(element.into()));
        self
    }

    pub fn component<C: Component>(mut self, props: C::Properties) -> Self {
        self.children.push(VNode::from(VComp::new::<C>(
            props,
            NodeRef::default(),
            None,
        )));
        self
    }

    pub fn text(mut self, text: impl Into<AttrValue>) -> Self {
        self.children.push(VNode::from(VText::new(text)));
        self
    }
}

impl Into<VTag> for Noembed {
    fn into(self) -> VTag {
        let mut vtag = VTag::new("noembed");
        vtag.attributes = Attributes::Vec(self.attributes);
        vtag.add_children(self.children.into_iter());
        vtag
    }
}

impl Into<VNode> for Noembed {
    fn into(self) -> VNode {
        let vtag: VTag = self.into();
        VNode::from(vtag)
    }
}

pub struct Noframes {
    attributes: Vec<PositionalAttr>,
    children: Vec<VNode>,
}

impl Noframes {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("accesskey", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("autocapitalize", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("class", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contextmenu", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("dir", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("draggable", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("hidden", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("id", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("itemprop", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("lang", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("slot", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("spellcheck", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("style", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("tabindex", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("translate", value));
        self
    }

    pub fn child(mut self, element: impl Into<VTag>) -> Self {
        self.children.push(VNode::from(element.into()));
        self
    }

    pub fn component<C: Component>(mut self, props: C::Properties) -> Self {
        self.children.push(VNode::from(VComp::new::<C>(
            props,
            NodeRef::default(),
            None,
        )));
        self
    }

    pub fn text(mut self, text: impl Into<AttrValue>) -> Self {
        self.children.push(VNode::from(VText::new(text)));
        self
    }
}

impl Into<VTag> for Noframes {
    fn into(self) -> VTag {
        let mut vtag = VTag::new("noframes");
        vtag.attributes = Attributes::Vec(self.attributes);
        vtag.add_children(self.children.into_iter());
        vtag
    }
}

impl Into<VNode> for Noframes {
    fn into(self) -> VNode {
        let vtag: VTag = self.into();
        VNode::from(vtag)
    }
}

pub struct Plaintext {
    attributes: Vec<PositionalAttr>,
    children: Vec<VNode>,
}

impl Plaintext {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("accesskey", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("autocapitalize", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("class", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contextmenu", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("dir", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("draggable", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("hidden", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("id", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("itemprop", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("lang", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("slot", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("spellcheck", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("style", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("tabindex", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("translate", value));
        self
    }

    pub fn child(mut self, element: impl Into<VTag>) -> Self {
        self.children.push(VNode::from(element.into()));
        self
    }

    pub fn component<C: Component>(mut self, props: C::Properties) -> Self {
        self.children.push(VNode::from(VComp::new::<C>(
            props,
            NodeRef::default(),
            None,
        )));
        self
    }

    pub fn text(mut self, text: impl Into<AttrValue>) -> Self {
        self.children.push(VNode::from(VText::new(text)));
        self
    }
}

impl Into<VTag> for Plaintext {
    fn into(self) -> VTag {
        let mut vtag = VTag::new("plaintext");
        vtag.attributes = Attributes::Vec(self.attributes);
        vtag.add_children(self.children.into_iter());
        vtag
    }
}

impl Into<VNode> for Plaintext {
    fn into(self) -> VNode {
        let vtag: VTag = self.into();
        VNode::from(vtag)
    }
}

pub struct Rb {
    attributes: Vec<PositionalAttr>,
    children: Vec<VNode>,
}

impl Rb {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("accesskey", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("autocapitalize", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("class", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contextmenu", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("dir", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("draggable", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("hidden", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("id", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("itemprop", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("lang", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("slot", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("spellcheck", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("style", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("tabindex", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("translate", value));
        self
    }

    pub fn child(mut self, element: impl Into<VTag>) -> Self {
        self.children.push(VNode::from(element.into()));
        self
    }

    pub fn component<C: Component>(mut self, props: C::Properties) -> Self {
        self.children.push(VNode::from(VComp::new::<C>(
            props,
            NodeRef::default(),
            None,
        )));
        self
    }

    pub fn text(mut self, text: impl Into<AttrValue>) -> Self {
        self.children.push(VNode::from(VText::new(text)));
        self
    }
}

impl Into<VTag> for Rb {
    fn into(self) -> VTag {
        let mut vtag = VTag::new("rb");
        vtag.attributes = Attributes::Vec(self.attributes);
        vtag.add_children(self.children.into_iter());
        vtag
    }
}

impl Into<VNode> for Rb {
    fn into(self) -> VNode {
        let vtag: VTag = self.into();
        VNode::from(vtag)
    }
}

pub struct Rtc {
    attributes: Vec<PositionalAttr>,
    children: Vec<VNode>,
}

impl Rtc {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("accesskey", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("autocapitalize", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("class", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contextmenu", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("dir", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("draggable", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("hidden", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("id", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("itemprop", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("lang", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("slot", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("spellcheck", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("style", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("tabindex", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("translate", value));
        self
    }

    pub fn child(mut self, element: impl Into<VTag>) -> Self {
        self.children.push(VNode::from(element.into()));
        self
    }

    pub fn component<C: Component>(mut self, props: C::Properties) -> Self {
        self.children.push(VNode::from(VComp::new::<C>(
            props,
            NodeRef::default(),
            None,
        )));
        self
    }

    pub fn text(mut self, text: impl Into<AttrValue>) -> Self {
        self.children.push(VNode::from(VText::new(text)));
        self
    }
}

impl Into<VTag> for Rtc {
    fn into(self) -> VTag {
        let mut vtag = VTag::new("rtc");
        vtag.attributes = Attributes::Vec(self.attributes);
        vtag.add_children(self.children.into_iter());
        vtag
    }
}

impl Into<VNode> for Rtc {
    fn into(self) -> VNode {
        let vtag: VTag = self.into();
        VNode::from(vtag)
    }
}

pub struct Shadow {
    attributes: Vec<PositionalAttr>,
    children: Vec<VNode>,
}

impl Shadow {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("accesskey", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("autocapitalize", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("class", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contextmenu", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("dir", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("draggable", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("hidden", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("id", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("itemprop", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("lang", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("slot", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("spellcheck", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("style", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("tabindex", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("translate", value));
        self
    }

    pub fn child(mut self, element: impl Into<VTag>) -> Self {
        self.children.push(VNode::from(element.into()));
        self
    }

    pub fn component<C: Component>(mut self, props: C::Properties) -> Self {
        self.children.push(VNode::from(VComp::new::<C>(
            props,
            NodeRef::default(),
            None,
        )));
        self
    }

    pub fn text(mut self, text: impl Into<AttrValue>) -> Self {
        self.children.push(VNode::from(VText::new(text)));
        self
    }
}

impl Into<VTag> for Shadow {
    fn into(self) -> VTag {
        let mut vtag = VTag::new("shadow");
        vtag.attributes = Attributes::Vec(self.attributes);
        vtag.add_children(self.children.into_iter());
        vtag
    }
}

impl Into<VNode> for Shadow {
    fn into(self) -> VNode {
        let vtag: VTag = self.into();
        VNode::from(vtag)
    }
}

pub struct Spacer {
    attributes: Vec<PositionalAttr>,
    children: Vec<VNode>,
}

impl Spacer {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("accesskey", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("autocapitalize", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("class", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contextmenu", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("dir", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("draggable", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("hidden", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("id", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("itemprop", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("lang", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("slot", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("spellcheck", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("style", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("tabindex", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("translate", value));
        self
    }

    pub fn child(mut self, element: impl Into<VTag>) -> Self {
        self.children.push(VNode::from(element.into()));
        self
    }

    pub fn component<C: Component>(mut self, props: C::Properties) -> Self {
        self.children.push(VNode::from(VComp::new::<C>(
            props,
            NodeRef::default(),
            None,
        )));
        self
    }

    pub fn text(mut self, text: impl Into<AttrValue>) -> Self {
        self.children.push(VNode::from(VText::new(text)));
        self
    }
}

impl Into<VTag> for Spacer {
    fn into(self) -> VTag {
        let mut vtag = VTag::new("spacer");
        vtag.attributes = Attributes::Vec(self.attributes);
        vtag.add_children(self.children.into_iter());
        vtag
    }
}

impl Into<VNode> for Spacer {
    fn into(self) -> VNode {
        let vtag: VTag = self.into();
        VNode::from(vtag)
    }
}

pub struct Strike {
    attributes: Vec<PositionalAttr>,
    children: Vec<VNode>,
}

impl Strike {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("accesskey", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("autocapitalize", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("class", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contextmenu", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("dir", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("draggable", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("hidden", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("id", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("itemprop", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("lang", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("slot", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("spellcheck", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("style", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("tabindex", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("translate", value));
        self
    }

    pub fn child(mut self, element: impl Into<VTag>) -> Self {
        self.children.push(VNode::from(element.into()));
        self
    }

    pub fn component<C: Component>(mut self, props: C::Properties) -> Self {
        self.children.push(VNode::from(VComp::new::<C>(
            props,
            NodeRef::default(),
            None,
        )));
        self
    }

    pub fn text(mut self, text: impl Into<AttrValue>) -> Self {
        self.children.push(VNode::from(VText::new(text)));
        self
    }
}

impl Into<VTag> for Strike {
    fn into(self) -> VTag {
        let mut vtag = VTag::new("strike");
        vtag.attributes = Attributes::Vec(self.attributes);
        vtag.add_children(self.children.into_iter());
        vtag
    }
}

impl Into<VNode> for Strike {
    fn into(self) -> VNode {
        let vtag: VTag = self.into();
        VNode::from(vtag)
    }
}

pub struct Tt {
    attributes: Vec<PositionalAttr>,
    children: Vec<VNode>,
}

impl Tt {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("accesskey", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("autocapitalize", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("class", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contextmenu", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("dir", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("draggable", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("hidden", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("id", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("itemprop", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("lang", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("slot", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("spellcheck", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("style", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("tabindex", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("translate", value));
        self
    }

    pub fn child(mut self, element: impl Into<VTag>) -> Self {
        self.children.push(VNode::from(element.into()));
        self
    }

    pub fn component<C: Component>(mut self, props: C::Properties) -> Self {
        self.children.push(VNode::from(VComp::new::<C>(
            props,
            NodeRef::default(),
            None,
        )));
        self
    }

    pub fn text(mut self, text: impl Into<AttrValue>) -> Self {
        self.children.push(VNode::from(VText::new(text)));
        self
    }
}

impl Into<VTag> for Tt {
    fn into(self) -> VTag {
        let mut vtag = VTag::new("tt");
        vtag.attributes = Attributes::Vec(self.attributes);
        vtag.add_children(self.children.into_iter());
        vtag
    }
}

impl Into<VNode> for Tt {
    fn into(self) -> VNode {
        let vtag: VTag = self.into();
        VNode::from(vtag)
    }
}

pub struct Xmp {
    attributes: Vec<PositionalAttr>,
    children: Vec<VNode>,
}

impl Xmp {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("accesskey", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("autocapitalize", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("class", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contextmenu", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("dir", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("draggable", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("hidden", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("id", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("itemprop", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("lang", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("slot", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("spellcheck", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("style", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("tabindex", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("translate", value));
        self
    }

    pub fn child(mut self, element: impl Into<VTag>) -> Self {
        self.children.push(VNode::from(element.into()));
        self
    }

    pub fn component<C: Component>(mut self, props: C::Properties) -> Self {
        self.children.push(VNode::from(VComp::new::<C>(
            props,
            NodeRef::default(),
            None,
        )));
        self
    }

    pub fn text(mut self, text: impl Into<AttrValue>) -> Self {
        self.children.push(VNode::from(VText::new(text)));
        self
    }
}

impl Into<VTag> for Xmp {
    fn into(self) -> VTag {
        let mut vtag = VTag::new("xmp");
        vtag.attributes = Attributes::Vec(self.attributes);
        vtag.add_children(self.children.into_iter());
        vtag
    }
}

impl Into<VNode> for Xmp {
    fn into(self) -> VNode {
        let vtag: VTag = self.into();
        VNode::from(vtag)
    }
}

pub struct H1 {
    attributes: Vec<PositionalAttr>,
    children: Vec<VNode>,
}

impl H1 {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("accesskey", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("autocapitalize", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("class", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contextmenu", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("dir", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("draggable", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("hidden", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("id", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("itemprop", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("lang", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("slot", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("spellcheck", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("style", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("tabindex", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("translate", value));
        self
    }

    pub fn child(mut self, element: impl Into<VTag>) -> Self {
        self.children.push(VNode::from(element.into()));
        self
    }

    pub fn component<C: Component>(mut self, props: C::Properties) -> Self {
        self.children.push(VNode::from(VComp::new::<C>(
            props,
            NodeRef::default(),
            None,
        )));
        self
    }

    pub fn text(mut self, text: impl Into<AttrValue>) -> Self {
        self.children.push(VNode::from(VText::new(text)));
        self
    }
}

impl Into<VTag> for H1 {
    fn into(self) -> VTag {
        let mut vtag = VTag::new("h1");
        vtag.attributes = Attributes::Vec(self.attributes);
        vtag.add_children(self.children.into_iter());
        vtag
    }
}

impl Into<VNode> for H1 {
    fn into(self) -> VNode {
        let vtag: VTag = self.into();
        VNode::from(vtag)
    }
}

pub struct H2 {
    attributes: Vec<PositionalAttr>,
    children: Vec<VNode>,
}

impl H2 {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("accesskey", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("autocapitalize", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("class", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contextmenu", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("dir", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("draggable", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("hidden", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("id", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("itemprop", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("lang", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("slot", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("spellcheck", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("style", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("tabindex", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("translate", value));
        self
    }

    pub fn child(mut self, element: impl Into<VTag>) -> Self {
        self.children.push(VNode::from(element.into()));
        self
    }

    pub fn component<C: Component>(mut self, props: C::Properties) -> Self {
        self.children.push(VNode::from(VComp::new::<C>(
            props,
            NodeRef::default(),
            None,
        )));
        self
    }

    pub fn text(mut self, text: impl Into<AttrValue>) -> Self {
        self.children.push(VNode::from(VText::new(text)));
        self
    }
}

impl Into<VTag> for H2 {
    fn into(self) -> VTag {
        let mut vtag = VTag::new("h2");
        vtag.attributes = Attributes::Vec(self.attributes);
        vtag.add_children(self.children.into_iter());
        vtag
    }
}

impl Into<VNode> for H2 {
    fn into(self) -> VNode {
        let vtag: VTag = self.into();
        VNode::from(vtag)
    }
}

pub struct H3 {
    attributes: Vec<PositionalAttr>,
    children: Vec<VNode>,
}

impl H3 {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("accesskey", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("autocapitalize", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("class", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contextmenu", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("dir", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("draggable", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("hidden", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("id", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("itemprop", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("lang", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("slot", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("spellcheck", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("style", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("tabindex", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("translate", value));
        self
    }

    pub fn child(mut self, element: impl Into<VTag>) -> Self {
        self.children.push(VNode::from(element.into()));
        self
    }

    pub fn component<C: Component>(mut self, props: C::Properties) -> Self {
        self.children.push(VNode::from(VComp::new::<C>(
            props,
            NodeRef::default(),
            None,
        )));
        self
    }

    pub fn text(mut self, text: impl Into<AttrValue>) -> Self {
        self.children.push(VNode::from(VText::new(text)));
        self
    }
}

impl Into<VTag> for H3 {
    fn into(self) -> VTag {
        let mut vtag = VTag::new("h3");
        vtag.attributes = Attributes::Vec(self.attributes);
        vtag.add_children(self.children.into_iter());
        vtag
    }
}

impl Into<VNode> for H3 {
    fn into(self) -> VNode {
        let vtag: VTag = self.into();
        VNode::from(vtag)
    }
}

pub struct H4 {
    attributes: Vec<PositionalAttr>,
    children: Vec<VNode>,
}

impl H4 {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("accesskey", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("autocapitalize", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("class", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contextmenu", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("dir", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("draggable", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("hidden", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("id", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("itemprop", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("lang", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("slot", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("spellcheck", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("style", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("tabindex", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("translate", value));
        self
    }

    pub fn child(mut self, element: impl Into<VTag>) -> Self {
        self.children.push(VNode::from(element.into()));
        self
    }

    pub fn component<C: Component>(mut self, props: C::Properties) -> Self {
        self.children.push(VNode::from(VComp::new::<C>(
            props,
            NodeRef::default(),
            None,
        )));
        self
    }

    pub fn text(mut self, text: impl Into<AttrValue>) -> Self {
        self.children.push(VNode::from(VText::new(text)));
        self
    }
}

impl Into<VTag> for H4 {
    fn into(self) -> VTag {
        let mut vtag = VTag::new("h4");
        vtag.attributes = Attributes::Vec(self.attributes);
        vtag.add_children(self.children.into_iter());
        vtag
    }
}

impl Into<VNode> for H4 {
    fn into(self) -> VNode {
        let vtag: VTag = self.into();
        VNode::from(vtag)
    }
}

pub struct H5 {
    attributes: Vec<PositionalAttr>,
    children: Vec<VNode>,
}

impl H5 {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("accesskey", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("autocapitalize", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("class", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contextmenu", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("dir", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("draggable", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("hidden", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("id", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("itemprop", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("lang", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("slot", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("spellcheck", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("style", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("tabindex", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("translate", value));
        self
    }

    pub fn child(mut self, element: impl Into<VTag>) -> Self {
        self.children.push(VNode::from(element.into()));
        self
    }

    pub fn component<C: Component>(mut self, props: C::Properties) -> Self {
        self.children.push(VNode::from(VComp::new::<C>(
            props,
            NodeRef::default(),
            None,
        )));
        self
    }

    pub fn text(mut self, text: impl Into<AttrValue>) -> Self {
        self.children.push(VNode::from(VText::new(text)));
        self
    }
}

impl Into<VTag> for H5 {
    fn into(self) -> VTag {
        let mut vtag = VTag::new("h5");
        vtag.attributes = Attributes::Vec(self.attributes);
        vtag.add_children(self.children.into_iter());
        vtag
    }
}

impl Into<VNode> for H5 {
    fn into(self) -> VNode {
        let vtag: VTag = self.into();
        VNode::from(vtag)
    }
}

pub struct H6 {
    attributes: Vec<PositionalAttr>,
    children: Vec<VNode>,
}

impl H6 {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
        }
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("accesskey", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("autocapitalize", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("class", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("contextmenu", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("dir", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("draggable", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("hidden", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("id", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("itemprop", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("lang", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("slot", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("spellcheck", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("style", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("tabindex", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(PositionalAttr::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(PositionalAttr::new("translate", value));
        self
    }

    pub fn child(mut self, element: impl Into<VTag>) -> Self {
        self.children.push(VNode::from(element.into()));
        self
    }

    pub fn component<C: Component>(mut self, props: C::Properties) -> Self {
        self.children.push(VNode::from(VComp::new::<C>(
            props,
            NodeRef::default(),
            None,
        )));
        self
    }

    pub fn text(mut self, text: impl Into<AttrValue>) -> Self {
        self.children.push(VNode::from(VText::new(text)));
        self
    }
}

impl Into<VTag> for H6 {
    fn into(self) -> VTag {
        let mut vtag = VTag::new("h6");
        vtag.attributes = Attributes::Vec(self.attributes);
        vtag.add_children(self.children.into_iter());
        vtag
    }
}

impl Into<VNode> for H6 {
    fn into(self) -> VNode {
        let vtag: VTag = self.into();
        VNode::from(vtag)
    }
}
