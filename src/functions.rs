use crate::elements::*;
use yew::virtual_dom::AttrValue;
pub fn html() -> Html {
    Html::new()
}
pub fn base() -> Base {
    Base::new()
}
pub fn head() -> Head {
    Head::new()
}
pub fn link() -> Link {
    Link::new()
}
pub fn meta() -> Meta {
    Meta::new()
}
pub fn style(value: impl Into<AttrValue>) -> Style {
    Style::new().text(value.into())
}
pub fn title(value: impl Into<AttrValue>) -> Title {
    Title::new().text(value.into())
}
pub fn body() -> Body {
    Body::new()
}
pub fn address() -> Address {
    Address::new()
}
pub fn article() -> Article {
    Article::new()
}
pub fn aside() -> Aside {
    Aside::new()
}
pub fn footer() -> Footer {
    Footer::new()
}
pub fn header() -> Header {
    Header::new()
}
pub fn main() -> Main {
    Main::new()
}
pub fn nav() -> Nav {
    Nav::new()
}
pub fn section() -> Section {
    Section::new()
}
pub fn blockquote() -> Blockquote {
    Blockquote::new()
}
pub fn dd() -> Dd {
    Dd::new()
}
pub fn div() -> Div {
    Div::new()
}
pub fn dl() -> Dl {
    Dl::new()
}
pub fn dt() -> Dt {
    Dt::new()
}
pub fn figcaption() -> Figcaption {
    Figcaption::new()
}
pub fn figure() -> Figure {
    Figure::new()
}
pub fn hr() -> Hr {
    Hr::new()
}
pub fn li() -> Li {
    Li::new()
}
pub fn ol() -> Ol {
    Ol::new()
}
pub fn p(value: impl Into<AttrValue>) -> P {
    P::new().text(value.into())
}
pub fn pre() -> Pre {
    Pre::new()
}
pub fn ul() -> Ul {
    Ul::new()
}
pub fn a(value: impl Into<AttrValue>) -> A {
    A::new().text(value.into())
}
pub fn abbr(value: impl Into<AttrValue>) -> Abbr {
    Abbr::new().text(value.into())
}
pub fn b(value: impl Into<AttrValue>) -> B {
    B::new().text(value.into())
}
pub fn bdi(value: impl Into<AttrValue>) -> Bdi {
    Bdi::new().text(value.into())
}
pub fn bdo() -> Bdo {
    Bdo::new()
}
pub fn br() -> Br {
    Br::new()
}
pub fn cite() -> Cite {
    Cite::new()
}
pub fn code(value: impl Into<AttrValue>) -> Code {
    Code::new().text(value.into())
}
pub fn data() -> Data {
    Data::new()
}
pub fn dfn(value: impl Into<AttrValue>) -> Dfn {
    Dfn::new().text(value.into())
}
pub fn em(value: impl Into<AttrValue>) -> Em {
    Em::new().text(value.into())
}
pub fn i(value: impl Into<AttrValue>) -> I {
    I::new().text(value.into())
}
pub fn kbd() -> Kbd {
    Kbd::new()
}
pub fn mark() -> Mark {
    Mark::new()
}
pub fn q() -> Q {
    Q::new()
}
pub fn rp() -> Rp {
    Rp::new()
}
pub fn rt() -> Rt {
    Rt::new()
}
pub fn ruby() -> Ruby {
    Ruby::new()
}
pub fn s() -> S {
    S::new()
}
pub fn samp() -> Samp {
    Samp::new()
}
pub fn small(value: impl Into<AttrValue>) -> Small {
    Small::new().text(value.into())
}
pub fn span() -> Span {
    Span::new()
}
pub fn strong(value: impl Into<AttrValue>) -> Strong {
    Strong::new().text(value.into())
}
pub fn sub() -> Sub {
    Sub::new()
}
pub fn sup() -> Sup {
    Sup::new()
}
pub fn time() -> Time {
    Time::new()
}
pub fn u() -> U {
    U::new()
}
pub fn var() -> Var {
    Var::new()
}
pub fn wbr() -> Wbr {
    Wbr::new()
}
pub fn area() -> Area {
    Area::new()
}
pub fn audio() -> Audio {
    Audio::new()
}
pub fn img() -> Img {
    Img::new()
}
pub fn map() -> Map {
    Map::new()
}
pub fn track() -> Track {
    Track::new()
}
pub fn video() -> Video {
    Video::new()
}
pub fn embed() -> Embed {
    Embed::new()
}
pub fn iframe() -> Iframe {
    Iframe::new()
}
pub fn object() -> Object {
    Object::new()
}
pub fn param() -> Param {
    Param::new()
}
pub fn picture() -> Picture {
    Picture::new()
}
pub fn portal() -> Portal {
    Portal::new()
}
pub fn source() -> Source {
    Source::new()
}
pub fn svg() -> Svg {
    Svg::new()
}
pub fn math() -> Math {
    Math::new()
}
pub fn canvas() -> Canvas {
    Canvas::new()
}
pub fn noscript() -> Noscript {
    Noscript::new()
}
pub fn script(value: impl Into<AttrValue>) -> Script {
    Script::new().text(value.into())
}
pub fn del() -> Del {
    Del::new()
}
pub fn ins() -> Ins {
    Ins::new()
}
pub fn caption() -> Caption {
    Caption::new()
}
pub fn col() -> Col {
    Col::new()
}
pub fn colgroup() -> Colgroup {
    Colgroup::new()
}
pub fn table() -> Table {
    Table::new()
}
pub fn tbody() -> Tbody {
    Tbody::new()
}
pub fn td() -> Td {
    Td::new()
}
pub fn tfoot() -> Tfoot {
    Tfoot::new()
}
pub fn th() -> Th {
    Th::new()
}
pub fn thead() -> Thead {
    Thead::new()
}
pub fn tr() -> Tr {
    Tr::new()
}
pub fn button(value: impl Into<AttrValue>) -> Button {
    Button::new().text(value.into())
}
pub fn datalist() -> Datalist {
    Datalist::new()
}
pub fn fieldset() -> Fieldset {
    Fieldset::new()
}
pub fn form() -> Form {
    Form::new()
}
pub fn input() -> Input {
    Input::new()
}
pub fn label(value: impl Into<AttrValue>) -> Label {
    Label::new().text(value.into())
}
pub fn legend() -> Legend {
    Legend::new()
}
pub fn meter() -> Meter {
    Meter::new()
}
pub fn optgroup() -> Optgroup {
    Optgroup::new()
}
pub fn option() -> Option {
    Option::new()
}
pub fn output() -> Output {
    Output::new()
}
pub fn progress() -> Progress {
    Progress::new()
}
pub fn select() -> Select {
    Select::new()
}
pub fn textarea() -> Textarea {
    Textarea::new()
}
pub fn details() -> Details {
    Details::new()
}
pub fn dialog() -> Dialog {
    Dialog::new()
}
pub fn menu() -> Menu {
    Menu::new()
}
pub fn summary() -> Summary {
    Summary::new()
}
pub fn slot() -> Slot {
    Slot::new()
}
pub fn template() -> Template {
    Template::new()
}
pub fn acronym() -> Acronym {
    Acronym::new()
}
pub fn applet() -> Applet {
    Applet::new()
}
pub fn basefont() -> Basefont {
    Basefont::new()
}
pub fn bgsound() -> Bgsound {
    Bgsound::new()
}
pub fn big() -> Big {
    Big::new()
}
pub fn blink() -> Blink {
    Blink::new()
}
pub fn center() -> Center {
    Center::new()
}
pub fn content() -> Content {
    Content::new()
}
pub fn dir() -> Dir {
    Dir::new()
}
pub fn font() -> Font {
    Font::new()
}
pub fn frame() -> Frame {
    Frame::new()
}
pub fn frameset() -> Frameset {
    Frameset::new()
}
pub fn hgroup() -> Hgroup {
    Hgroup::new()
}
pub fn image() -> Image {
    Image::new()
}
pub fn keygen() -> Keygen {
    Keygen::new()
}
pub fn marquee() -> Marquee {
    Marquee::new()
}
pub fn menuitem() -> Menuitem {
    Menuitem::new()
}
pub fn nobr() -> Nobr {
    Nobr::new()
}
pub fn noembed() -> Noembed {
    Noembed::new()
}
pub fn noframes() -> Noframes {
    Noframes::new()
}
pub fn plaintext() -> Plaintext {
    Plaintext::new()
}
pub fn rb() -> Rb {
    Rb::new()
}
pub fn rtc() -> Rtc {
    Rtc::new()
}
pub fn shadow() -> Shadow {
    Shadow::new()
}
pub fn spacer() -> Spacer {
    Spacer::new()
}
pub fn strike() -> Strike {
    Strike::new()
}
pub fn tt() -> Tt {
    Tt::new()
}
pub fn xmp() -> Xmp {
    Xmp::new()
}
pub fn h1(value: impl Into<AttrValue>) -> H1 {
    H1::new().text(value.into())
}
pub fn h2(value: impl Into<AttrValue>) -> H2 {
    H2::new().text(value.into())
}
pub fn h3(value: impl Into<AttrValue>) -> H3 {
    H3::new().text(value.into())
}
pub fn h4(value: impl Into<AttrValue>) -> H4 {
    H4::new().text(value.into())
}
pub fn h5(value: impl Into<AttrValue>) -> H5 {
    H5::new().text(value.into())
}
pub fn h6(value: impl Into<AttrValue>) -> H6 {
    H6::new().text(value.into())
}
