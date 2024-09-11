use std::sync::Arc;

use x11::xlib::{self, Window};

use crate::{config::CustomData, style::ColorSchemesXft, wm};

pub type Cursor = Cursor_<xlib::Cursor>;

pub struct Cursor_<T> {
    pub normal: T,
    pub resize: T,
    pub mov: T
}

pub struct State<'a> {
    pub screen: i32,
    pub root: xlib::Window,
    pub cursor: Cursor,
    pub dpy: &'a mut xlib::Display,
    pub workspaces: Vec<wm::Space<'a>>,
    pub colors : ColorSchemesXft,
    pub active: Active,
}

pub struct Active {
    pub workspace: usize,
    pub window: Window,
}

pub static mut KEYBINDINGS : Vec<Keybinding> = Vec::new();

pub static mut MOUSEMOTIONS: MousemotionsType = MousemotionsType{
    on_press: Vec::new(),
    on_release: Vec::new(),
    on_move: Vec::new(), 
    on_window_enter: Vec::new()
};
pub struct MousemotionsType {
    pub on_press: Vec<Mousemotion>,
    pub on_release: Vec<Mousemotion>,
    pub on_move: Vec<Mousemotion>,
    pub on_window_enter: Vec<Mousemotion>
}


pub struct Keybinding {
    pub mdky: u32,
    pub key: u32,
    pub callback: Arc<dyn Fn(&mut State) + Send + Sync>
}

pub struct Mousemotion {
    pub mdky: u32,
    pub button: u32,
    pub callback: Arc<dyn Fn(&mut State, (i32, i32)) + Send + Sync>
}
