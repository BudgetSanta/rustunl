use crate::app::Profile;

pub fn profile_data() -> Vec<Profile<'static>> {
    vec![
        Profile {
            id: "ba8c458127de4288ab427576e732cc18",
            name: "hip-kingfish-pleasing-flea",
            status: "connected",
            server_address: "123.456.789.0",
            client_address: "987.654.321.0",
        },
        Profile {
            id: "0900f5387287432a9248d1102d7e6ae7",
            name: "tops-heron-valued-yeti",
            status: "disconnected",
            server_address: "-",
            client_address: "-",
        },
    ]
}
