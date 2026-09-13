# Iocraft Router
Router for Iocraft.
## Example Setup
```rs
use iocraft_router::{Router, RouterProvider, Route};
use iocraft::prelude::*;
#[component]
pub fn App() -> impl Into<AnyElement<'static>> {
    element! {
        View {
            RouterProvider(default: "my_component", vec![
                Route("my_component", || element! {MyComponent}.into())
            ])
        }
    }
}
#[component]
pub fn MyComponent(mut hooks: Hooks) -> impl Into<AnyElement<'static>> {
    let mut router = hooks.use_context::<Router>().clone();
    element! {
        Fragment {
            View {
                Text(content: "hello world")
            }
        }
    }
}
```
## Example Navigate and Send Data to Another Component
```rs
pub struct Data {
    pub name: String
}
#[component]
pub fn MyComponent ... {
  ...
    router.navigate("another_component", Arc::new(Data { name: String::from("john")}));
  ...
}
#[component]
pub fn AnotherComponent .. {
    ...
    let data = router.get_state::<Data>();
    ...
}
```
More example see [t-auth](https://github.com/pebrianz/t-auth).
