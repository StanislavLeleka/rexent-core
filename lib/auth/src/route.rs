use actix_web::http::Method;

pub struct Route {
    pub path: &'static str,
    pub method: Method,
}

impl Route {
    pub fn matches_path_and_method(&self, path: &str, method: &Method) -> bool {
        self.matches_path(path) && self.matches_method(method)
    }

    fn matches_path(&self, path: &str) -> bool {
        let expect_path = self.path.split('/').collect::<Vec<_>>();
        let this_path = path.split('/').collect::<Vec<_>>();

        if expect_path.len() != this_path.len() {
            return false;
        };

        let path_set = expect_path.iter().zip(this_path.iter());

        for (expect_path, this_path) in path_set {
            if Self::is_slug_path(expect_path) {
                continue;
            }

            if expect_path != this_path {
                return false;
            }
        }

        true
    }

    fn matches_method(&self, method: &Method) -> bool {
        self.method == *method
    }

    fn is_slug_path(text: &str) -> bool {
        let first = text.chars().next().unwrap_or(' ');
        let last = text.chars().last().unwrap_or(' ');
        first == '{' && last == '}'
    }
}
