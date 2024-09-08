use x11::xlib::{self, Window};

use crate::{style::{ColorSchemesXft}, wm};

pub type WmAtoms = WmAtoms_<xlib::Atom>; 
pub type NetAtoms = NetAtoms_<xlib::Atom>;
pub type Cursor = Cursor_<xlib::Cursor>;

pub struct NetAtoms_<T> {
    pub active_window: T,
    pub supported: T,
    pub state: T,
    pub check: T,
    pub fullscreen: T,
    pub wtype: T
}

pub struct WmAtoms_<T> {
    pub protocols: T,
    pub delete: T,
    pub state: T,
    pub take_focus: T
}

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
    pub keybindings: Vec<Keybinding>,
    pub active: Active
}

pub struct Active {
    pub workspace: usize,
    pub window: Window
}

pub struct Keybinding {}
