//! Query router.

use crate::plugin::plugins;

use pgdog_plugin::{FfiQuery, Route};
use tokio::time::Instant;
use tracing::debug;

pub mod error;
pub mod parser;

pub use error::Error;

use super::Buffer;

/// Query router.
pub struct Router {
    route: Route,
}

impl Default for Router {
    fn default() -> Self {
        Self::new()
    }
}

impl Router {
    /// Create new router.
    pub fn new() -> Router {
        Self {
            route: Route::unknown(),
        }
    }

    /// Route a query to a shard.
    ///
    /// If the router can't determine the route for the query to take,
    /// previous route is preserved. This is useful in case the client
    /// doesn't supply enough information in the buffer, e.g. just issued
    /// a Describe request to a previously submitted Parse.
    pub fn query(&mut self, buffer: &Buffer) -> Result<Route, Error> {
        let query = buffer
            .query()
            .map_err(|_| Error::NoQueryInBuffer)?
            .ok_or(Error::NoQueryInBuffer)?;
        let query = FfiQuery::new(&query)?;
        let now = Instant::now();

        for plugin in plugins() {
            match plugin.route(query.query()) {
                None => continue,
                Some(route) => {
                    if route.is_unknown() {
                        continue;
                    }

                    self.route = route;

                    debug!(
                        "routing {} to shard {} [{}, {:.3}ms]",
                        if route.read() { "read" } else { "write" },
                        route.shard().unwrap_or(0),
                        plugin.name(),
                        now.elapsed().as_secs_f64() * 1000.0,
                    );

                    return Ok(route);
                }
            }
        }

        Ok(self.route)
    }

    /// Get current route.
    pub fn route(&self) -> &Route {
        &self.route
    }
}
