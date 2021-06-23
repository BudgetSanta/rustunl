pub mod event;

mod components;

use termion::event::Key;

use components::{tab::TabsState, table::StatefulTable};
use event::Event;

use crate::data::profile_data;

pub fn action_event(app: &mut App, ev: Event<Key>) {
    match ev {
        Event::Input(key) => match key {
            Key::Char(c) => {
                app.on_key(c);
            }
            Key::Up => {
                app.on_up();
            }
            Key::Down => {
                app.on_down();
            }
            Key::Left => {
                app.on_left();
            }
            Key::Right => {
                app.on_right();
            }
            Key::Ctrl(c) => {
                app.on_ctrl_key(c);
            }
            Key::Esc => {
                app.on_esc();
            }
            _ => {}
        },
        Event::Tick => {
            app.on_tick();
        }
    }
}

pub struct Profile<'a> {
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
    pub profiles: StatefulTable<Profile<'a>>,
    pub enhanced_graphics: bool,
}

impl<'a> App<'a> {
    pub fn new(title: &'a str, enhanced_graphics: bool) -> App<'a> {
        App {
            title,
            should_quit: false,
            tabs: TabsState::new(vec!["Profiles"]),
            profiles: StatefulTable::with_items(profile_data()),
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
