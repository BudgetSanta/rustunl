use super::{
    app::Rustunl,
    proxy::{self, Response},
};

impl Rustunl {
    pub fn ping(&self) -> Response {
        proxy::get(self, "/ping")
    }

    pub fn get_profile(&self) -> Response {
        proxy::get(self, "/profile")
    }

    pub fn post_profile(&self, body: &str) -> Response {
        proxy::post(self, "/profile", body)
    }

    //pub fn delete_profile(&self) -> Response {
    //    proxy::delete(self, "/profile")
    //}

    pub fn get_events(&self) -> Response {
        proxy::get(self, "/events")
    }

    //pub fn put_token(&self) -> Response {
    //    proxy::put(self, "/token")
    //}

    //pub fn delete_token(&self) -> Response {
    //    proxy::delete(self, "/token")
    //}

    pub fn stop(&self, body: &str) -> Response {
        proxy::post(self, "/stop", body)
    }

    pub fn restart(&self, body: &str) -> Response {
        proxy::post(self, "/restart", body)
    }

    pub fn get_status(&self) -> Response {
        proxy::get(self, "/status")
    }

    pub fn get_state(&self) -> Response {
        proxy::get(self, "/state")
    }

    pub fn wakeup(&self, body: &str) -> Response {
        proxy::post(self, "/wakeup", body)
    }
}
