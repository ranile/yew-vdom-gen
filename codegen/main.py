import ast

KEYWORDS = [
    'for',
    'loop',
    'async',
    'type'
]

CURLY_BRACE_OPEN = '{'
CURLY_BRACE_CLOSE = '}'
NEW_LINE = '\n'
TAB = '\t'

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

LISTENERS = {
    "abort": "web_sys::Event",
    "auxclick": "yew::MouseEvent",
    "blur": "web_sys::FocusEvent",
    "cancel": "web_sys::Event",
    "canplay": "web_sys::Event",
    "canplaythrough": "web_sys::Event",
    "change": "web_sys::Event",
    "click": "yew::MouseEvent",
    "close": "web_sys::Event",
    "contextmenu": "yew::MouseEvent",
    "cuechange": "web_sys::Event",
    "dblclick": "yew::MouseEvent",
    "drag": "web_sys::DragEvent",
    "dragend": "web_sys::DragEvent",
    "dragenter": "web_sys::DragEvent",
    "dragexit": "web_sys::DragEvent",
    "dragleave": "web_sys::DragEvent",
    "dragover": "web_sys::DragEvent",
    "dragstart": "web_sys::DragEvent",
    "drop": "web_sys::DragEvent",
    "durationchange": "web_sys::Event",
    "emptied": "web_sys::Event",
    "ended": "web_sys::Event",
    "error": "web_sys::Event",
    "focus": "web_sys::FocusEvent",
    "formdata": "web_sys::Event",
    "input": "web_sys::InputEvent",
    "invalid": "web_sys::Event",
    "keydown": "web_sys::KeyboardEvent",
    "keypress": "web_sys::KeyboardEvent",
    "keyup": "web_sys::KeyboardEvent",
    "load": "web_sys::Event",
    "loadeddata": "web_sys::Event",
    "loadedmetadata": "web_sys::Event",
    "loadstart": "web_sys::ProgressEvent",
    "mousedown": "yew::MouseEvent",
    "mouseenter": "yew::MouseEvent",
    "mouseleave": "yew::MouseEvent",
    "mousemove": "yew::MouseEvent",
    "mouseout": "yew::MouseEvent",
    "mouseover": "yew::MouseEvent",
    "mouseup": "yew::MouseEvent",
    "pause": "web_sys::Event",
    "play": "web_sys::Event",
    "playing": "web_sys::Event",
    "progress": "web_sys::ProgressEvent",
    "ratechange": "web_sys::Event",
    "reset": "web_sys::Event",
    "resize": "web_sys::Event",
    "scroll": "web_sys::Event",
    "securitypolicyviolation": "web_sys::Event",
    "seeked": "web_sys::Event",
    "seeking": "web_sys::Event",
    "select": "web_sys::Event",
    "slotchange": "web_sys::Event",
    "stalled": "web_sys::Event",
    "submit": "web_sys::FocusEvent",
    "suspend": "web_sys::Event",
    "timeupdate": "web_sys::Event",
    "toggle": "web_sys::Event",
    "volumechange": "web_sys::Event",
    "waiting": "web_sys::Event",
    "wheel": "web_sys::WheelEvent",
    "copy": "web_sys::Event",
    "cut": "web_sys::Event",
    "paste": "web_sys::Event",
    "animationcancel": "web_sys::AnimationEvent",
    "animationend": "web_sys::AnimationEvent",
    "animationiteration": "web_sys::AnimationEvent",
    "animationstart": "web_sys::AnimationEvent",
    "gotpointercapture": "web_sys::PointerEvent",
    "loadend": "web_sys::ProgressEvent",
    "lostpointercapture": "web_sys::PointerEvent",
    "pointercancel": "web_sys::PointerEvent",
    "pointerdown": "web_sys::PointerEvent",
    "pointerenter": "web_sys::PointerEvent",
    "pointerleave": "web_sys::PointerEvent",
    "pointerlockchange": "web_sys::Event",
    "pointerlockerror": "web_sys::Event",
    "pointermove": "web_sys::PointerEvent",
    "pointerout": "web_sys::PointerEvent",
    "pointerover": "web_sys::PointerEvent",
    "pointerup": "web_sys::PointerEvent",
    "selectionchange": "web_sys::Event",
    "selectstart": "web_sys::Event",
    "show": "web_sys::Event",
    "touchcancel": "web_sys::TouchEvent",
    "touchend": "web_sys::TouchEvent",
    "touchmove": "web_sys::TouchEvent",
    "touchstart": "web_sys::TouchEvent",
    "transitioncancel": "web_sys::TransitionEvent",
    "transitionend": "web_sys::TransitionEvent",
    "transitionrun": "web_sys::TransitionEvent",
    "transitionstart": "web_sys::TransitionEvent",
}

elements_rs = []
functions_rs = []
listeners_rs = []

e = ast.get_elements_with_attrs()


def map_listeners_to_fns():
    output = []
    for (listener, ty) in LISTENERS.items():
        output.append(f'''
    pub fn on_{listener}<F>(callback: F) -> Listener
        where F: Fn({ty}) + 'static 
    {CURLY_BRACE_OPEN}
        let listener = yew::html::on{listener.replace('_', '')}::Wrapper::new(Callback::from(callback));
        Rc::new(listener)
    {CURLY_BRACE_CLOSE}
        ''')

    return ''.join(output)


for (element_name, element_attrs) in e.items():
    attrs, default = [attr.replace('-', '_') for attr in element_attrs['attrs']], element_attrs['default']
    element_name = element_name.strip('>').strip('<')
    ident = element_name.title()

    attrs = [
        # we start with {TAB} just for formatting in the output file
        # rustfmt doesn't format items in macros
        f'''{TAB}{attr if attr not in KEYWORDS else f'r#{attr}'} => "{attr}";'''
        for attr in attrs
    ]

    elements_rs.append(f'''
build_velement!({ident}, "{element_name}", [{NEW_LINE}{f'{NEW_LINE}'.join(attrs)}{NEW_LINE}]);
    '''.strip())

    param = '' if default is None else 'value: impl Into<AttrValue>'
    call = '' if default is None else '.text(value.into())'
    functions_rs.append(f'''
    pub fn {element_name}({param}) -> {ident} {CURLY_BRACE_OPEN}
        {ident}::new()
            {call}
    {CURLY_BRACE_CLOSE}
    '''.strip())

with open('../yew-vdomer/src/elements.rs', 'w') as f:
    f.write('use yew::virtual_dom::{VNode, AttrValue, VTag};')
    f.write("use crate::{Attribute, Listener, VElement, build_velement};")
    f.write('\n'.join(elements_rs))

with open('../yew-vdomer/src/functions.rs', 'w') as f:
    f.write('use yew::virtual_dom::{AttrValue};')
    f.write('use crate::elements::*;')
    f.write('use crate::VElement;')
    f.write(''.join(functions_rs))

with open('../yew-vdomer/src/listeners.rs', 'w') as f:
    f.write("use std::rc::Rc;")
    f.write("use yew::Callback;")
    f.write("use crate::Listener;")
    f.write(''.join(map_listeners_to_fns()))
