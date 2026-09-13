//! # Iocraft Router
//!
//! Router for Iocraft.
//!
//! ## Example Setup
//!
//! ```
//! use iocraft_router::{Router, RouterProvider, Route};
//! use iocraft::prelude::*;
//!
//! #[component]
//! pub fn App() -> impl Into<AnyElement<'static>> {
//!     element! {
//!         View {
//!             RouterProvider(default: "my_component", vec![
//!                 Route("my_component", || element! {MyComponent}.into())
//!             ])
//!         }
//!     }
//! }
//!
//! #[component]
//! pub fn MyComponent(mut hooks: Hooks) -> impl Into<AnyElement<'static>> {
//!     let mut router = hooks.use_context::<Router>().clone();
//!
//!     element! {
//!         Fragment {
//!             View {
//!                 Text(content: "hello world")
//!             }
//!         }
//!     }
//! }
//!
//! ```
//!
//! ## Example Navigate and Send Data to Another Component
//! ```
//! pub struct Data {
//!     pub name: String
//! }
//! #[component]
//! pub fn MyComponent ... {
//!   ...
//!     router.navigate("another_component", Arc::new(Data { name: String::from("john")}));
//!   ...
//! }
//!
//! #[component]
//! pub fn AnotherComponent .. {
//!     ...
//!     let data = router.get_state::<Data>();
//!     ...
//! }
//! ```
//!
//! More example see [t-auth](https://github.com/pebrianz/t-auth).

extern crate alloc;

use alloc::{string::String, sync::Arc, vec::Vec};
use core::any::Any;
use hashbrown::HashMap;
use iocraft::prelude::*;

pub struct Route<'a>(pub &'a str, pub fn() -> AnyElement<'static>);

#[derive(Clone)]
pub struct HistoryState(pub String, Option<Arc<dyn Any + Sync + Send>>);

impl HistoryState {
    #[allow(dead_code)]
    pub fn get<T: Any + Sync + Send>(&self) -> Arc<T> {
        self.1.clone().unwrap().downcast::<T>().unwrap()
    }

    #[allow(dead_code)]
    pub fn try_get<T: Any + Sync + Send>(&self) -> Option<Arc<T>> {
        self.1.clone().unwrap().downcast::<T>().ok()
    }
}

#[derive(Clone)]
pub struct RouterHistory(State<Vec<HistoryState>>);

impl RouterHistory {
    #[allow(dead_code)]
    pub fn last(&self) -> Option<HistoryState> {
        self.0.read().last().cloned()
    }
    #[allow(dead_code)]
    pub fn len(&self) -> usize {
        self.0.read().len()
    }
    #[allow(dead_code)]
    pub fn push(&mut self, state: HistoryState) {
        self.0.write().push(state);
    }
    #[allow(dead_code)]
    pub fn pop(&mut self) -> Option<HistoryState> {
        self.0.write().pop()
    }
}

#[derive(Clone)]
pub struct Router {
    routes: HashMap<String, fn() -> AnyElement<'static>>,
    pub history: RouterHistory,
}

impl Router {
    pub fn new(history: RouterHistory, routes: &mut [Route]) -> Self {
        let mut routes_map: HashMap<String, fn() -> AnyElement<'static>> = HashMap::new();

        for route in routes {
            routes_map.insert(route.0.to_string(), route.1);
        }

        Self {
            routes: routes_map,
            history,
        }
    }

    #[allow(dead_code)]
    pub fn go_back(&mut self) {
        if self.history.len() > 1 {
            self.history.pop();
        }
    }

    #[allow(dead_code)]
    pub fn go_to<'a>(&mut self, name: &'a str) {
        self.history.push(HistoryState(name.to_string(), None));
    }

    #[allow(dead_code)]
    pub fn navigate<'a>(&mut self, name: &'a str, state: Arc<dyn Any + Send + Sync + 'static>) {
        self.history
            .push(HistoryState(name.to_string(), Some(state)));
    }

    #[allow(dead_code)]
    pub fn state(&self) -> Option<HistoryState> {
        self.history.last()
    }

    #[allow(dead_code)]
    pub fn get_state<T: Any + Send + Sync>(&self) -> Arc<T> {
        self.state().unwrap().get::<T>()
    }

    #[allow(dead_code)]
    pub fn try_get_state<T: Any + Send + Sync>(&self) -> Option<Arc<T>> {
        self.state().unwrap().try_get::<T>()
    }
}

impl Hook for Router {}

pub trait RouterHook {
    fn use_router(&mut self, initial: String, routes: &mut [Route]) -> &mut Router;
}

impl RouterHook for Hooks<'_, '_> {
    fn use_router(&mut self, initial: String, routes: &mut [Route]) -> &mut Router {
        let state = self.use_state(|| vec![HistoryState(initial, None)]);
        self.use_hook(move || Router::new(RouterHistory(state), routes))
    }
}

#[derive(Default, Props)]
pub struct RouterProviderProps<'a> {
    pub routes: Vec<Route<'a>>,
    pub default: &'a str,
}

#[component]
pub fn RouterProvider<'a>(
    props: &mut RouterProviderProps<'a>,
    mut hooks: Hooks,
) -> impl Into<AnyElement<'static>> {
    let default = hooks.use_const(|| {
        let default = props.default.to_string();

        if default.is_empty() {
            return props
                .routes
                .first()
                .expect("cannot find any route")
                .0
                .to_string();
        }

        default
    });

    let router = hooks.use_router(default, &mut props.routes);
    let state_name = router.state().unwrap().0;

    let element = router
        .routes
        .get(&state_name)
        .unwrap_or_else(|| panic!("route '{state_name}' not found"));

    element! {
        ContextProvider(value: Context::Owned(Box::new(router.clone()))) {
            #(element())
        }
    }
}
