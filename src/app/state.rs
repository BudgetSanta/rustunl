use rustunl::pritunl::Rustunl;

use crate::app::data::profile_data;

use super::components::{tab::TabsState, table::StatefulTable};

pub struct TableProfile<'a> {
    pub id: &'a str,
    pub name: &'a str,
    pub status: &'a str,
    pub server_address: &'a str,
    pub client_address: &'a str,
}

pub struct App<'a> {
    pub title: &'a str,
    pub should_quit: bool,
    pub tabs: TabsState<'a>,
    pub profiles: StatefulTable<TableProfile<'a>>,
    pub enhanced_graphics: bool,
}

impl<'a> App<'a> {
    pub fn new(r: &'a Rustunl, title: &'a str, enhanced_graphics: bool) -> App<'a> {
        App {
            title,
            should_quit: false,
            tabs: TabsState::new(vec!["Profiles"]),
            profiles: StatefulTable::with_items(profile_data(r)),
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

    pub fn on_tick(&mut self) {}
}
