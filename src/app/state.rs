use pritunl::Client;

use super::components::{tab::TabsState, table::StatefulTable};
use crate::app::data::profile_data;

pub struct TableProfile {
    pub id: String,
    pub name: String,
    pub status: String,
    pub server_address: String,
    pub client_address: String,
}

pub struct App<'a> {
    pub title: &'a str,
    pub client: Client,
    pub should_quit: bool,
    pub tabs: TabsState<'a>,
    pub profiles: StatefulTable<TableProfile>,
    pub enhanced_graphics: bool,
}

impl<'a> App<'a> {
    pub fn new(title: &'a str, enhanced_graphics: bool) -> App<'a> {
        App {
            title,
            client: Client::new(),
            should_quit: false,
            tabs: TabsState::new(vec!["Profiles"]),
            profiles: StatefulTable::with_items(vec![]),
            enhanced_graphics,
        }
    }

    pub fn on_up(&mut self) {
        self.profiles.previous();
    }

    pub fn on_down(&mut self) {
        self.profiles.next();
    }

    pub fn on_right(&mut self) {
        self.tabs.next();
    }

    pub fn on_left(&mut self) {
        self.tabs.previous();
    }

    pub fn on_key(&mut self, c: char) {
        match c {
            '1' => self.tabs.index = 0,
            _ => {}
        }
    }

    pub fn on_ctrl_key(&mut self, c: char) {
        match c {
            'c' | 'd' => {
                self.should_quit = true;
            }
            _ => {}
        }
    }

    pub fn on_esc(&mut self) {
        self.profiles.unselect();
    }

    pub fn on_tick(&mut self) {
        self.profiles.set_items(profile_data(&self.client));
    }
}
