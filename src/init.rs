use x11::xft::{XftColor, XftColorAllocName};
use x11::xlib::{CWCursor, CWEventMask, PropModeReplace, XChangeWindowAttributes, XDefaultColormap, XDefaultGC, XDefaultVisual, XFlush, XMapWindow, XSetWindowAttributes, XWhitePixel, XA_WINDOW};
use x11::xlib::{self, False, XChangeProperty, XCreateSimpleWindow, XCreateWindow, XSync};
use x11::xrender::XRenderColor;
use std::ffi::CStr;
use std::ffi::CString;
use std::ptr::null;
use std::mem;

use crate::config::STYLE;
use crate::wm;
use crate::style::{self};
use crate::state::{Active, Cursor, NetAtoms, WmAtoms};

use super::error;
use super::state;

pub fn check_other_wms(dpy: &mut xlib::Display){
    unsafe {
        xlib::XSetErrorHandler(Some(error::xerror_start));
        xlib::XSelectInput(dpy, xlib::XDefaultRootWindow(dpy), xlib::SubstructureRedirectMask);
        XSync(dpy, False);
        xlib::XSetErrorHandler(Some(error::xerror));
        XSync(dpy, False);
    }
}

macro_rules! init_cursor {
    ($dpy:expr, $ty:expr) => {{
        unsafe {xlib::XCreateFontCursor($dpy, $ty)}
    }};
}

pub fn setup(dpy: &mut xlib::Display) -> state::State {
    let mut state: state::State;
    {
        let screen =  unsafe { xlib::XDefaultScreen(dpy) };
        let root: u64 = unsafe { xlib::XRootWindow(dpy, screen) };
    
        state = state::State {
            screen: screen,
            root: root,
            cursor: Cursor {
                normal: init_cursor!(dpy, 68 /* XC left ptr */),
                resize: init_cursor!(dpy, 120 /* XC sizing */),
                mov: init_cursor!(dpy, 52  /* XC fleur */)
            },
            dpy: dpy,
            workspaces: Vec::new(),
            colors: unsafe { mem::zeroed() },
            keybindings: Vec::new(),
            active: Active {
                workspace: 0,
                window: root
            }
        };
    }
    
    state.colors = STYLE.colors.to_xft(&mut state);

    unsafe {
        XChangeWindowAttributes(state.dpy, state.root, CWEventMask | CWCursor,  &mut XSetWindowAttributes {
            background_pixmap: 0,
            background_pixel: 0,
            border_pixmap: xlib::CopyFromParent as u64,
            border_pixel: 0,
            bit_gravity: xlib::ForgetGravity,
            win_gravity: xlib::NorthWestGravity,
            backing_store: 0,
            backing_planes: 1,
            backing_pixel: 0,
            save_under: 0,
            event_mask: xlib::SubstructureRedirectMask | 
                        xlib::SubstructureNotifyMask | 
                        xlib::PointerMotionHintMask | 
                        xlib::EnterWindowMask | 
                        xlib::LeaveWindowMask | 
                        xlib::StructureNotifyMask | 
                        xlib::PropertyChangeMask,
            do_not_propagate_mask: 0,
            override_redirect: 0,
            colormap: xlib::CopyFromParent as u64,
            cursor: state.cursor.normal
        });
    }

    state
}