import ast

KEYWORDS = [
    'for',
    'loop',
    'async',
    'type'
]

CURLY_BRACE_OPEN = '{'
CURLY_BRACE_CLOSE = '}'

BOOLEAN_ATTRS = [
    'allowfullscreen',
    'allowpaymentrequest',
    'async',
    'autofocus',
    'autoplay',
    'checked',
    'controls',
    'default',
    'disabled',
    'formnovalidate',
    'hidden',
    'ismap',
    'itemscope',
    'loop',
    'multiple',
    'muted',
    'nomodule',
    'novalidate',
    'open',
    'playsinline',
    'readonly',
    'required',
    'reversed',
    'selected',
    'truespeed',
]

ELEMENT_FUNCTIONS = '''
    pub fn child(mut self, element: impl Into<VTag>) -> Self {
        self.children.push(VNode::from(element.into()));
        self
    }

    pub fn component<C: Component>(mut self, props: C::Properties) -> Self {
        self.children.push(VNode::from(VComp::new::<C>(props, NodeRef::default(), None)));
        self
    }

    pub fn text(mut self, text: impl Into<AttrValue>) -> Self {
        self.children.push(VNode::from(VText::new(text)));
        self
    }
'''

elements_rs = []
functions_rs = []

e = ast.get_elements_with_attrs()


def map_attrs_to_fns(attrs):
    output = []
    for attr in attrs:
        ident = attr if attr not in KEYWORDS else f'r#{attr}'
        output.append(f'''
    pub fn {ident}(mut self, value: AttrValue) -> Self {CURLY_BRACE_OPEN}
        self.attributes.push(PositionalAttr::new("{attr}", value));
        self
    {CURLY_BRACE_CLOSE}
        ''')
    return ''.join(output)


for (element_name, element_attrs) in e.items():
    attrs, default = [attr.replace('-', '_') for attr in element_attrs['attrs']], element_attrs['default']
    element_name = element_name.strip('>').strip('<')
    ident = element_name.title()

    elements_rs.append(f'''
pub struct {ident} {CURLY_BRACE_OPEN}
    attributes: Vec<PositionalAttr>,
    children: Vec<VNode>,
{CURLY_BRACE_CLOSE}

impl {ident} {CURLY_BRACE_OPEN}
    pub(crate) fn new() -> Self {CURLY_BRACE_OPEN}
        Self {CURLY_BRACE_OPEN}
            attributes: vec![],
            children: vec![],
        {CURLY_BRACE_CLOSE}
    {CURLY_BRACE_CLOSE}
    
    {map_attrs_to_fns(attrs)}
    
    {ELEMENT_FUNCTIONS}
{CURLY_BRACE_CLOSE}

impl Into<VTag> for {ident} {CURLY_BRACE_OPEN}
    fn into(self) -> VTag {CURLY_BRACE_OPEN}
        let mut vtag = VTag::new("{element_name}");
        vtag.attributes = Attributes::Vec(self.attributes);
        vtag.add_children(self.children.into_iter());
        vtag
    {CURLY_BRACE_CLOSE}
{CURLY_BRACE_CLOSE}

impl Into<VNode> for {ident} {CURLY_BRACE_OPEN}
    fn into(self) -> VNode {CURLY_BRACE_OPEN}
        let vtag: VTag = self.into();
        VNode::from(vtag)
    {CURLY_BRACE_CLOSE}
{CURLY_BRACE_CLOSE}
    ''')

    param = '' if default is None else 'value: impl Into<AttrValue>'
    call = '' if default is None else '.text(value.into())'
    functions_rs.append(f'''
    pub fn {element_name}({param}) -> {ident} {CURLY_BRACE_OPEN}
        {ident}::new()
            {call}
    {CURLY_BRACE_CLOSE}
    ''')

with open('../src/elements.rs', 'w') as f:
    f.write('use yew::{Component, NodeRef};')
    f.write('use yew::virtual_dom::{PositionalAttr, VNode, AttrValue, VText, VComp, VTag, Attributes};')
    f.write('\n'.join(elements_rs))

with open('../src/functions.rs', 'w') as f:
    f.write('use yew::virtual_dom::{AttrValue};')
    f.write('use crate::elements::*;')
    f.write('\n'.join(functions_rs))
