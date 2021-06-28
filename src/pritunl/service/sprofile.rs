//struct Sprofile {
//    id: String,
//    name: String,
//    server: String,
//    user_id: String,
//    user: String,
//    password_mode: String,
//    password: String,
//    profile: profile,
//}

impl Rustunl {
    pub fn get_system_profile(&self) -> Response {
        proxy::get(self, "/sprofile")
    }

    //pub fn put_system_profile(&self) -> Response {
    //    proxy::put(self, "/sprofile")
    //}

    //pub fn delete_system_profile(&self) -> Response {
    //    proxy::delete(self, "/sprofile")
    //}

    pub fn get_system_profile_log(&self, id: &str) -> Response {
        proxy::get(self, &format!("/sprofile/{}/log", id))
    }

    pub fn delete_system_profile_log(&self, id: &str) -> Response {
        proxy::get(self, &format!("/sprofile/{}/log", id))
    }
}
