use rustunl::pritunl::Rustunl;

use super::state::TableProfile;

pub fn profile_data<'a>(r: &'a Rustunl) -> Vec<TableProfile<'a>> {
    r.profiles
        .iter()
        .map(|p| TableProfile {
            id: &p.server_id,
            name: &p.server,
            status: "",
            server_address: "",
            client_address: "",
        })
        .collect()
}
