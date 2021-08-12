use crate::Listener;
use std::rc::Rc;
use yew::Callback;
pub fn on_abort<F>(callback: F) -> Listener
where
    F: Fn(web_sys::Event) + 'static,
{
    let listener = yew::html::onabort::Wrapper::new(Callback::from(callback));
    Rc::new(listener)
}

pub fn on_auxclick<F>(callback: F) -> Listener
where
    F: Fn(yew::MouseEvent) + 'static,
{
    let listener = yew::html::onauxclick::Wrapper::new(Callback::from(callback));
    Rc::new(listener)
}

pub fn on_blur<F>(callback: F) -> Listener
where
    F: Fn(web_sys::FocusEvent) + 'static,
{
    let listener = yew::html::onblur::Wrapper::new(Callback::from(callback));
    Rc::new(listener)
}

pub fn on_cancel<F>(callback: F) -> Listener
where
    F: Fn(web_sys::Event) + 'static,
{
    let listener = yew::html::oncancel::Wrapper::new(Callback::from(callback));
    Rc::new(listener)
}

pub fn on_canplay<F>(callback: F) -> Listener
where
    F: Fn(web_sys::Event) + 'static,
{
    let listener = yew::html::oncanplay::Wrapper::new(Callback::from(callback));
    Rc::new(listener)
}

pub fn on_canplaythrough<F>(callback: F) -> Listener
where
    F: Fn(web_sys::Event) + 'static,
{
    let listener = yew::html::oncanplaythrough::Wrapper::new(Callback::from(callback));
    Rc::new(listener)
}

pub fn on_change<F>(callback: F) -> Listener
where
    F: Fn(yew::ChangeData) + 'static,
{
    let listener = yew::html::onchange::Wrapper::new(Callback::from(callback));
    Rc::new(listener)
}

pub fn on_click<F>(callback: F) -> Listener
where
    F: Fn(yew::MouseEvent) + 'static,
{
    let listener = yew::html::onclick::Wrapper::new(Callback::from(callback));
    Rc::new(listener)
}

pub fn on_close<F>(callback: F) -> Listener
where
    F: Fn(web_sys::Event) + 'static,
{
    let listener = yew::html::onclose::Wrapper::new(Callback::from(callback));
    Rc::new(listener)
}

pub fn on_contextmenu<F>(callback: F) -> Listener
where
    F: Fn(yew::MouseEvent) + 'static,
{
    let listener = yew::html::oncontextmenu::Wrapper::new(Callback::from(callback));
    Rc::new(listener)
}

pub fn on_cuechange<F>(callback: F) -> Listener
where
    F: Fn(web_sys::Event) + 'static,
{
    let listener = yew::html::oncuechange::Wrapper::new(Callback::from(callback));
    Rc::new(listener)
}

pub fn on_dblclick<F>(callback: F) -> Listener
where
    F: Fn(yew::MouseEvent) + 'static,
{
    let listener = yew::html::ondblclick::Wrapper::new(Callback::from(callback));
    Rc::new(listener)
}

pub fn on_drag<F>(callback: F) -> Listener
where
    F: Fn(web_sys::DragEvent) + 'static,
{
    let listener = yew::html::ondrag::Wrapper::new(Callback::from(callback));
    Rc::new(listener)
}

pub fn on_dragend<F>(callback: F) -> Listener
where
    F: Fn(web_sys::DragEvent) + 'static,
{
    let listener = yew::html::ondragend::Wrapper::new(Callback::from(callback));
    Rc::new(listener)
}

pub fn on_dragenter<F>(callback: F) -> Listener
where
    F: Fn(web_sys::DragEvent) + 'static,
{
    let listener = yew::html::ondragenter::Wrapper::new(Callback::from(callback));
    Rc::new(listener)
}

pub fn on_dragexit<F>(callback: F) -> Listener
where
    F: Fn(web_sys::DragEvent) + 'static,
{
    let listener = yew::html::ondragexit::Wrapper::new(Callback::from(callback));
    Rc::new(listener)
}

pub fn on_dragleave<F>(callback: F) -> Listener
where
    F: Fn(web_sys::DragEvent) + 'static,
{
    let listener = yew::html::ondragleave::Wrapper::new(Callback::from(callback));
    Rc::new(listener)
}

pub fn on_dragover<F>(callback: F) -> Listener
where
    F: Fn(web_sys::DragEvent) + 'static,
{
    let listener = yew::html::ondragover::Wrapper::new(Callback::from(callback));
    Rc::new(listener)
}

pub fn on_dragstart<F>(callback: F) -> Listener
where
    F: Fn(web_sys::DragEvent) + 'static,
{
    let listener = yew::html::ondragstart::Wrapper::new(Callback::from(callback));
    Rc::new(listener)
}

pub fn on_drop<F>(callback: F) -> Listener
where
    F: Fn(web_sys::DragEvent) + 'static,
{
    let listener = yew::html::ondrop::Wrapper::new(Callback::from(callback));
    Rc::new(listener)
}

pub fn on_durationchange<F>(callback: F) -> Listener
where
    F: Fn(web_sys::Event) + 'static,
{
    let listener = yew::html::ondurationchange::Wrapper::new(Callback::from(callback));
    Rc::new(listener)
}

pub fn on_emptied<F>(callback: F) -> Listener
where
    F: Fn(web_sys::Event) + 'static,
{
    let listener = yew::html::onemptied::Wrapper::new(Callback::from(callback));
    Rc::new(listener)
}

pub fn on_ended<F>(callback: F) -> Listener
where
    F: Fn(web_sys::Event) + 'static,
{
    let listener = yew::html::onended::Wrapper::new(Callback::from(callback));
    Rc::new(listener)
}

pub fn on_error<F>(callback: F) -> Listener
where
    F: Fn(web_sys::Event) + 'static,
{
    let listener = yew::html::onerror::Wrapper::new(Callback::from(callback));
    Rc::new(listener)
}

pub fn on_focus<F>(callback: F) -> Listener
where
    F: Fn(web_sys::FocusEvent) + 'static,
{
    let listener = yew::html::onfocus::Wrapper::new(Callback::from(callback));
    Rc::new(listener)
}

pub fn on_formdata<F>(callback: F) -> Listener
where
    F: Fn(web_sys::Event) + 'static,
{
    let listener = yew::html::onformdata::Wrapper::new(Callback::from(callback));
    Rc::new(listener)
}

pub fn on_input<F>(callback: F) -> Listener
where
    F: Fn(yew::InputData) + 'static,
{
    let listener = yew::html::oninput::Wrapper::new(Callback::from(callback));
    Rc::new(listener)
}

pub fn on_invalid<F>(callback: F) -> Listener
where
    F: Fn(web_sys::Event) + 'static,
{
    let listener = yew::html::oninvalid::Wrapper::new(Callback::from(callback));
    Rc::new(listener)
}

pub fn on_keydown<F>(callback: F) -> Listener
where
    F: Fn(web_sys::KeyboardEvent) + 'static,
{
    let listener = yew::html::onkeydown::Wrapper::new(Callback::from(callback));
    Rc::new(listener)
}

pub fn on_keypress<F>(callback: F) -> Listener
where
    F: Fn(web_sys::KeyboardEvent) + 'static,
{
    let listener = yew::html::onkeypress::Wrapper::new(Callback::from(callback));
    Rc::new(listener)
}

pub fn on_keyup<F>(callback: F) -> Listener
where
    F: Fn(web_sys::KeyboardEvent) + 'static,
{
    let listener = yew::html::onkeyup::Wrapper::new(Callback::from(callback));
    Rc::new(listener)
}

pub fn on_load<F>(callback: F) -> Listener
where
    F: Fn(web_sys::Event) + 'static,
{
    let listener = yew::html::onload::Wrapper::new(Callback::from(callback));
    Rc::new(listener)
}

pub fn on_loadeddata<F>(callback: F) -> Listener
where
    F: Fn(web_sys::Event) + 'static,
{
    let listener = yew::html::onloadeddata::Wrapper::new(Callback::from(callback));
    Rc::new(listener)
}

pub fn on_loadedmetadata<F>(callback: F) -> Listener
where
    F: Fn(web_sys::Event) + 'static,
{
    let listener = yew::html::onloadedmetadata::Wrapper::new(Callback::from(callback));
    Rc::new(listener)
}

pub fn on_loadstart<F>(callback: F) -> Listener
where
    F: Fn(web_sys::ProgressEvent) + 'static,
{
    let listener = yew::html::onloadstart::Wrapper::new(Callback::from(callback));
    Rc::new(listener)
}

pub fn on_mousedown<F>(callback: F) -> Listener
where
    F: Fn(yew::MouseEvent) + 'static,
{
    let listener = yew::html::onmousedown::Wrapper::new(Callback::from(callback));
    Rc::new(listener)
}

pub fn on_mouseenter<F>(callback: F) -> Listener
where
    F: Fn(yew::MouseEvent) + 'static,
{
    let listener = yew::html::onmouseenter::Wrapper::new(Callback::from(callback));
    Rc::new(listener)
}

pub fn on_mouseleave<F>(callback: F) -> Listener
where
    F: Fn(yew::MouseEvent) + 'static,
{
    let listener = yew::html::onmouseleave::Wrapper::new(Callback::from(callback));
    Rc::new(listener)
}

pub fn on_mousemove<F>(callback: F) -> Listener
where
    F: Fn(yew::MouseEvent) + 'static,
{
    let listener = yew::html::onmousemove::Wrapper::new(Callback::from(callback));
    Rc::new(listener)
}

pub fn on_mouseout<F>(callback: F) -> Listener
where
    F: Fn(yew::MouseEvent) + 'static,
{
    let listener = yew::html::onmouseout::Wrapper::new(Callback::from(callback));
    Rc::new(listener)
}

pub fn on_mouseover<F>(callback: F) -> Listener
where
    F: Fn(yew::MouseEvent) + 'static,
{
    let listener = yew::html::onmouseover::Wrapper::new(Callback::from(callback));
    Rc::new(listener)
}

pub fn on_mouseup<F>(callback: F) -> Listener
where
    F: Fn(yew::MouseEvent) + 'static,
{
    let listener = yew::html::onmouseup::Wrapper::new(Callback::from(callback));
    Rc::new(listener)
}

pub fn on_pause<F>(callback: F) -> Listener
where
    F: Fn(web_sys::Event) + 'static,
{
    let listener = yew::html::onpause::Wrapper::new(Callback::from(callback));
    Rc::new(listener)
}

pub fn on_play<F>(callback: F) -> Listener
where
    F: Fn(web_sys::Event) + 'static,
{
    let listener = yew::html::onplay::Wrapper::new(Callback::from(callback));
    Rc::new(listener)
}

pub fn on_playing<F>(callback: F) -> Listener
where
    F: Fn(web_sys::Event) + 'static,
{
    let listener = yew::html::onplaying::Wrapper::new(Callback::from(callback));
    Rc::new(listener)
}

pub fn on_progress<F>(callback: F) -> Listener
where
    F: Fn(web_sys::ProgressEvent) + 'static,
{
    let listener = yew::html::onprogress::Wrapper::new(Callback::from(callback));
    Rc::new(listener)
}

pub fn on_ratechange<F>(callback: F) -> Listener
where
    F: Fn(web_sys::Event) + 'static,
{
    let listener = yew::html::onratechange::Wrapper::new(Callback::from(callback));
    Rc::new(listener)
}

pub fn on_reset<F>(callback: F) -> Listener
where
    F: Fn(web_sys::Event) + 'static,
{
    let listener = yew::html::onreset::Wrapper::new(Callback::from(callback));
    Rc::new(listener)
}

pub fn on_resize<F>(callback: F) -> Listener
where
    F: Fn(web_sys::Event) + 'static,
{
    let listener = yew::html::onresize::Wrapper::new(Callback::from(callback));
    Rc::new(listener)
}

pub fn on_scroll<F>(callback: F) -> Listener
where
    F: Fn(web_sys::Event) + 'static,
{
    let listener = yew::html::onscroll::Wrapper::new(Callback::from(callback));
    Rc::new(listener)
}

pub fn on_securitypolicyviolation<F>(callback: F) -> Listener
where
    F: Fn(web_sys::Event) + 'static,
{
    let listener = yew::html::onsecuritypolicyviolation::Wrapper::new(Callback::from(callback));
    Rc::new(listener)
}

pub fn on_seeked<F>(callback: F) -> Listener
where
    F: Fn(web_sys::Event) + 'static,
{
    let listener = yew::html::onseeked::Wrapper::new(Callback::from(callback));
    Rc::new(listener)
}

pub fn on_seeking<F>(callback: F) -> Listener
where
    F: Fn(web_sys::Event) + 'static,
{
    let listener = yew::html::onseeking::Wrapper::new(Callback::from(callback));
    Rc::new(listener)
}

pub fn on_select<F>(callback: F) -> Listener
where
    F: Fn(web_sys::Event) + 'static,
{
    let listener = yew::html::onselect::Wrapper::new(Callback::from(callback));
    Rc::new(listener)
}

pub fn on_slotchange<F>(callback: F) -> Listener
where
    F: Fn(web_sys::Event) + 'static,
{
    let listener = yew::html::onslotchange::Wrapper::new(Callback::from(callback));
    Rc::new(listener)
}

pub fn on_stalled<F>(callback: F) -> Listener
where
    F: Fn(web_sys::Event) + 'static,
{
    let listener = yew::html::onstalled::Wrapper::new(Callback::from(callback));
    Rc::new(listener)
}

pub fn on_submit<F>(callback: F) -> Listener
where
    F: Fn(web_sys::FocusEvent) + 'static,
{
    let listener = yew::html::onsubmit::Wrapper::new(Callback::from(callback));
    Rc::new(listener)
}

pub fn on_suspend<F>(callback: F) -> Listener
where
    F: Fn(web_sys::Event) + 'static,
{
    let listener = yew::html::onsuspend::Wrapper::new(Callback::from(callback));
    Rc::new(listener)
}

pub fn on_timeupdate<F>(callback: F) -> Listener
where
    F: Fn(web_sys::Event) + 'static,
{
    let listener = yew::html::ontimeupdate::Wrapper::new(Callback::from(callback));
    Rc::new(listener)
}

pub fn on_toggle<F>(callback: F) -> Listener
where
    F: Fn(web_sys::Event) + 'static,
{
    let listener = yew::html::ontoggle::Wrapper::new(Callback::from(callback));
    Rc::new(listener)
}

pub fn on_volumechange<F>(callback: F) -> Listener
where
    F: Fn(web_sys::Event) + 'static,
{
    let listener = yew::html::onvolumechange::Wrapper::new(Callback::from(callback));
    Rc::new(listener)
}

pub fn on_waiting<F>(callback: F) -> Listener
where
    F: Fn(web_sys::Event) + 'static,
{
    let listener = yew::html::onwaiting::Wrapper::new(Callback::from(callback));
    Rc::new(listener)
}

pub fn on_wheel<F>(callback: F) -> Listener
where
    F: Fn(web_sys::WheelEvent) + 'static,
{
    let listener = yew::html::onwheel::Wrapper::new(Callback::from(callback));
    Rc::new(listener)
}

pub fn on_copy<F>(callback: F) -> Listener
where
    F: Fn(web_sys::Event) + 'static,
{
    let listener = yew::html::oncopy::Wrapper::new(Callback::from(callback));
    Rc::new(listener)
}

pub fn on_cut<F>(callback: F) -> Listener
where
    F: Fn(web_sys::Event) + 'static,
{
    let listener = yew::html::oncut::Wrapper::new(Callback::from(callback));
    Rc::new(listener)
}

pub fn on_paste<F>(callback: F) -> Listener
where
    F: Fn(web_sys::Event) + 'static,
{
    let listener = yew::html::onpaste::Wrapper::new(Callback::from(callback));
    Rc::new(listener)
}

pub fn on_animationcancel<F>(callback: F) -> Listener
where
    F: Fn(web_sys::AnimationEvent) + 'static,
{
    let listener = yew::html::onanimationcancel::Wrapper::new(Callback::from(callback));
    Rc::new(listener)
}

pub fn on_animationend<F>(callback: F) -> Listener
where
    F: Fn(web_sys::AnimationEvent) + 'static,
{
    let listener = yew::html::onanimationend::Wrapper::new(Callback::from(callback));
    Rc::new(listener)
}

pub fn on_animationiteration<F>(callback: F) -> Listener
where
    F: Fn(web_sys::AnimationEvent) + 'static,
{
    let listener = yew::html::onanimationiteration::Wrapper::new(Callback::from(callback));
    Rc::new(listener)
}

pub fn on_animationstart<F>(callback: F) -> Listener
where
    F: Fn(web_sys::AnimationEvent) + 'static,
{
    let listener = yew::html::onanimationstart::Wrapper::new(Callback::from(callback));
    Rc::new(listener)
}

pub fn on_gotpointercapture<F>(callback: F) -> Listener
where
    F: Fn(web_sys::PointerEvent) + 'static,
{
    let listener = yew::html::ongotpointercapture::Wrapper::new(Callback::from(callback));
    Rc::new(listener)
}

pub fn on_loadend<F>(callback: F) -> Listener
where
    F: Fn(web_sys::ProgressEvent) + 'static,
{
    let listener = yew::html::onloadend::Wrapper::new(Callback::from(callback));
    Rc::new(listener)
}

pub fn on_lostpointercapture<F>(callback: F) -> Listener
where
    F: Fn(web_sys::PointerEvent) + 'static,
{
    let listener = yew::html::onlostpointercapture::Wrapper::new(Callback::from(callback));
    Rc::new(listener)
}

pub fn on_pointercancel<F>(callback: F) -> Listener
where
    F: Fn(web_sys::PointerEvent) + 'static,
{
    let listener = yew::html::onpointercancel::Wrapper::new(Callback::from(callback));
    Rc::new(listener)
}

pub fn on_pointerdown<F>(callback: F) -> Listener
where
    F: Fn(web_sys::PointerEvent) + 'static,
{
    let listener = yew::html::onpointerdown::Wrapper::new(Callback::from(callback));
    Rc::new(listener)
}

pub fn on_pointerenter<F>(callback: F) -> Listener
where
    F: Fn(web_sys::PointerEvent) + 'static,
{
    let listener = yew::html::onpointerenter::Wrapper::new(Callback::from(callback));
    Rc::new(listener)
}

pub fn on_pointerleave<F>(callback: F) -> Listener
where
    F: Fn(web_sys::PointerEvent) + 'static,
{
    let listener = yew::html::onpointerleave::Wrapper::new(Callback::from(callback));
    Rc::new(listener)
}

pub fn on_pointerlockchange<F>(callback: F) -> Listener
where
    F: Fn(web_sys::Event) + 'static,
{
    let listener = yew::html::onpointerlockchange::Wrapper::new(Callback::from(callback));
    Rc::new(listener)
}

pub fn on_pointerlockerror<F>(callback: F) -> Listener
where
    F: Fn(web_sys::Event) + 'static,
{
    let listener = yew::html::onpointerlockerror::Wrapper::new(Callback::from(callback));
    Rc::new(listener)
}

pub fn on_pointermove<F>(callback: F) -> Listener
where
    F: Fn(web_sys::PointerEvent) + 'static,
{
    let listener = yew::html::onpointermove::Wrapper::new(Callback::from(callback));
    Rc::new(listener)
}

pub fn on_pointerout<F>(callback: F) -> Listener
where
    F: Fn(web_sys::PointerEvent) + 'static,
{
    let listener = yew::html::onpointerout::Wrapper::new(Callback::from(callback));
    Rc::new(listener)
}

pub fn on_pointerover<F>(callback: F) -> Listener
where
    F: Fn(web_sys::PointerEvent) + 'static,
{
    let listener = yew::html::onpointerover::Wrapper::new(Callback::from(callback));
    Rc::new(listener)
}

pub fn on_pointerup<F>(callback: F) -> Listener
where
    F: Fn(web_sys::PointerEvent) + 'static,
{
    let listener = yew::html::onpointerup::Wrapper::new(Callback::from(callback));
    Rc::new(listener)
}

pub fn on_selectionchange<F>(callback: F) -> Listener
where
    F: Fn(web_sys::Event) + 'static,
{
    let listener = yew::html::onselectionchange::Wrapper::new(Callback::from(callback));
    Rc::new(listener)
}

pub fn on_selectstart<F>(callback: F) -> Listener
where
    F: Fn(web_sys::Event) + 'static,
{
    let listener = yew::html::onselectstart::Wrapper::new(Callback::from(callback));
    Rc::new(listener)
}

pub fn on_show<F>(callback: F) -> Listener
where
    F: Fn(web_sys::Event) + 'static,
{
    let listener = yew::html::onshow::Wrapper::new(Callback::from(callback));
    Rc::new(listener)
}

pub fn on_touchcancel<F>(callback: F) -> Listener
where
    F: Fn(web_sys::TouchEvent) + 'static,
{
    let listener = yew::html::ontouchcancel::Wrapper::new(Callback::from(callback));
    Rc::new(listener)
}

pub fn on_touchend<F>(callback: F) -> Listener
where
    F: Fn(web_sys::TouchEvent) + 'static,
{
    let listener = yew::html::ontouchend::Wrapper::new(Callback::from(callback));
    Rc::new(listener)
}

pub fn on_touchmove<F>(callback: F) -> Listener
where
    F: Fn(web_sys::TouchEvent) + 'static,
{
    let listener = yew::html::ontouchmove::Wrapper::new(Callback::from(callback));
    Rc::new(listener)
}

pub fn on_touchstart<F>(callback: F) -> Listener
where
    F: Fn(web_sys::TouchEvent) + 'static,
{
    let listener = yew::html::ontouchstart::Wrapper::new(Callback::from(callback));
    Rc::new(listener)
}

pub fn on_transitioncancel<F>(callback: F) -> Listener
where
    F: Fn(web_sys::TransitionEvent) + 'static,
{
    let listener = yew::html::ontransitioncancel::Wrapper::new(Callback::from(callback));
    Rc::new(listener)
}

pub fn on_transitionend<F>(callback: F) -> Listener
where
    F: Fn(web_sys::TransitionEvent) + 'static,
{
    let listener = yew::html::ontransitionend::Wrapper::new(Callback::from(callback));
    Rc::new(listener)
}

pub fn on_transitionrun<F>(callback: F) -> Listener
where
    F: Fn(web_sys::TransitionEvent) + 'static,
{
    let listener = yew::html::ontransitionrun::Wrapper::new(Callback::from(callback));
    Rc::new(listener)
}

pub fn on_transitionstart<F>(callback: F) -> Listener
where
    F: Fn(web_sys::TransitionEvent) + 'static,
{
    let listener = yew::html::ontransitionstart::Wrapper::new(Callback::from(callback));
    Rc::new(listener)
}
