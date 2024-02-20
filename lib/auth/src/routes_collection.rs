use actix_web::http::Method;

use crate::route::Route;

pub struct RoutesCollection {
    routes: Vec<Route>,
}

impl RoutesCollection {
    pub fn new() -> Self {
        RoutesCollection { routes: Vec::new() }
    }

    pub fn add_route(&mut self, path: &'static str, method: Method) {
        self.routes.push(Route { path, method });
    }

    pub fn get_routes(&self) -> &Vec<Route> {
        &self.routes
    }
}
