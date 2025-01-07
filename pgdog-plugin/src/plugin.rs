use super::Query;
use crate::bindings::{self, Route};
use libloading::{library_filename, Library, Symbol};

/// Plugin interface.
#[derive(Debug)]
pub struct Plugin<'a> {
    name: String,
    /// Initialization routine.
    init: Option<Symbol<'a, unsafe extern "C" fn()>>,
    /// Route query to a shard.
    route: Option<Symbol<'a, unsafe extern "C" fn(bindings::Query) -> Route>>,
}

impl<'a> Plugin<'a> {
    /// Load library using a cross-platform naming convention.
    pub fn library(name: &str) -> Result<Library, libloading::Error> {
        let name = library_filename(name);
        unsafe { Library::new(name) }
    }

    /// Load standard methods from the plugin library.
    pub fn load(name: &str, library: &'a Library) -> Self {
        let route = if let Ok(route) = unsafe { library.get(b"pgdog_route_query\0") } {
            Some(route)
        } else {
            None
        };

        let init = if let Ok(init) = unsafe { library.get(b"pgdog_init\0") } {
            Some(init)
        } else {
            None
        };

        Self {
            name: name.to_owned(),
            route,
            init,
        }
    }

    /// Route query.
    pub fn route(&self, query: Query) -> Option<Route> {
        self.route.as_ref().map(|route| unsafe { route(query.into()) })
    }

    /// Perform initialization.
    pub fn init(&self) -> bool {
        if let Some(init) = &self.init {
            unsafe {
                init();
            }
            true
        } else {
            false
        }
    }

    /// Plugin name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Check that we have the required methods.
    pub fn valid(&self) -> bool {
        self.route.is_some()
    }
}
