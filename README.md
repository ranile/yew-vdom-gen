# Yew DSL

Construct Yew’s Virtual DOM using ergonomic Rust idioms.

The API is based around functions and makes no use of macros, like `yew::html!` which allows it to provide great 
IDE support. IntelliSense is your friend when using this library.

## Usage

```rust
use yew::prelude::*;
use yew_dsl::prelude::*;

struct Component1;
impl Component for Component1 {
    // ...
    fn view(&self) -> Html {
        h1("Heading ").into()
    }
}

struct Component2;
impl Component for Component2 {
    // ...
    fn view(&self) -> Html {
        div()
            .component::<Component2>(yew::props!(Component1::Properties {}))
            .child(h2("test2"))
            .listener(on_click(|_e| log!("test")))
            .into()
    }
}
```

## Documentation

The API docs are hosted on [docs.rs](https://docs.rs/yew-dsl). 
