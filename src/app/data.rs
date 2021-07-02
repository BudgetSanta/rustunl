use pritunl::Client;

use super::state::TableProfile;

pub fn profile_data<'a>(r: &'a Client) -> Vec<TableProfile> {
    let sys_profs = r.get_system_profiles().unwrap();
    let active_profiles = r.query_active_profiles().unwrap();
    let mut table_profs = vec![];

    for sys_prof in sys_profs {
        let active_profile = active_profiles.get(&sys_prof.id);

        let (status, server_address, client_address) = match active_profile {
            None => ("Disconnected", "-", "-"),
            Some(p) => match p.timestamp {
                0 => ("Connecting", &p.server_addr[..], "-"),
                _ => ("Connected", &p.server_addr[..], &p.client_addr[..]),
            },
        };

        table_profs.push(TableProfile {
            id: sys_prof.id.clone()[..8].into(),
            name: sys_prof.server.clone(),
            status: status.into(),
            server_address: server_address.into(),
            client_address: client_address.into(),
        })
    }

    table_profs
}
