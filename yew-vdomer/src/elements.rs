use crate::{impl_velement, Attribute, Listener, VElement};
use yew::virtual_dom::{AttrValue, VNode, VTag};
pub struct Html {
    attributes: Vec<Attribute>,
    children: Vec<VNode>,
    listeners: Vec<Listener>,
}

impl Html {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
            listeners: vec![],
        }
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("accesskey", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("autocapitalize", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("class", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("contextmenu", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("dir", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("draggable", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("hidden", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("id", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("itemprop", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("lang", value));
        self
    }

    pub fn manifest(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("manifest", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("slot", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("spellcheck", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("style", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("tabindex", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("translate", value));
        self
    }
}

impl_velement!(Html, "html");
pub struct Base {
    attributes: Vec<Attribute>,
    children: Vec<VNode>,
    listeners: Vec<Listener>,
}

impl Base {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
            listeners: vec![],
        }
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("accesskey", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("autocapitalize", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("class", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("contextmenu", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("dir", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("draggable", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("hidden", value));
        self
    }

    pub fn href(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("href", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("id", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("itemprop", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("lang", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("slot", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("spellcheck", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("style", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("tabindex", value));
        self
    }

    pub fn target(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("target", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("translate", value));
        self
    }
}

impl_velement!(Base, "base");
pub struct Head {
    attributes: Vec<Attribute>,
    children: Vec<VNode>,
    listeners: Vec<Listener>,
}

impl Head {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
            listeners: vec![],
        }
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("accesskey", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("autocapitalize", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("class", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("contextmenu", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("dir", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("draggable", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("hidden", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("id", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("itemprop", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("lang", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("slot", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("spellcheck", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("style", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("tabindex", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("translate", value));
        self
    }
}

impl_velement!(Head, "head");
pub struct Link {
    attributes: Vec<Attribute>,
    children: Vec<VNode>,
    listeners: Vec<Listener>,
}

impl Link {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
            listeners: vec![],
        }
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("accesskey", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("autocapitalize", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("class", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("contextmenu", value));
        self
    }

    pub fn crossorigin(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("crossorigin", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("dir", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("draggable", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("hidden", value));
        self
    }

    pub fn href(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("href", value));
        self
    }

    pub fn hreflang(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("hreflang", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("id", value));
        self
    }

    pub fn importance(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("importance", value));
        self
    }

    pub fn integrity(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("integrity", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("itemprop", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("lang", value));
        self
    }

    pub fn media(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("media", value));
        self
    }

    pub fn referrerpolicy(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("referrerpolicy", value));
        self
    }

    pub fn rel(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("rel", value));
        self
    }

    pub fn sizes(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("sizes", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("slot", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("spellcheck", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("style", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("tabindex", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("translate", value));
        self
    }
}

impl_velement!(Link, "link");
pub struct Meta {
    attributes: Vec<Attribute>,
    children: Vec<VNode>,
    listeners: Vec<Listener>,
}

impl Meta {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
            listeners: vec![],
        }
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("accesskey", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("autocapitalize", value));
        self
    }

    pub fn charset(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("charset", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("class", value));
        self
    }

    pub fn content(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("content", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("contextmenu", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("dir", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("draggable", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("hidden", value));
        self
    }

    pub fn http_equiv(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("http_equiv", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("id", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("itemprop", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("lang", value));
        self
    }

    pub fn name(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("name", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("slot", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("spellcheck", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("style", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("tabindex", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("translate", value));
        self
    }
}

impl_velement!(Meta, "meta");
pub struct Style {
    attributes: Vec<Attribute>,
    children: Vec<VNode>,
    listeners: Vec<Listener>,
}

impl Style {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
            listeners: vec![],
        }
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("accesskey", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("autocapitalize", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("class", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("contextmenu", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("dir", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("draggable", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("hidden", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("id", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("itemprop", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("lang", value));
        self
    }

    pub fn media(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("media", value));
        self
    }

    pub fn scoped(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("scoped", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("slot", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("spellcheck", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("style", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("tabindex", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("translate", value));
        self
    }

    pub fn r#type(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("type", value));
        self
    }
}

impl_velement!(Style, "style");
pub struct Title {
    attributes: Vec<Attribute>,
    children: Vec<VNode>,
    listeners: Vec<Listener>,
}

impl Title {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
            listeners: vec![],
        }
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("accesskey", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("autocapitalize", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("class", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("contextmenu", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("dir", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("draggable", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("hidden", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("id", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("itemprop", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("lang", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("slot", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("spellcheck", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("style", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("tabindex", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("translate", value));
        self
    }
}

impl_velement!(Title, "title");
pub struct Body {
    attributes: Vec<Attribute>,
    children: Vec<VNode>,
    listeners: Vec<Listener>,
}

impl Body {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
            listeners: vec![],
        }
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("accesskey", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("autocapitalize", value));
        self
    }

    pub fn background(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("background", value));
        self
    }

    pub fn bgcolor(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("bgcolor", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("class", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("contextmenu", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("dir", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("draggable", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("hidden", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("id", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("itemprop", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("lang", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("slot", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("spellcheck", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("style", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("tabindex", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("translate", value));
        self
    }
}

impl_velement!(Body, "body");
pub struct Address {
    attributes: Vec<Attribute>,
    children: Vec<VNode>,
    listeners: Vec<Listener>,
}

impl Address {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
            listeners: vec![],
        }
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("accesskey", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("autocapitalize", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("class", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("contextmenu", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("dir", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("draggable", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("hidden", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("id", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("itemprop", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("lang", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("slot", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("spellcheck", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("style", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("tabindex", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("translate", value));
        self
    }
}

impl_velement!(Address, "address");
pub struct Article {
    attributes: Vec<Attribute>,
    children: Vec<VNode>,
    listeners: Vec<Listener>,
}

impl Article {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
            listeners: vec![],
        }
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("accesskey", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("autocapitalize", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("class", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("contextmenu", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("dir", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("draggable", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("hidden", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("id", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("itemprop", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("lang", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("slot", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("spellcheck", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("style", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("tabindex", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("translate", value));
        self
    }
}

impl_velement!(Article, "article");
pub struct Aside {
    attributes: Vec<Attribute>,
    children: Vec<VNode>,
    listeners: Vec<Listener>,
}

impl Aside {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
            listeners: vec![],
        }
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("accesskey", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("autocapitalize", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("class", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("contextmenu", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("dir", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("draggable", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("hidden", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("id", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("itemprop", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("lang", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("slot", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("spellcheck", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("style", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("tabindex", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("translate", value));
        self
    }
}

impl_velement!(Aside, "aside");
pub struct Footer {
    attributes: Vec<Attribute>,
    children: Vec<VNode>,
    listeners: Vec<Listener>,
}

impl Footer {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
            listeners: vec![],
        }
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("accesskey", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("autocapitalize", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("class", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("contextmenu", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("dir", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("draggable", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("hidden", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("id", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("itemprop", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("lang", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("slot", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("spellcheck", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("style", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("tabindex", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("translate", value));
        self
    }
}

impl_velement!(Footer, "footer");
pub struct Header {
    attributes: Vec<Attribute>,
    children: Vec<VNode>,
    listeners: Vec<Listener>,
}

impl Header {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
            listeners: vec![],
        }
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("accesskey", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("autocapitalize", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("class", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("contextmenu", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("dir", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("draggable", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("hidden", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("id", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("itemprop", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("lang", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("slot", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("spellcheck", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("style", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("tabindex", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("translate", value));
        self
    }
}

impl_velement!(Header, "header");
pub struct Main {
    attributes: Vec<Attribute>,
    children: Vec<VNode>,
    listeners: Vec<Listener>,
}

impl Main {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
            listeners: vec![],
        }
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("accesskey", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("autocapitalize", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("class", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("contextmenu", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("dir", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("draggable", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("hidden", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("id", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("itemprop", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("lang", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("slot", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("spellcheck", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("style", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("tabindex", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("translate", value));
        self
    }
}

impl_velement!(Main, "main");
pub struct Nav {
    attributes: Vec<Attribute>,
    children: Vec<VNode>,
    listeners: Vec<Listener>,
}

impl Nav {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
            listeners: vec![],
        }
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("accesskey", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("autocapitalize", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("class", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("contextmenu", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("dir", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("draggable", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("hidden", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("id", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("itemprop", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("lang", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("slot", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("spellcheck", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("style", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("tabindex", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("translate", value));
        self
    }
}

impl_velement!(Nav, "nav");
pub struct Section {
    attributes: Vec<Attribute>,
    children: Vec<VNode>,
    listeners: Vec<Listener>,
}

impl Section {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
            listeners: vec![],
        }
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("accesskey", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("autocapitalize", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("class", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("contextmenu", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("dir", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("draggable", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("hidden", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("id", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("itemprop", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("lang", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("slot", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("spellcheck", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("style", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("tabindex", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("translate", value));
        self
    }
}

impl_velement!(Section, "section");
pub struct Blockquote {
    attributes: Vec<Attribute>,
    children: Vec<VNode>,
    listeners: Vec<Listener>,
}

impl Blockquote {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
            listeners: vec![],
        }
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("accesskey", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("autocapitalize", value));
        self
    }

    pub fn cite(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("cite", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("class", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("contextmenu", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("dir", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("draggable", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("hidden", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("id", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("itemprop", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("lang", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("slot", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("spellcheck", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("style", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("tabindex", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("translate", value));
        self
    }
}

impl_velement!(Blockquote, "blockquote");
pub struct Dd {
    attributes: Vec<Attribute>,
    children: Vec<VNode>,
    listeners: Vec<Listener>,
}

impl Dd {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
            listeners: vec![],
        }
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("accesskey", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("autocapitalize", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("class", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("contextmenu", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("dir", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("draggable", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("hidden", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("id", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("itemprop", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("lang", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("slot", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("spellcheck", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("style", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("tabindex", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("translate", value));
        self
    }
}

impl_velement!(Dd, "dd");
pub struct Div {
    attributes: Vec<Attribute>,
    children: Vec<VNode>,
    listeners: Vec<Listener>,
}

impl Div {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
            listeners: vec![],
        }
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("accesskey", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("autocapitalize", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("class", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("contextmenu", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("dir", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("draggable", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("hidden", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("id", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("itemprop", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("lang", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("slot", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("spellcheck", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("style", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("tabindex", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("translate", value));
        self
    }
}

impl_velement!(Div, "div");
pub struct Dl {
    attributes: Vec<Attribute>,
    children: Vec<VNode>,
    listeners: Vec<Listener>,
}

impl Dl {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
            listeners: vec![],
        }
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("accesskey", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("autocapitalize", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("class", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("contextmenu", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("dir", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("draggable", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("hidden", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("id", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("itemprop", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("lang", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("slot", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("spellcheck", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("style", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("tabindex", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("translate", value));
        self
    }
}

impl_velement!(Dl, "dl");
pub struct Dt {
    attributes: Vec<Attribute>,
    children: Vec<VNode>,
    listeners: Vec<Listener>,
}

impl Dt {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
            listeners: vec![],
        }
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("accesskey", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("autocapitalize", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("class", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("contextmenu", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("dir", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("draggable", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("hidden", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("id", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("itemprop", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("lang", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("slot", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("spellcheck", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("style", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("tabindex", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("translate", value));
        self
    }
}

impl_velement!(Dt, "dt");
pub struct Figcaption {
    attributes: Vec<Attribute>,
    children: Vec<VNode>,
    listeners: Vec<Listener>,
}

impl Figcaption {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
            listeners: vec![],
        }
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("accesskey", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("autocapitalize", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("class", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("contextmenu", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("dir", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("draggable", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("hidden", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("id", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("itemprop", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("lang", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("slot", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("spellcheck", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("style", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("tabindex", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("translate", value));
        self
    }
}

impl_velement!(Figcaption, "figcaption");
pub struct Figure {
    attributes: Vec<Attribute>,
    children: Vec<VNode>,
    listeners: Vec<Listener>,
}

impl Figure {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
            listeners: vec![],
        }
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("accesskey", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("autocapitalize", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("class", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("contextmenu", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("dir", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("draggable", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("hidden", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("id", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("itemprop", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("lang", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("slot", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("spellcheck", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("style", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("tabindex", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("translate", value));
        self
    }
}

impl_velement!(Figure, "figure");
pub struct Hr {
    attributes: Vec<Attribute>,
    children: Vec<VNode>,
    listeners: Vec<Listener>,
}

impl Hr {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
            listeners: vec![],
        }
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("accesskey", value));
        self
    }

    pub fn align(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("align", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("autocapitalize", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("class", value));
        self
    }

    pub fn color(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("color", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("contextmenu", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("dir", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("draggable", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("hidden", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("id", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("itemprop", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("lang", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("slot", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("spellcheck", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("style", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("tabindex", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("translate", value));
        self
    }
}

impl_velement!(Hr, "hr");
pub struct Li {
    attributes: Vec<Attribute>,
    children: Vec<VNode>,
    listeners: Vec<Listener>,
}

impl Li {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
            listeners: vec![],
        }
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("accesskey", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("autocapitalize", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("class", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("contextmenu", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("dir", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("draggable", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("hidden", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("id", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("itemprop", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("lang", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("slot", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("spellcheck", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("style", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("tabindex", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("translate", value));
        self
    }

    pub fn value(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("value", value));
        self
    }
}

impl_velement!(Li, "li");
pub struct Ol {
    attributes: Vec<Attribute>,
    children: Vec<VNode>,
    listeners: Vec<Listener>,
}

impl Ol {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
            listeners: vec![],
        }
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("accesskey", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("autocapitalize", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("class", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("contextmenu", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("dir", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("draggable", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("hidden", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("id", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("itemprop", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("lang", value));
        self
    }

    pub fn reversed(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("reversed", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("slot", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("spellcheck", value));
        self
    }

    pub fn start(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("start", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("style", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("tabindex", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("translate", value));
        self
    }
}

impl_velement!(Ol, "ol");
pub struct P {
    attributes: Vec<Attribute>,
    children: Vec<VNode>,
    listeners: Vec<Listener>,
}

impl P {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
            listeners: vec![],
        }
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("accesskey", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("autocapitalize", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("class", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("contextmenu", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("dir", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("draggable", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("hidden", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("id", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("itemprop", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("lang", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("slot", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("spellcheck", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("style", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("tabindex", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("translate", value));
        self
    }
}

impl_velement!(P, "p");
pub struct Pre {
    attributes: Vec<Attribute>,
    children: Vec<VNode>,
    listeners: Vec<Listener>,
}

impl Pre {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
            listeners: vec![],
        }
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("accesskey", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("autocapitalize", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("class", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("contextmenu", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("dir", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("draggable", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("hidden", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("id", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("itemprop", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("lang", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("slot", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("spellcheck", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("style", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("tabindex", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("translate", value));
        self
    }
}

impl_velement!(Pre, "pre");
pub struct Ul {
    attributes: Vec<Attribute>,
    children: Vec<VNode>,
    listeners: Vec<Listener>,
}

impl Ul {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
            listeners: vec![],
        }
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("accesskey", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("autocapitalize", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("class", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("contextmenu", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("dir", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("draggable", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("hidden", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("id", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("itemprop", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("lang", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("slot", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("spellcheck", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("style", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("tabindex", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("translate", value));
        self
    }
}

impl_velement!(Ul, "ul");
pub struct A {
    attributes: Vec<Attribute>,
    children: Vec<VNode>,
    listeners: Vec<Listener>,
}

impl A {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
            listeners: vec![],
        }
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("accesskey", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("autocapitalize", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("class", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("contextmenu", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("dir", value));
        self
    }

    pub fn download(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("download", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("draggable", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("hidden", value));
        self
    }

    pub fn href(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("href", value));
        self
    }

    pub fn hreflang(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("hreflang", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("id", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("itemprop", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("lang", value));
        self
    }

    pub fn media(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("media", value));
        self
    }

    pub fn ping(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("ping", value));
        self
    }

    pub fn referrerpolicy(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("referrerpolicy", value));
        self
    }

    pub fn rel(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("rel", value));
        self
    }

    pub fn shape(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("shape", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("slot", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("spellcheck", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("style", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("tabindex", value));
        self
    }

    pub fn target(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("target", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("translate", value));
        self
    }
}

impl_velement!(A, "a");
pub struct Abbr {
    attributes: Vec<Attribute>,
    children: Vec<VNode>,
    listeners: Vec<Listener>,
}

impl Abbr {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
            listeners: vec![],
        }
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("accesskey", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("autocapitalize", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("class", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("contextmenu", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("dir", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("draggable", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("hidden", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("id", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("itemprop", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("lang", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("slot", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("spellcheck", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("style", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("tabindex", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("translate", value));
        self
    }
}

impl_velement!(Abbr, "abbr");
pub struct B {
    attributes: Vec<Attribute>,
    children: Vec<VNode>,
    listeners: Vec<Listener>,
}

impl B {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
            listeners: vec![],
        }
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("accesskey", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("autocapitalize", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("class", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("contextmenu", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("dir", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("draggable", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("hidden", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("id", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("itemprop", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("lang", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("slot", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("spellcheck", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("style", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("tabindex", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("translate", value));
        self
    }
}

impl_velement!(B, "b");
pub struct Bdi {
    attributes: Vec<Attribute>,
    children: Vec<VNode>,
    listeners: Vec<Listener>,
}

impl Bdi {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
            listeners: vec![],
        }
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("accesskey", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("autocapitalize", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("class", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("contextmenu", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("dir", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("draggable", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("hidden", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("id", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("itemprop", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("lang", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("slot", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("spellcheck", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("style", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("tabindex", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("translate", value));
        self
    }
}

impl_velement!(Bdi, "bdi");
pub struct Bdo {
    attributes: Vec<Attribute>,
    children: Vec<VNode>,
    listeners: Vec<Listener>,
}

impl Bdo {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
            listeners: vec![],
        }
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("accesskey", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("autocapitalize", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("class", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("contextmenu", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("dir", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("draggable", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("hidden", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("id", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("itemprop", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("lang", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("slot", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("spellcheck", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("style", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("tabindex", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("translate", value));
        self
    }
}

impl_velement!(Bdo, "bdo");
pub struct Br {
    attributes: Vec<Attribute>,
    children: Vec<VNode>,
    listeners: Vec<Listener>,
}

impl Br {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
            listeners: vec![],
        }
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("accesskey", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("autocapitalize", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("class", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("contextmenu", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("dir", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("draggable", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("hidden", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("id", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("itemprop", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("lang", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("slot", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("spellcheck", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("style", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("tabindex", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("translate", value));
        self
    }
}

impl_velement!(Br, "br");
pub struct Cite {
    attributes: Vec<Attribute>,
    children: Vec<VNode>,
    listeners: Vec<Listener>,
}

impl Cite {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
            listeners: vec![],
        }
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("accesskey", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("autocapitalize", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("class", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("contextmenu", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("dir", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("draggable", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("hidden", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("id", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("itemprop", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("lang", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("slot", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("spellcheck", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("style", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("tabindex", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("translate", value));
        self
    }
}

impl_velement!(Cite, "cite");
pub struct Code {
    attributes: Vec<Attribute>,
    children: Vec<VNode>,
    listeners: Vec<Listener>,
}

impl Code {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
            listeners: vec![],
        }
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("accesskey", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("autocapitalize", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("class", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("contextmenu", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("dir", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("draggable", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("hidden", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("id", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("itemprop", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("lang", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("slot", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("spellcheck", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("style", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("tabindex", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("translate", value));
        self
    }
}

impl_velement!(Code, "code");
pub struct Data {
    attributes: Vec<Attribute>,
    children: Vec<VNode>,
    listeners: Vec<Listener>,
}

impl Data {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
            listeners: vec![],
        }
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("accesskey", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("autocapitalize", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("class", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("contextmenu", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("dir", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("draggable", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("hidden", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("id", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("itemprop", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("lang", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("slot", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("spellcheck", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("style", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("tabindex", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("translate", value));
        self
    }

    pub fn value(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("value", value));
        self
    }
}

impl_velement!(Data, "data");
pub struct Dfn {
    attributes: Vec<Attribute>,
    children: Vec<VNode>,
    listeners: Vec<Listener>,
}

impl Dfn {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
            listeners: vec![],
        }
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("accesskey", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("autocapitalize", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("class", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("contextmenu", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("dir", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("draggable", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("hidden", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("id", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("itemprop", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("lang", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("slot", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("spellcheck", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("style", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("tabindex", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("translate", value));
        self
    }
}

impl_velement!(Dfn, "dfn");
pub struct Em {
    attributes: Vec<Attribute>,
    children: Vec<VNode>,
    listeners: Vec<Listener>,
}

impl Em {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
            listeners: vec![],
        }
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("accesskey", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("autocapitalize", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("class", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("contextmenu", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("dir", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("draggable", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("hidden", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("id", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("itemprop", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("lang", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("slot", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("spellcheck", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("style", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("tabindex", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("translate", value));
        self
    }
}

impl_velement!(Em, "em");
pub struct I {
    attributes: Vec<Attribute>,
    children: Vec<VNode>,
    listeners: Vec<Listener>,
}

impl I {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
            listeners: vec![],
        }
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("accesskey", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("autocapitalize", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("class", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("contextmenu", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("dir", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("draggable", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("hidden", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("id", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("itemprop", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("lang", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("slot", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("spellcheck", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("style", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("tabindex", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("translate", value));
        self
    }
}

impl_velement!(I, "i");
pub struct Kbd {
    attributes: Vec<Attribute>,
    children: Vec<VNode>,
    listeners: Vec<Listener>,
}

impl Kbd {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
            listeners: vec![],
        }
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("accesskey", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("autocapitalize", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("class", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("contextmenu", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("dir", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("draggable", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("hidden", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("id", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("itemprop", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("lang", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("slot", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("spellcheck", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("style", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("tabindex", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("translate", value));
        self
    }
}

impl_velement!(Kbd, "kbd");
pub struct Mark {
    attributes: Vec<Attribute>,
    children: Vec<VNode>,
    listeners: Vec<Listener>,
}

impl Mark {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
            listeners: vec![],
        }
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("accesskey", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("autocapitalize", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("class", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("contextmenu", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("dir", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("draggable", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("hidden", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("id", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("itemprop", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("lang", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("slot", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("spellcheck", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("style", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("tabindex", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("translate", value));
        self
    }
}

impl_velement!(Mark, "mark");
pub struct Q {
    attributes: Vec<Attribute>,
    children: Vec<VNode>,
    listeners: Vec<Listener>,
}

impl Q {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
            listeners: vec![],
        }
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("accesskey", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("autocapitalize", value));
        self
    }

    pub fn cite(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("cite", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("class", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("contextmenu", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("dir", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("draggable", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("hidden", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("id", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("itemprop", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("lang", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("slot", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("spellcheck", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("style", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("tabindex", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("translate", value));
        self
    }
}

impl_velement!(Q, "q");
pub struct Rp {
    attributes: Vec<Attribute>,
    children: Vec<VNode>,
    listeners: Vec<Listener>,
}

impl Rp {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
            listeners: vec![],
        }
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("accesskey", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("autocapitalize", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("class", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("contextmenu", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("dir", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("draggable", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("hidden", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("id", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("itemprop", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("lang", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("slot", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("spellcheck", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("style", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("tabindex", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("translate", value));
        self
    }
}

impl_velement!(Rp, "rp");
pub struct Rt {
    attributes: Vec<Attribute>,
    children: Vec<VNode>,
    listeners: Vec<Listener>,
}

impl Rt {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
            listeners: vec![],
        }
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("accesskey", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("autocapitalize", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("class", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("contextmenu", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("dir", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("draggable", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("hidden", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("id", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("itemprop", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("lang", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("slot", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("spellcheck", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("style", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("tabindex", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("translate", value));
        self
    }
}

impl_velement!(Rt, "rt");
pub struct Ruby {
    attributes: Vec<Attribute>,
    children: Vec<VNode>,
    listeners: Vec<Listener>,
}

impl Ruby {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
            listeners: vec![],
        }
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("accesskey", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("autocapitalize", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("class", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("contextmenu", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("dir", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("draggable", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("hidden", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("id", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("itemprop", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("lang", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("slot", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("spellcheck", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("style", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("tabindex", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("translate", value));
        self
    }
}

impl_velement!(Ruby, "ruby");
pub struct S {
    attributes: Vec<Attribute>,
    children: Vec<VNode>,
    listeners: Vec<Listener>,
}

impl S {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
            listeners: vec![],
        }
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("accesskey", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("autocapitalize", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("class", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("contextmenu", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("dir", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("draggable", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("hidden", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("id", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("itemprop", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("lang", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("slot", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("spellcheck", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("style", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("tabindex", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("translate", value));
        self
    }
}

impl_velement!(S, "s");
pub struct Samp {
    attributes: Vec<Attribute>,
    children: Vec<VNode>,
    listeners: Vec<Listener>,
}

impl Samp {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
            listeners: vec![],
        }
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("accesskey", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("autocapitalize", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("class", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("contextmenu", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("dir", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("draggable", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("hidden", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("id", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("itemprop", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("lang", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("slot", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("spellcheck", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("style", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("tabindex", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("translate", value));
        self
    }
}

impl_velement!(Samp, "samp");
pub struct Small {
    attributes: Vec<Attribute>,
    children: Vec<VNode>,
    listeners: Vec<Listener>,
}

impl Small {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
            listeners: vec![],
        }
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("accesskey", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("autocapitalize", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("class", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("contextmenu", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("dir", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("draggable", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("hidden", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("id", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("itemprop", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("lang", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("slot", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("spellcheck", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("style", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("tabindex", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("translate", value));
        self
    }
}

impl_velement!(Small, "small");
pub struct Span {
    attributes: Vec<Attribute>,
    children: Vec<VNode>,
    listeners: Vec<Listener>,
}

impl Span {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
            listeners: vec![],
        }
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("accesskey", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("autocapitalize", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("class", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("contextmenu", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("dir", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("draggable", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("hidden", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("id", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("itemprop", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("lang", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("slot", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("spellcheck", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("style", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("tabindex", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("translate", value));
        self
    }
}

impl_velement!(Span, "span");
pub struct Strong {
    attributes: Vec<Attribute>,
    children: Vec<VNode>,
    listeners: Vec<Listener>,
}

impl Strong {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
            listeners: vec![],
        }
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("accesskey", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("autocapitalize", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("class", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("contextmenu", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("dir", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("draggable", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("hidden", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("id", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("itemprop", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("lang", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("slot", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("spellcheck", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("style", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("tabindex", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("translate", value));
        self
    }
}

impl_velement!(Strong, "strong");
pub struct Sub {
    attributes: Vec<Attribute>,
    children: Vec<VNode>,
    listeners: Vec<Listener>,
}

impl Sub {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
            listeners: vec![],
        }
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("accesskey", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("autocapitalize", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("class", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("contextmenu", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("dir", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("draggable", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("hidden", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("id", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("itemprop", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("lang", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("slot", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("spellcheck", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("style", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("tabindex", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("translate", value));
        self
    }
}

impl_velement!(Sub, "sub");
pub struct Sup {
    attributes: Vec<Attribute>,
    children: Vec<VNode>,
    listeners: Vec<Listener>,
}

impl Sup {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
            listeners: vec![],
        }
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("accesskey", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("autocapitalize", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("class", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("contextmenu", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("dir", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("draggable", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("hidden", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("id", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("itemprop", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("lang", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("slot", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("spellcheck", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("style", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("tabindex", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("translate", value));
        self
    }
}

impl_velement!(Sup, "sup");
pub struct Time {
    attributes: Vec<Attribute>,
    children: Vec<VNode>,
    listeners: Vec<Listener>,
}

impl Time {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
            listeners: vec![],
        }
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("accesskey", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("autocapitalize", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("class", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("contextmenu", value));
        self
    }

    pub fn datetime(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("datetime", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("dir", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("draggable", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("hidden", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("id", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("itemprop", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("lang", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("slot", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("spellcheck", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("style", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("tabindex", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("translate", value));
        self
    }
}

impl_velement!(Time, "time");
pub struct U {
    attributes: Vec<Attribute>,
    children: Vec<VNode>,
    listeners: Vec<Listener>,
}

impl U {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
            listeners: vec![],
        }
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("accesskey", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("autocapitalize", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("class", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("contextmenu", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("dir", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("draggable", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("hidden", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("id", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("itemprop", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("lang", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("slot", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("spellcheck", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("style", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("tabindex", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("translate", value));
        self
    }
}

impl_velement!(U, "u");
pub struct Var {
    attributes: Vec<Attribute>,
    children: Vec<VNode>,
    listeners: Vec<Listener>,
}

impl Var {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
            listeners: vec![],
        }
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("accesskey", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("autocapitalize", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("class", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("contextmenu", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("dir", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("draggable", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("hidden", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("id", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("itemprop", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("lang", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("slot", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("spellcheck", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("style", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("tabindex", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("translate", value));
        self
    }
}

impl_velement!(Var, "var");
pub struct Wbr {
    attributes: Vec<Attribute>,
    children: Vec<VNode>,
    listeners: Vec<Listener>,
}

impl Wbr {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
            listeners: vec![],
        }
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("accesskey", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("autocapitalize", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("class", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("contextmenu", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("dir", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("draggable", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("hidden", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("id", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("itemprop", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("lang", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("slot", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("spellcheck", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("style", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("tabindex", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("translate", value));
        self
    }
}

impl_velement!(Wbr, "wbr");
pub struct Area {
    attributes: Vec<Attribute>,
    children: Vec<VNode>,
    listeners: Vec<Listener>,
}

impl Area {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
            listeners: vec![],
        }
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("accesskey", value));
        self
    }

    pub fn alt(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("alt", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("autocapitalize", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("class", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("contextmenu", value));
        self
    }

    pub fn coords(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("coords", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("dir", value));
        self
    }

    pub fn download(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("download", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("draggable", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("hidden", value));
        self
    }

    pub fn href(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("href", value));
        self
    }

    pub fn hreflang(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("hreflang", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("id", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("itemprop", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("lang", value));
        self
    }

    pub fn media(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("media", value));
        self
    }

    pub fn ping(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("ping", value));
        self
    }

    pub fn referrerpolicy(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("referrerpolicy", value));
        self
    }

    pub fn rel(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("rel", value));
        self
    }

    pub fn shape(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("shape", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("slot", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("spellcheck", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("style", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("tabindex", value));
        self
    }

    pub fn target(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("target", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("translate", value));
        self
    }
}

impl_velement!(Area, "area");
pub struct Audio {
    attributes: Vec<Attribute>,
    children: Vec<VNode>,
    listeners: Vec<Listener>,
}

impl Audio {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
            listeners: vec![],
        }
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("accesskey", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("autocapitalize", value));
        self
    }

    pub fn autoplay(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("autoplay", value));
        self
    }

    pub fn buffered(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("buffered", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("class", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("contextmenu", value));
        self
    }

    pub fn controls(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("controls", value));
        self
    }

    pub fn crossorigin(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("crossorigin", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("dir", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("draggable", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("hidden", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("id", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("itemprop", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("lang", value));
        self
    }

    pub fn r#loop(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("loop", value));
        self
    }

    pub fn muted(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("muted", value));
        self
    }

    pub fn preload(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("preload", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("slot", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("spellcheck", value));
        self
    }

    pub fn src(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("src", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("style", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("tabindex", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("translate", value));
        self
    }
}

impl_velement!(Audio, "audio");
pub struct Img {
    attributes: Vec<Attribute>,
    children: Vec<VNode>,
    listeners: Vec<Listener>,
}

impl Img {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
            listeners: vec![],
        }
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("accesskey", value));
        self
    }

    pub fn align(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("align", value));
        self
    }

    pub fn alt(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("alt", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("autocapitalize", value));
        self
    }

    pub fn border(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("border", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("class", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("contextmenu", value));
        self
    }

    pub fn crossorigin(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("crossorigin", value));
        self
    }

    pub fn decoding(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("decoding", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("dir", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("draggable", value));
        self
    }

    pub fn height(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("height", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("hidden", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("id", value));
        self
    }

    pub fn importance(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("importance", value));
        self
    }

    pub fn intrinsicsize(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("intrinsicsize", value));
        self
    }

    pub fn ismap(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("ismap", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("itemprop", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("lang", value));
        self
    }

    pub fn loading(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("loading", value));
        self
    }

    pub fn referrerpolicy(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("referrerpolicy", value));
        self
    }

    pub fn sizes(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("sizes", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("slot", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("spellcheck", value));
        self
    }

    pub fn src(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("src", value));
        self
    }

    pub fn srcset(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("srcset", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("style", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("tabindex", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("translate", value));
        self
    }

    pub fn usemap(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("usemap", value));
        self
    }

    pub fn width(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("width", value));
        self
    }
}

impl_velement!(Img, "img");
pub struct Map {
    attributes: Vec<Attribute>,
    children: Vec<VNode>,
    listeners: Vec<Listener>,
}

impl Map {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
            listeners: vec![],
        }
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("accesskey", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("autocapitalize", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("class", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("contextmenu", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("dir", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("draggable", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("hidden", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("id", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("itemprop", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("lang", value));
        self
    }

    pub fn name(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("name", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("slot", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("spellcheck", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("style", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("tabindex", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("translate", value));
        self
    }
}

impl_velement!(Map, "map");
pub struct Track {
    attributes: Vec<Attribute>,
    children: Vec<VNode>,
    listeners: Vec<Listener>,
}

impl Track {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
            listeners: vec![],
        }
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("accesskey", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("autocapitalize", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("class", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("contextmenu", value));
        self
    }

    pub fn default(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("default", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("dir", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("draggable", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("hidden", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("id", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("itemprop", value));
        self
    }

    pub fn kind(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("kind", value));
        self
    }

    pub fn label(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("label", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("lang", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("slot", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("spellcheck", value));
        self
    }

    pub fn src(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("src", value));
        self
    }

    pub fn srclang(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("srclang", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("style", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("tabindex", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("translate", value));
        self
    }
}

impl_velement!(Track, "track");
pub struct Video {
    attributes: Vec<Attribute>,
    children: Vec<VNode>,
    listeners: Vec<Listener>,
}

impl Video {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
            listeners: vec![],
        }
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("accesskey", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("autocapitalize", value));
        self
    }

    pub fn autoplay(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("autoplay", value));
        self
    }

    pub fn buffered(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("buffered", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("class", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("contextmenu", value));
        self
    }

    pub fn controls(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("controls", value));
        self
    }

    pub fn crossorigin(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("crossorigin", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("dir", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("draggable", value));
        self
    }

    pub fn height(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("height", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("hidden", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("id", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("itemprop", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("lang", value));
        self
    }

    pub fn r#loop(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("loop", value));
        self
    }

    pub fn muted(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("muted", value));
        self
    }

    pub fn poster(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("poster", value));
        self
    }

    pub fn preload(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("preload", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("slot", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("spellcheck", value));
        self
    }

    pub fn src(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("src", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("style", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("tabindex", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("translate", value));
        self
    }

    pub fn width(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("width", value));
        self
    }
}

impl_velement!(Video, "video");
pub struct Embed {
    attributes: Vec<Attribute>,
    children: Vec<VNode>,
    listeners: Vec<Listener>,
}

impl Embed {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
            listeners: vec![],
        }
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("accesskey", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("autocapitalize", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("class", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("contextmenu", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("dir", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("draggable", value));
        self
    }

    pub fn height(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("height", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("hidden", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("id", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("itemprop", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("lang", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("slot", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("spellcheck", value));
        self
    }

    pub fn src(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("src", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("style", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("tabindex", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("translate", value));
        self
    }

    pub fn r#type(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("type", value));
        self
    }

    pub fn width(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("width", value));
        self
    }
}

impl_velement!(Embed, "embed");
pub struct Iframe {
    attributes: Vec<Attribute>,
    children: Vec<VNode>,
    listeners: Vec<Listener>,
}

impl Iframe {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
            listeners: vec![],
        }
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("accesskey", value));
        self
    }

    pub fn align(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("align", value));
        self
    }

    pub fn allow(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("allow", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("autocapitalize", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("class", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("contextmenu", value));
        self
    }

    pub fn csp(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("csp", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("dir", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("draggable", value));
        self
    }

    pub fn height(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("height", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("hidden", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("id", value));
        self
    }

    pub fn importance(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("importance", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("itemprop", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("lang", value));
        self
    }

    pub fn loading(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("loading", value));
        self
    }

    pub fn name(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("name", value));
        self
    }

    pub fn referrerpolicy(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("referrerpolicy", value));
        self
    }

    pub fn sandbox(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("sandbox", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("slot", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("spellcheck", value));
        self
    }

    pub fn src(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("src", value));
        self
    }

    pub fn srcdoc(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("srcdoc", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("style", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("tabindex", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("translate", value));
        self
    }

    pub fn width(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("width", value));
        self
    }
}

impl_velement!(Iframe, "iframe");
pub struct Object {
    attributes: Vec<Attribute>,
    children: Vec<VNode>,
    listeners: Vec<Listener>,
}

impl Object {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
            listeners: vec![],
        }
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("accesskey", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("autocapitalize", value));
        self
    }

    pub fn border(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("border", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("class", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("contextmenu", value));
        self
    }

    pub fn data(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("data", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("dir", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("draggable", value));
        self
    }

    pub fn form(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("form", value));
        self
    }

    pub fn height(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("height", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("hidden", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("id", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("itemprop", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("lang", value));
        self
    }

    pub fn name(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("name", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("slot", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("spellcheck", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("style", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("tabindex", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("translate", value));
        self
    }

    pub fn r#type(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("type", value));
        self
    }

    pub fn usemap(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("usemap", value));
        self
    }

    pub fn width(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("width", value));
        self
    }
}

impl_velement!(Object, "object");
pub struct Param {
    attributes: Vec<Attribute>,
    children: Vec<VNode>,
    listeners: Vec<Listener>,
}

impl Param {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
            listeners: vec![],
        }
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("accesskey", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("autocapitalize", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("class", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("contextmenu", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("dir", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("draggable", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("hidden", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("id", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("itemprop", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("lang", value));
        self
    }

    pub fn name(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("name", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("slot", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("spellcheck", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("style", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("tabindex", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("translate", value));
        self
    }

    pub fn value(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("value", value));
        self
    }
}

impl_velement!(Param, "param");
pub struct Picture {
    attributes: Vec<Attribute>,
    children: Vec<VNode>,
    listeners: Vec<Listener>,
}

impl Picture {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
            listeners: vec![],
        }
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("accesskey", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("autocapitalize", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("class", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("contextmenu", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("dir", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("draggable", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("hidden", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("id", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("itemprop", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("lang", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("slot", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("spellcheck", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("style", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("tabindex", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("translate", value));
        self
    }
}

impl_velement!(Picture, "picture");
pub struct Portal {
    attributes: Vec<Attribute>,
    children: Vec<VNode>,
    listeners: Vec<Listener>,
}

impl Portal {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
            listeners: vec![],
        }
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("accesskey", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("autocapitalize", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("class", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("contextmenu", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("dir", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("draggable", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("hidden", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("id", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("itemprop", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("lang", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("slot", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("spellcheck", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("style", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("tabindex", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("translate", value));
        self
    }
}

impl_velement!(Portal, "portal");
pub struct Source {
    attributes: Vec<Attribute>,
    children: Vec<VNode>,
    listeners: Vec<Listener>,
}

impl Source {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
            listeners: vec![],
        }
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("accesskey", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("autocapitalize", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("class", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("contextmenu", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("dir", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("draggable", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("hidden", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("id", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("itemprop", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("lang", value));
        self
    }

    pub fn media(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("media", value));
        self
    }

    pub fn sizes(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("sizes", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("slot", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("spellcheck", value));
        self
    }

    pub fn src(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("src", value));
        self
    }

    pub fn srcset(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("srcset", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("style", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("tabindex", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("translate", value));
        self
    }

    pub fn r#type(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("type", value));
        self
    }
}

impl_velement!(Source, "source");
pub struct Svg {
    attributes: Vec<Attribute>,
    children: Vec<VNode>,
    listeners: Vec<Listener>,
}

impl Svg {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
            listeners: vec![],
        }
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("accesskey", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("autocapitalize", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("class", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("contextmenu", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("dir", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("draggable", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("hidden", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("id", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("itemprop", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("lang", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("slot", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("spellcheck", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("style", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("tabindex", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("translate", value));
        self
    }
}

impl_velement!(Svg, "svg");
pub struct Math {
    attributes: Vec<Attribute>,
    children: Vec<VNode>,
    listeners: Vec<Listener>,
}

impl Math {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
            listeners: vec![],
        }
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("accesskey", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("autocapitalize", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("class", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("contextmenu", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("dir", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("draggable", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("hidden", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("id", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("itemprop", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("lang", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("slot", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("spellcheck", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("style", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("tabindex", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("translate", value));
        self
    }
}

impl_velement!(Math, "math");
pub struct Canvas {
    attributes: Vec<Attribute>,
    children: Vec<VNode>,
    listeners: Vec<Listener>,
}

impl Canvas {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
            listeners: vec![],
        }
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("accesskey", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("autocapitalize", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("class", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("contextmenu", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("dir", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("draggable", value));
        self
    }

    pub fn height(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("height", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("hidden", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("id", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("itemprop", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("lang", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("slot", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("spellcheck", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("style", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("tabindex", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("translate", value));
        self
    }

    pub fn width(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("width", value));
        self
    }
}

impl_velement!(Canvas, "canvas");
pub struct Noscript {
    attributes: Vec<Attribute>,
    children: Vec<VNode>,
    listeners: Vec<Listener>,
}

impl Noscript {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
            listeners: vec![],
        }
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("accesskey", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("autocapitalize", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("class", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("contextmenu", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("dir", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("draggable", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("hidden", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("id", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("itemprop", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("lang", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("slot", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("spellcheck", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("style", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("tabindex", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("translate", value));
        self
    }
}

impl_velement!(Noscript, "noscript");
pub struct Script {
    attributes: Vec<Attribute>,
    children: Vec<VNode>,
    listeners: Vec<Listener>,
}

impl Script {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
            listeners: vec![],
        }
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("accesskey", value));
        self
    }

    pub fn r#async(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("async", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("autocapitalize", value));
        self
    }

    pub fn charset(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("charset", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("class", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("contextmenu", value));
        self
    }

    pub fn crossorigin(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("crossorigin", value));
        self
    }

    pub fn defer(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("defer", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("dir", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("draggable", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("hidden", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("id", value));
        self
    }

    pub fn importance(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("importance", value));
        self
    }

    pub fn integrity(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("integrity", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("itemprop", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("lang", value));
        self
    }

    pub fn language(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("language", value));
        self
    }

    pub fn referrerpolicy(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("referrerpolicy", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("slot", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("spellcheck", value));
        self
    }

    pub fn src(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("src", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("style", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("tabindex", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("translate", value));
        self
    }

    pub fn r#type(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("type", value));
        self
    }
}

impl_velement!(Script, "script");
pub struct Del {
    attributes: Vec<Attribute>,
    children: Vec<VNode>,
    listeners: Vec<Listener>,
}

impl Del {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
            listeners: vec![],
        }
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("accesskey", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("autocapitalize", value));
        self
    }

    pub fn cite(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("cite", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("class", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("contextmenu", value));
        self
    }

    pub fn datetime(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("datetime", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("dir", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("draggable", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("hidden", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("id", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("itemprop", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("lang", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("slot", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("spellcheck", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("style", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("tabindex", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("translate", value));
        self
    }
}

impl_velement!(Del, "del");
pub struct Ins {
    attributes: Vec<Attribute>,
    children: Vec<VNode>,
    listeners: Vec<Listener>,
}

impl Ins {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
            listeners: vec![],
        }
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("accesskey", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("autocapitalize", value));
        self
    }

    pub fn cite(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("cite", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("class", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("contextmenu", value));
        self
    }

    pub fn datetime(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("datetime", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("dir", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("draggable", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("hidden", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("id", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("itemprop", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("lang", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("slot", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("spellcheck", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("style", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("tabindex", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("translate", value));
        self
    }
}

impl_velement!(Ins, "ins");
pub struct Caption {
    attributes: Vec<Attribute>,
    children: Vec<VNode>,
    listeners: Vec<Listener>,
}

impl Caption {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
            listeners: vec![],
        }
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("accesskey", value));
        self
    }

    pub fn align(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("align", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("autocapitalize", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("class", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("contextmenu", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("dir", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("draggable", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("hidden", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("id", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("itemprop", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("lang", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("slot", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("spellcheck", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("style", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("tabindex", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("translate", value));
        self
    }
}

impl_velement!(Caption, "caption");
pub struct Col {
    attributes: Vec<Attribute>,
    children: Vec<VNode>,
    listeners: Vec<Listener>,
}

impl Col {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
            listeners: vec![],
        }
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("accesskey", value));
        self
    }

    pub fn align(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("align", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("autocapitalize", value));
        self
    }

    pub fn bgcolor(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("bgcolor", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("class", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("contextmenu", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("dir", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("draggable", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("hidden", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("id", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("itemprop", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("lang", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("slot", value));
        self
    }

    pub fn span(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("span", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("spellcheck", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("style", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("tabindex", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("translate", value));
        self
    }
}

impl_velement!(Col, "col");
pub struct Colgroup {
    attributes: Vec<Attribute>,
    children: Vec<VNode>,
    listeners: Vec<Listener>,
}

impl Colgroup {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
            listeners: vec![],
        }
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("accesskey", value));
        self
    }

    pub fn align(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("align", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("autocapitalize", value));
        self
    }

    pub fn bgcolor(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("bgcolor", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("class", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("contextmenu", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("dir", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("draggable", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("hidden", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("id", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("itemprop", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("lang", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("slot", value));
        self
    }

    pub fn span(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("span", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("spellcheck", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("style", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("tabindex", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("translate", value));
        self
    }
}

impl_velement!(Colgroup, "colgroup");
pub struct Table {
    attributes: Vec<Attribute>,
    children: Vec<VNode>,
    listeners: Vec<Listener>,
}

impl Table {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
            listeners: vec![],
        }
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("accesskey", value));
        self
    }

    pub fn align(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("align", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("autocapitalize", value));
        self
    }

    pub fn background(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("background", value));
        self
    }

    pub fn bgcolor(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("bgcolor", value));
        self
    }

    pub fn border(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("border", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("class", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("contextmenu", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("dir", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("draggable", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("hidden", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("id", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("itemprop", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("lang", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("slot", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("spellcheck", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("style", value));
        self
    }

    pub fn summary(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("summary", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("tabindex", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("translate", value));
        self
    }
}

impl_velement!(Table, "table");
pub struct Tbody {
    attributes: Vec<Attribute>,
    children: Vec<VNode>,
    listeners: Vec<Listener>,
}

impl Tbody {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
            listeners: vec![],
        }
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("accesskey", value));
        self
    }

    pub fn align(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("align", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("autocapitalize", value));
        self
    }

    pub fn bgcolor(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("bgcolor", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("class", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("contextmenu", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("dir", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("draggable", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("hidden", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("id", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("itemprop", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("lang", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("slot", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("spellcheck", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("style", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("tabindex", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("translate", value));
        self
    }
}

impl_velement!(Tbody, "tbody");
pub struct Td {
    attributes: Vec<Attribute>,
    children: Vec<VNode>,
    listeners: Vec<Listener>,
}

impl Td {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
            listeners: vec![],
        }
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("accesskey", value));
        self
    }

    pub fn align(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("align", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("autocapitalize", value));
        self
    }

    pub fn background(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("background", value));
        self
    }

    pub fn bgcolor(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("bgcolor", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("class", value));
        self
    }

    pub fn colspan(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("colspan", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("contextmenu", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("dir", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("draggable", value));
        self
    }

    pub fn headers(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("headers", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("hidden", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("id", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("itemprop", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("lang", value));
        self
    }

    pub fn rowspan(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("rowspan", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("slot", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("spellcheck", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("style", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("tabindex", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("translate", value));
        self
    }
}

impl_velement!(Td, "td");
pub struct Tfoot {
    attributes: Vec<Attribute>,
    children: Vec<VNode>,
    listeners: Vec<Listener>,
}

impl Tfoot {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
            listeners: vec![],
        }
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("accesskey", value));
        self
    }

    pub fn align(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("align", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("autocapitalize", value));
        self
    }

    pub fn bgcolor(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("bgcolor", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("class", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("contextmenu", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("dir", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("draggable", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("hidden", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("id", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("itemprop", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("lang", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("slot", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("spellcheck", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("style", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("tabindex", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("translate", value));
        self
    }
}

impl_velement!(Tfoot, "tfoot");
pub struct Th {
    attributes: Vec<Attribute>,
    children: Vec<VNode>,
    listeners: Vec<Listener>,
}

impl Th {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
            listeners: vec![],
        }
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("accesskey", value));
        self
    }

    pub fn align(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("align", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("autocapitalize", value));
        self
    }

    pub fn background(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("background", value));
        self
    }

    pub fn bgcolor(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("bgcolor", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("class", value));
        self
    }

    pub fn colspan(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("colspan", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("contextmenu", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("dir", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("draggable", value));
        self
    }

    pub fn headers(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("headers", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("hidden", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("id", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("itemprop", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("lang", value));
        self
    }

    pub fn rowspan(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("rowspan", value));
        self
    }

    pub fn scope(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("scope", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("slot", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("spellcheck", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("style", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("tabindex", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("translate", value));
        self
    }
}

impl_velement!(Th, "th");
pub struct Thead {
    attributes: Vec<Attribute>,
    children: Vec<VNode>,
    listeners: Vec<Listener>,
}

impl Thead {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
            listeners: vec![],
        }
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("accesskey", value));
        self
    }

    pub fn align(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("align", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("autocapitalize", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("class", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("contextmenu", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("dir", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("draggable", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("hidden", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("id", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("itemprop", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("lang", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("slot", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("spellcheck", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("style", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("tabindex", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("translate", value));
        self
    }
}

impl_velement!(Thead, "thead");
pub struct Tr {
    attributes: Vec<Attribute>,
    children: Vec<VNode>,
    listeners: Vec<Listener>,
}

impl Tr {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
            listeners: vec![],
        }
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("accesskey", value));
        self
    }

    pub fn align(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("align", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("autocapitalize", value));
        self
    }

    pub fn bgcolor(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("bgcolor", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("class", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("contextmenu", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("dir", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("draggable", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("hidden", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("id", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("itemprop", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("lang", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("slot", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("spellcheck", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("style", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("tabindex", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("translate", value));
        self
    }
}

impl_velement!(Tr, "tr");
pub struct Button {
    attributes: Vec<Attribute>,
    children: Vec<VNode>,
    listeners: Vec<Listener>,
}

impl Button {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
            listeners: vec![],
        }
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("accesskey", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("autocapitalize", value));
        self
    }

    pub fn autofocus(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("autofocus", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("class", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("contextmenu", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("dir", value));
        self
    }

    pub fn disabled(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("disabled", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("draggable", value));
        self
    }

    pub fn form(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("form", value));
        self
    }

    pub fn formaction(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("formaction", value));
        self
    }

    pub fn formenctype(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("formenctype", value));
        self
    }

    pub fn formmethod(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("formmethod", value));
        self
    }

    pub fn formnovalidate(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("formnovalidate", value));
        self
    }

    pub fn formtarget(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("formtarget", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("hidden", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("id", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("itemprop", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("lang", value));
        self
    }

    pub fn name(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("name", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("slot", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("spellcheck", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("style", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("tabindex", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("translate", value));
        self
    }

    pub fn r#type(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("type", value));
        self
    }

    pub fn value(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("value", value));
        self
    }
}

impl_velement!(Button, "button");
pub struct Datalist {
    attributes: Vec<Attribute>,
    children: Vec<VNode>,
    listeners: Vec<Listener>,
}

impl Datalist {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
            listeners: vec![],
        }
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("accesskey", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("autocapitalize", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("class", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("contextmenu", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("dir", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("draggable", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("hidden", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("id", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("itemprop", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("lang", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("slot", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("spellcheck", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("style", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("tabindex", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("translate", value));
        self
    }
}

impl_velement!(Datalist, "datalist");
pub struct Fieldset {
    attributes: Vec<Attribute>,
    children: Vec<VNode>,
    listeners: Vec<Listener>,
}

impl Fieldset {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
            listeners: vec![],
        }
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("accesskey", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("autocapitalize", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("class", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("contextmenu", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("dir", value));
        self
    }

    pub fn disabled(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("disabled", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("draggable", value));
        self
    }

    pub fn form(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("form", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("hidden", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("id", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("itemprop", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("lang", value));
        self
    }

    pub fn name(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("name", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("slot", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("spellcheck", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("style", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("tabindex", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("translate", value));
        self
    }
}

impl_velement!(Fieldset, "fieldset");
pub struct Form {
    attributes: Vec<Attribute>,
    children: Vec<VNode>,
    listeners: Vec<Listener>,
}

impl Form {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
            listeners: vec![],
        }
    }

    pub fn accept(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("accept", value));
        self
    }

    pub fn accept_charset(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("accept_charset", value));
        self
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("accesskey", value));
        self
    }

    pub fn action(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("action", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("autocapitalize", value));
        self
    }

    pub fn autocomplete(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("autocomplete", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("class", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("contextmenu", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("dir", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("draggable", value));
        self
    }

    pub fn enctype(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("enctype", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("hidden", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("id", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("itemprop", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("lang", value));
        self
    }

    pub fn method(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("method", value));
        self
    }

    pub fn name(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("name", value));
        self
    }

    pub fn novalidate(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("novalidate", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("slot", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("spellcheck", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("style", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("tabindex", value));
        self
    }

    pub fn target(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("target", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("translate", value));
        self
    }
}

impl_velement!(Form, "form");
pub struct Input {
    attributes: Vec<Attribute>,
    children: Vec<VNode>,
    listeners: Vec<Listener>,
}

impl Input {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
            listeners: vec![],
        }
    }

    pub fn accept(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("accept", value));
        self
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("accesskey", value));
        self
    }

    pub fn alt(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("alt", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("autocapitalize", value));
        self
    }

    pub fn autocomplete(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("autocomplete", value));
        self
    }

    pub fn autofocus(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("autofocus", value));
        self
    }

    pub fn capture(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("capture", value));
        self
    }

    pub fn checked(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("checked", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("class", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("contextmenu", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("dir", value));
        self
    }

    pub fn dirname(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("dirname", value));
        self
    }

    pub fn disabled(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("disabled", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("draggable", value));
        self
    }

    pub fn form(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("form", value));
        self
    }

    pub fn formaction(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("formaction", value));
        self
    }

    pub fn formenctype(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("formenctype", value));
        self
    }

    pub fn formmethod(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("formmethod", value));
        self
    }

    pub fn formnovalidate(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("formnovalidate", value));
        self
    }

    pub fn formtarget(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("formtarget", value));
        self
    }

    pub fn height(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("height", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("hidden", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("id", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("itemprop", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("lang", value));
        self
    }

    pub fn list(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("list", value));
        self
    }

    pub fn max(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("max", value));
        self
    }

    pub fn maxlength(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("maxlength", value));
        self
    }

    pub fn minlength(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("minlength", value));
        self
    }

    pub fn min(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("min", value));
        self
    }

    pub fn multiple(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("multiple", value));
        self
    }

    pub fn name(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("name", value));
        self
    }

    pub fn pattern(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("pattern", value));
        self
    }

    pub fn placeholder(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("placeholder", value));
        self
    }

    pub fn readonly(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("readonly", value));
        self
    }

    pub fn required(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("required", value));
        self
    }

    pub fn size(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("size", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("slot", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("spellcheck", value));
        self
    }

    pub fn src(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("src", value));
        self
    }

    pub fn step(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("step", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("style", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("tabindex", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("translate", value));
        self
    }

    pub fn r#type(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("type", value));
        self
    }

    pub fn usemap(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("usemap", value));
        self
    }

    pub fn value(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("value", value));
        self
    }

    pub fn width(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("width", value));
        self
    }
}

impl_velement!(Input, "input");
pub struct Label {
    attributes: Vec<Attribute>,
    children: Vec<VNode>,
    listeners: Vec<Listener>,
}

impl Label {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
            listeners: vec![],
        }
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("accesskey", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("autocapitalize", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("class", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("contextmenu", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("dir", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("draggable", value));
        self
    }

    pub fn r#for(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("for", value));
        self
    }

    pub fn form(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("form", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("hidden", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("id", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("itemprop", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("lang", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("slot", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("spellcheck", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("style", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("tabindex", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("translate", value));
        self
    }
}

impl_velement!(Label, "label");
pub struct Legend {
    attributes: Vec<Attribute>,
    children: Vec<VNode>,
    listeners: Vec<Listener>,
}

impl Legend {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
            listeners: vec![],
        }
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("accesskey", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("autocapitalize", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("class", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("contextmenu", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("dir", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("draggable", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("hidden", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("id", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("itemprop", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("lang", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("slot", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("spellcheck", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("style", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("tabindex", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("translate", value));
        self
    }
}

impl_velement!(Legend, "legend");
pub struct Meter {
    attributes: Vec<Attribute>,
    children: Vec<VNode>,
    listeners: Vec<Listener>,
}

impl Meter {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
            listeners: vec![],
        }
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("accesskey", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("autocapitalize", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("class", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("contextmenu", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("dir", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("draggable", value));
        self
    }

    pub fn form(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("form", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("hidden", value));
        self
    }

    pub fn high(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("high", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("id", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("itemprop", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("lang", value));
        self
    }

    pub fn low(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("low", value));
        self
    }

    pub fn max(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("max", value));
        self
    }

    pub fn min(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("min", value));
        self
    }

    pub fn optimum(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("optimum", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("slot", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("spellcheck", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("style", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("tabindex", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("translate", value));
        self
    }

    pub fn value(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("value", value));
        self
    }
}

impl_velement!(Meter, "meter");
pub struct Optgroup {
    attributes: Vec<Attribute>,
    children: Vec<VNode>,
    listeners: Vec<Listener>,
}

impl Optgroup {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
            listeners: vec![],
        }
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("accesskey", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("autocapitalize", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("class", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("contextmenu", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("dir", value));
        self
    }

    pub fn disabled(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("disabled", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("draggable", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("hidden", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("id", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("itemprop", value));
        self
    }

    pub fn label(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("label", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("lang", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("slot", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("spellcheck", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("style", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("tabindex", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("translate", value));
        self
    }
}

impl_velement!(Optgroup, "optgroup");
pub struct Option {
    attributes: Vec<Attribute>,
    children: Vec<VNode>,
    listeners: Vec<Listener>,
}

impl Option {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
            listeners: vec![],
        }
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("accesskey", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("autocapitalize", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("class", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("contextmenu", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("dir", value));
        self
    }

    pub fn disabled(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("disabled", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("draggable", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("hidden", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("id", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("itemprop", value));
        self
    }

    pub fn label(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("label", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("lang", value));
        self
    }

    pub fn selected(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("selected", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("slot", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("spellcheck", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("style", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("tabindex", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("translate", value));
        self
    }

    pub fn value(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("value", value));
        self
    }
}

impl_velement!(Option, "option");
pub struct Output {
    attributes: Vec<Attribute>,
    children: Vec<VNode>,
    listeners: Vec<Listener>,
}

impl Output {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
            listeners: vec![],
        }
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("accesskey", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("autocapitalize", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("class", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("contextmenu", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("dir", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("draggable", value));
        self
    }

    pub fn r#for(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("for", value));
        self
    }

    pub fn form(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("form", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("hidden", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("id", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("itemprop", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("lang", value));
        self
    }

    pub fn name(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("name", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("slot", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("spellcheck", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("style", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("tabindex", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("translate", value));
        self
    }
}

impl_velement!(Output, "output");
pub struct Progress {
    attributes: Vec<Attribute>,
    children: Vec<VNode>,
    listeners: Vec<Listener>,
}

impl Progress {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
            listeners: vec![],
        }
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("accesskey", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("autocapitalize", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("class", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("contextmenu", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("dir", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("draggable", value));
        self
    }

    pub fn form(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("form", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("hidden", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("id", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("itemprop", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("lang", value));
        self
    }

    pub fn max(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("max", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("slot", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("spellcheck", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("style", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("tabindex", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("translate", value));
        self
    }

    pub fn value(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("value", value));
        self
    }
}

impl_velement!(Progress, "progress");
pub struct Select {
    attributes: Vec<Attribute>,
    children: Vec<VNode>,
    listeners: Vec<Listener>,
}

impl Select {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
            listeners: vec![],
        }
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("accesskey", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("autocapitalize", value));
        self
    }

    pub fn autocomplete(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("autocomplete", value));
        self
    }

    pub fn autofocus(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("autofocus", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("class", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("contextmenu", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("dir", value));
        self
    }

    pub fn disabled(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("disabled", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("draggable", value));
        self
    }

    pub fn form(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("form", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("hidden", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("id", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("itemprop", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("lang", value));
        self
    }

    pub fn multiple(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("multiple", value));
        self
    }

    pub fn name(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("name", value));
        self
    }

    pub fn required(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("required", value));
        self
    }

    pub fn size(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("size", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("slot", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("spellcheck", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("style", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("tabindex", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("translate", value));
        self
    }
}

impl_velement!(Select, "select");
pub struct Textarea {
    attributes: Vec<Attribute>,
    children: Vec<VNode>,
    listeners: Vec<Listener>,
}

impl Textarea {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
            listeners: vec![],
        }
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("accesskey", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("autocapitalize", value));
        self
    }

    pub fn autocomplete(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("autocomplete", value));
        self
    }

    pub fn autofocus(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("autofocus", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("class", value));
        self
    }

    pub fn cols(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("cols", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("contextmenu", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("dir", value));
        self
    }

    pub fn dirname(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("dirname", value));
        self
    }

    pub fn disabled(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("disabled", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("draggable", value));
        self
    }

    pub fn enterkeyhint(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("enterkeyhint", value));
        self
    }

    pub fn form(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("form", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("hidden", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("id", value));
        self
    }

    pub fn inputmode(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("inputmode", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("itemprop", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("lang", value));
        self
    }

    pub fn maxlength(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("maxlength", value));
        self
    }

    pub fn minlength(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("minlength", value));
        self
    }

    pub fn name(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("name", value));
        self
    }

    pub fn placeholder(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("placeholder", value));
        self
    }

    pub fn readonly(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("readonly", value));
        self
    }

    pub fn required(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("required", value));
        self
    }

    pub fn rows(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("rows", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("slot", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("spellcheck", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("style", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("tabindex", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("translate", value));
        self
    }

    pub fn wrap(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("wrap", value));
        self
    }
}

impl_velement!(Textarea, "textarea");
pub struct Details {
    attributes: Vec<Attribute>,
    children: Vec<VNode>,
    listeners: Vec<Listener>,
}

impl Details {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
            listeners: vec![],
        }
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("accesskey", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("autocapitalize", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("class", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("contextmenu", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("dir", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("draggable", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("hidden", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("id", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("itemprop", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("lang", value));
        self
    }

    pub fn open(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("open", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("slot", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("spellcheck", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("style", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("tabindex", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("translate", value));
        self
    }
}

impl_velement!(Details, "details");
pub struct Dialog {
    attributes: Vec<Attribute>,
    children: Vec<VNode>,
    listeners: Vec<Listener>,
}

impl Dialog {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
            listeners: vec![],
        }
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("accesskey", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("autocapitalize", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("class", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("contextmenu", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("dir", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("draggable", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("hidden", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("id", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("itemprop", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("lang", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("slot", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("spellcheck", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("style", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("tabindex", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("translate", value));
        self
    }
}

impl_velement!(Dialog, "dialog");
pub struct Menu {
    attributes: Vec<Attribute>,
    children: Vec<VNode>,
    listeners: Vec<Listener>,
}

impl Menu {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
            listeners: vec![],
        }
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("accesskey", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("autocapitalize", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("class", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("contextmenu", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("dir", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("draggable", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("hidden", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("id", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("itemprop", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("lang", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("slot", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("spellcheck", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("style", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("tabindex", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("translate", value));
        self
    }

    pub fn r#type(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("type", value));
        self
    }
}

impl_velement!(Menu, "menu");
pub struct Summary {
    attributes: Vec<Attribute>,
    children: Vec<VNode>,
    listeners: Vec<Listener>,
}

impl Summary {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
            listeners: vec![],
        }
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("accesskey", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("autocapitalize", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("class", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("contextmenu", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("dir", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("draggable", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("hidden", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("id", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("itemprop", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("lang", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("slot", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("spellcheck", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("style", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("tabindex", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("translate", value));
        self
    }
}

impl_velement!(Summary, "summary");
pub struct Slot {
    attributes: Vec<Attribute>,
    children: Vec<VNode>,
    listeners: Vec<Listener>,
}

impl Slot {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
            listeners: vec![],
        }
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("accesskey", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("autocapitalize", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("class", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("contextmenu", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("dir", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("draggable", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("hidden", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("id", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("itemprop", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("lang", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("slot", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("spellcheck", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("style", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("tabindex", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("translate", value));
        self
    }
}

impl_velement!(Slot, "slot");
pub struct Template {
    attributes: Vec<Attribute>,
    children: Vec<VNode>,
    listeners: Vec<Listener>,
}

impl Template {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
            listeners: vec![],
        }
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("accesskey", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("autocapitalize", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("class", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("contextmenu", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("dir", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("draggable", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("hidden", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("id", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("itemprop", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("lang", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("slot", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("spellcheck", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("style", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("tabindex", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("translate", value));
        self
    }
}

impl_velement!(Template, "template");
pub struct Acronym {
    attributes: Vec<Attribute>,
    children: Vec<VNode>,
    listeners: Vec<Listener>,
}

impl Acronym {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
            listeners: vec![],
        }
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("accesskey", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("autocapitalize", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("class", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("contextmenu", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("dir", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("draggable", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("hidden", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("id", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("itemprop", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("lang", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("slot", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("spellcheck", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("style", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("tabindex", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("translate", value));
        self
    }
}

impl_velement!(Acronym, "acronym");
pub struct Applet {
    attributes: Vec<Attribute>,
    children: Vec<VNode>,
    listeners: Vec<Listener>,
}

impl Applet {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
            listeners: vec![],
        }
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("accesskey", value));
        self
    }

    pub fn align(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("align", value));
        self
    }

    pub fn alt(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("alt", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("autocapitalize", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("class", value));
        self
    }

    pub fn code(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("code", value));
        self
    }

    pub fn codebase(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("codebase", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("contextmenu", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("dir", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("draggable", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("hidden", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("id", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("itemprop", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("lang", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("slot", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("spellcheck", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("style", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("tabindex", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("translate", value));
        self
    }
}

impl_velement!(Applet, "applet");
pub struct Basefont {
    attributes: Vec<Attribute>,
    children: Vec<VNode>,
    listeners: Vec<Listener>,
}

impl Basefont {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
            listeners: vec![],
        }
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("accesskey", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("autocapitalize", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("class", value));
        self
    }

    pub fn color(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("color", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("contextmenu", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("dir", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("draggable", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("hidden", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("id", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("itemprop", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("lang", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("slot", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("spellcheck", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("style", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("tabindex", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("translate", value));
        self
    }
}

impl_velement!(Basefont, "basefont");
pub struct Bgsound {
    attributes: Vec<Attribute>,
    children: Vec<VNode>,
    listeners: Vec<Listener>,
}

impl Bgsound {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
            listeners: vec![],
        }
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("accesskey", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("autocapitalize", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("class", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("contextmenu", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("dir", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("draggable", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("hidden", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("id", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("itemprop", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("lang", value));
        self
    }

    pub fn r#loop(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("loop", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("slot", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("spellcheck", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("style", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("tabindex", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("translate", value));
        self
    }
}

impl_velement!(Bgsound, "bgsound");
pub struct Big {
    attributes: Vec<Attribute>,
    children: Vec<VNode>,
    listeners: Vec<Listener>,
}

impl Big {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
            listeners: vec![],
        }
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("accesskey", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("autocapitalize", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("class", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("contextmenu", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("dir", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("draggable", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("hidden", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("id", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("itemprop", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("lang", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("slot", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("spellcheck", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("style", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("tabindex", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("translate", value));
        self
    }
}

impl_velement!(Big, "big");
pub struct Blink {
    attributes: Vec<Attribute>,
    children: Vec<VNode>,
    listeners: Vec<Listener>,
}

impl Blink {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
            listeners: vec![],
        }
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("accesskey", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("autocapitalize", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("class", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("contextmenu", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("dir", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("draggable", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("hidden", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("id", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("itemprop", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("lang", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("slot", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("spellcheck", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("style", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("tabindex", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("translate", value));
        self
    }
}

impl_velement!(Blink, "blink");
pub struct Center {
    attributes: Vec<Attribute>,
    children: Vec<VNode>,
    listeners: Vec<Listener>,
}

impl Center {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
            listeners: vec![],
        }
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("accesskey", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("autocapitalize", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("class", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("contextmenu", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("dir", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("draggable", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("hidden", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("id", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("itemprop", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("lang", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("slot", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("spellcheck", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("style", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("tabindex", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("translate", value));
        self
    }
}

impl_velement!(Center, "center");
pub struct Content {
    attributes: Vec<Attribute>,
    children: Vec<VNode>,
    listeners: Vec<Listener>,
}

impl Content {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
            listeners: vec![],
        }
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("accesskey", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("autocapitalize", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("class", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("contextmenu", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("dir", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("draggable", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("hidden", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("id", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("itemprop", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("lang", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("slot", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("spellcheck", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("style", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("tabindex", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("translate", value));
        self
    }
}

impl_velement!(Content, "content");
pub struct Dir {
    attributes: Vec<Attribute>,
    children: Vec<VNode>,
    listeners: Vec<Listener>,
}

impl Dir {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
            listeners: vec![],
        }
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("accesskey", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("autocapitalize", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("class", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("contextmenu", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("dir", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("draggable", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("hidden", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("id", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("itemprop", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("lang", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("slot", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("spellcheck", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("style", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("tabindex", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("translate", value));
        self
    }
}

impl_velement!(Dir, "dir");
pub struct Font {
    attributes: Vec<Attribute>,
    children: Vec<VNode>,
    listeners: Vec<Listener>,
}

impl Font {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
            listeners: vec![],
        }
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("accesskey", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("autocapitalize", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("class", value));
        self
    }

    pub fn color(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("color", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("contextmenu", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("dir", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("draggable", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("hidden", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("id", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("itemprop", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("lang", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("slot", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("spellcheck", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("style", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("tabindex", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("translate", value));
        self
    }
}

impl_velement!(Font, "font");
pub struct Frame {
    attributes: Vec<Attribute>,
    children: Vec<VNode>,
    listeners: Vec<Listener>,
}

impl Frame {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
            listeners: vec![],
        }
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("accesskey", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("autocapitalize", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("class", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("contextmenu", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("dir", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("draggable", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("hidden", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("id", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("itemprop", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("lang", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("slot", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("spellcheck", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("style", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("tabindex", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("translate", value));
        self
    }
}

impl_velement!(Frame, "frame");
pub struct Frameset {
    attributes: Vec<Attribute>,
    children: Vec<VNode>,
    listeners: Vec<Listener>,
}

impl Frameset {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
            listeners: vec![],
        }
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("accesskey", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("autocapitalize", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("class", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("contextmenu", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("dir", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("draggable", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("hidden", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("id", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("itemprop", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("lang", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("slot", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("spellcheck", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("style", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("tabindex", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("translate", value));
        self
    }
}

impl_velement!(Frameset, "frameset");
pub struct Hgroup {
    attributes: Vec<Attribute>,
    children: Vec<VNode>,
    listeners: Vec<Listener>,
}

impl Hgroup {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
            listeners: vec![],
        }
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("accesskey", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("autocapitalize", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("class", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("contextmenu", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("dir", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("draggable", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("hidden", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("id", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("itemprop", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("lang", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("slot", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("spellcheck", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("style", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("tabindex", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("translate", value));
        self
    }
}

impl_velement!(Hgroup, "hgroup");
pub struct Image {
    attributes: Vec<Attribute>,
    children: Vec<VNode>,
    listeners: Vec<Listener>,
}

impl Image {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
            listeners: vec![],
        }
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("accesskey", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("autocapitalize", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("class", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("contextmenu", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("dir", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("draggable", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("hidden", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("id", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("itemprop", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("lang", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("slot", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("spellcheck", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("style", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("tabindex", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("translate", value));
        self
    }
}

impl_velement!(Image, "image");
pub struct Keygen {
    attributes: Vec<Attribute>,
    children: Vec<VNode>,
    listeners: Vec<Listener>,
}

impl Keygen {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
            listeners: vec![],
        }
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("accesskey", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("autocapitalize", value));
        self
    }

    pub fn autofocus(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("autofocus", value));
        self
    }

    pub fn challenge(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("challenge", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("class", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("contextmenu", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("dir", value));
        self
    }

    pub fn disabled(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("disabled", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("draggable", value));
        self
    }

    pub fn form(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("form", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("hidden", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("id", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("itemprop", value));
        self
    }

    pub fn keytype(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("keytype", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("lang", value));
        self
    }

    pub fn name(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("name", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("slot", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("spellcheck", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("style", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("tabindex", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("translate", value));
        self
    }
}

impl_velement!(Keygen, "keygen");
pub struct Marquee {
    attributes: Vec<Attribute>,
    children: Vec<VNode>,
    listeners: Vec<Listener>,
}

impl Marquee {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
            listeners: vec![],
        }
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("accesskey", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("autocapitalize", value));
        self
    }

    pub fn bgcolor(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("bgcolor", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("class", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("contextmenu", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("dir", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("draggable", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("hidden", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("id", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("itemprop", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("lang", value));
        self
    }

    pub fn r#loop(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("loop", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("slot", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("spellcheck", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("style", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("tabindex", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("translate", value));
        self
    }
}

impl_velement!(Marquee, "marquee");
pub struct Menuitem {
    attributes: Vec<Attribute>,
    children: Vec<VNode>,
    listeners: Vec<Listener>,
}

impl Menuitem {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
            listeners: vec![],
        }
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("accesskey", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("autocapitalize", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("class", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("contextmenu", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("dir", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("draggable", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("hidden", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("id", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("itemprop", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("lang", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("slot", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("spellcheck", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("style", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("tabindex", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("translate", value));
        self
    }
}

impl_velement!(Menuitem, "menuitem");
pub struct Nobr {
    attributes: Vec<Attribute>,
    children: Vec<VNode>,
    listeners: Vec<Listener>,
}

impl Nobr {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
            listeners: vec![],
        }
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("accesskey", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("autocapitalize", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("class", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("contextmenu", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("dir", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("draggable", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("hidden", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("id", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("itemprop", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("lang", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("slot", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("spellcheck", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("style", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("tabindex", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("translate", value));
        self
    }
}

impl_velement!(Nobr, "nobr");
pub struct Noembed {
    attributes: Vec<Attribute>,
    children: Vec<VNode>,
    listeners: Vec<Listener>,
}

impl Noembed {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
            listeners: vec![],
        }
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("accesskey", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("autocapitalize", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("class", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("contextmenu", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("dir", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("draggable", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("hidden", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("id", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("itemprop", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("lang", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("slot", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("spellcheck", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("style", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("tabindex", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("translate", value));
        self
    }
}

impl_velement!(Noembed, "noembed");
pub struct Noframes {
    attributes: Vec<Attribute>,
    children: Vec<VNode>,
    listeners: Vec<Listener>,
}

impl Noframes {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
            listeners: vec![],
        }
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("accesskey", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("autocapitalize", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("class", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("contextmenu", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("dir", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("draggable", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("hidden", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("id", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("itemprop", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("lang", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("slot", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("spellcheck", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("style", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("tabindex", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("translate", value));
        self
    }
}

impl_velement!(Noframes, "noframes");
pub struct Plaintext {
    attributes: Vec<Attribute>,
    children: Vec<VNode>,
    listeners: Vec<Listener>,
}

impl Plaintext {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
            listeners: vec![],
        }
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("accesskey", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("autocapitalize", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("class", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("contextmenu", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("dir", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("draggable", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("hidden", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("id", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("itemprop", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("lang", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("slot", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("spellcheck", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("style", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("tabindex", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("translate", value));
        self
    }
}

impl_velement!(Plaintext, "plaintext");
pub struct Rb {
    attributes: Vec<Attribute>,
    children: Vec<VNode>,
    listeners: Vec<Listener>,
}

impl Rb {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
            listeners: vec![],
        }
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("accesskey", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("autocapitalize", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("class", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("contextmenu", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("dir", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("draggable", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("hidden", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("id", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("itemprop", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("lang", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("slot", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("spellcheck", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("style", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("tabindex", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("translate", value));
        self
    }
}

impl_velement!(Rb, "rb");
pub struct Rtc {
    attributes: Vec<Attribute>,
    children: Vec<VNode>,
    listeners: Vec<Listener>,
}

impl Rtc {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
            listeners: vec![],
        }
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("accesskey", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("autocapitalize", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("class", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("contextmenu", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("dir", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("draggable", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("hidden", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("id", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("itemprop", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("lang", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("slot", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("spellcheck", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("style", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("tabindex", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("translate", value));
        self
    }
}

impl_velement!(Rtc, "rtc");
pub struct Shadow {
    attributes: Vec<Attribute>,
    children: Vec<VNode>,
    listeners: Vec<Listener>,
}

impl Shadow {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
            listeners: vec![],
        }
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("accesskey", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("autocapitalize", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("class", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("contextmenu", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("dir", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("draggable", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("hidden", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("id", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("itemprop", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("lang", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("slot", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("spellcheck", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("style", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("tabindex", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("translate", value));
        self
    }
}

impl_velement!(Shadow, "shadow");
pub struct Spacer {
    attributes: Vec<Attribute>,
    children: Vec<VNode>,
    listeners: Vec<Listener>,
}

impl Spacer {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
            listeners: vec![],
        }
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("accesskey", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("autocapitalize", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("class", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("contextmenu", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("dir", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("draggable", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("hidden", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("id", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("itemprop", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("lang", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("slot", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("spellcheck", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("style", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("tabindex", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("translate", value));
        self
    }
}

impl_velement!(Spacer, "spacer");
pub struct Strike {
    attributes: Vec<Attribute>,
    children: Vec<VNode>,
    listeners: Vec<Listener>,
}

impl Strike {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
            listeners: vec![],
        }
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("accesskey", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("autocapitalize", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("class", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("contextmenu", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("dir", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("draggable", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("hidden", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("id", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("itemprop", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("lang", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("slot", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("spellcheck", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("style", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("tabindex", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("translate", value));
        self
    }
}

impl_velement!(Strike, "strike");
pub struct Tt {
    attributes: Vec<Attribute>,
    children: Vec<VNode>,
    listeners: Vec<Listener>,
}

impl Tt {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
            listeners: vec![],
        }
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("accesskey", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("autocapitalize", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("class", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("contextmenu", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("dir", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("draggable", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("hidden", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("id", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("itemprop", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("lang", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("slot", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("spellcheck", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("style", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("tabindex", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("translate", value));
        self
    }
}

impl_velement!(Tt, "tt");
pub struct Xmp {
    attributes: Vec<Attribute>,
    children: Vec<VNode>,
    listeners: Vec<Listener>,
}

impl Xmp {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
            listeners: vec![],
        }
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("accesskey", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("autocapitalize", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("class", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("contextmenu", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("dir", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("draggable", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("hidden", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("id", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("itemprop", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("lang", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("slot", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("spellcheck", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("style", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("tabindex", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("translate", value));
        self
    }
}

impl_velement!(Xmp, "xmp");
pub struct H1 {
    attributes: Vec<Attribute>,
    children: Vec<VNode>,
    listeners: Vec<Listener>,
}

impl H1 {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
            listeners: vec![],
        }
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("accesskey", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("autocapitalize", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("class", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("contextmenu", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("dir", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("draggable", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("hidden", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("id", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("itemprop", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("lang", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("slot", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("spellcheck", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("style", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("tabindex", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("translate", value));
        self
    }
}

impl_velement!(H1, "h1");
pub struct H2 {
    attributes: Vec<Attribute>,
    children: Vec<VNode>,
    listeners: Vec<Listener>,
}

impl H2 {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
            listeners: vec![],
        }
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("accesskey", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("autocapitalize", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("class", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("contextmenu", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("dir", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("draggable", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("hidden", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("id", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("itemprop", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("lang", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("slot", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("spellcheck", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("style", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("tabindex", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("translate", value));
        self
    }
}

impl_velement!(H2, "h2");
pub struct H3 {
    attributes: Vec<Attribute>,
    children: Vec<VNode>,
    listeners: Vec<Listener>,
}

impl H3 {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
            listeners: vec![],
        }
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("accesskey", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("autocapitalize", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("class", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("contextmenu", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("dir", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("draggable", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("hidden", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("id", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("itemprop", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("lang", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("slot", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("spellcheck", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("style", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("tabindex", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("translate", value));
        self
    }
}

impl_velement!(H3, "h3");
pub struct H4 {
    attributes: Vec<Attribute>,
    children: Vec<VNode>,
    listeners: Vec<Listener>,
}

impl H4 {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
            listeners: vec![],
        }
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("accesskey", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("autocapitalize", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("class", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("contextmenu", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("dir", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("draggable", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("hidden", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("id", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("itemprop", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("lang", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("slot", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("spellcheck", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("style", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("tabindex", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("translate", value));
        self
    }
}

impl_velement!(H4, "h4");
pub struct H5 {
    attributes: Vec<Attribute>,
    children: Vec<VNode>,
    listeners: Vec<Listener>,
}

impl H5 {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
            listeners: vec![],
        }
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("accesskey", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("autocapitalize", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("class", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("contextmenu", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("dir", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("draggable", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("hidden", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("id", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("itemprop", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("lang", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("slot", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("spellcheck", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("style", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("tabindex", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("translate", value));
        self
    }
}

impl_velement!(H5, "h5");
pub struct H6 {
    attributes: Vec<Attribute>,
    children: Vec<VNode>,
    listeners: Vec<Listener>,
}

impl H6 {
    pub(crate) fn new() -> Self {
        Self {
            attributes: vec![],
            children: vec![],
            listeners: vec![],
        }
    }

    pub fn accesskey(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("accesskey", value));
        self
    }

    pub fn autocapitalize(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("autocapitalize", value));
        self
    }

    pub fn class(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("class", value));
        self
    }

    pub fn contenteditable(mut self, value: AttrValue) -> Self {
        self.attributes
            .push(Attribute::new("contenteditable", value));
        self
    }

    pub fn contextmenu(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("contextmenu", value));
        self
    }

    pub fn dir(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("dir", value));
        self
    }

    pub fn draggable(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("draggable", value));
        self
    }

    pub fn hidden(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("hidden", value));
        self
    }

    pub fn id(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("id", value));
        self
    }

    pub fn itemprop(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("itemprop", value));
        self
    }

    pub fn lang(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("lang", value));
        self
    }

    pub fn slot(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("slot", value));
        self
    }

    pub fn spellcheck(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("spellcheck", value));
        self
    }

    pub fn style(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("style", value));
        self
    }

    pub fn tabindex(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("tabindex", value));
        self
    }

    pub fn title(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("title", value));
        self
    }

    pub fn translate(mut self, value: AttrValue) -> Self {
        self.attributes.push(Attribute::new("translate", value));
        self
    }
}

impl_velement!(H6, "h6");
