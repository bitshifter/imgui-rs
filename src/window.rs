use imgui_sys;
use std::ptr;

use super::{
    Ui,
    ImGuiSetCond,
    ImGuiWindowFlags,
    ImGuiWindowFlags_NoTitleBar, ImGuiWindowFlags_NoResize, ImGuiWindowFlags_NoMove,
    ImGuiWindowFlags_NoScrollbar, ImGuiWindowFlags_NoScrollWithMouse, ImGuiWindowFlags_NoCollapse,
    ImGuiWindowFlags_AlwaysAutoResize, ImGuiWindowFlags_ShowBorders,
    ImGuiWindowFlags_NoSavedSettings, ImGuiWindowFlags_NoInputs, ImGuiWindowFlags_MenuBar,
    ImGuiWindowFlags_HorizontalScrollbar, ImGuiWindowFlags_NoFocusOnAppearing,
    ImGuiWindowFlags_NoBringToFrontOnFocus,
    ImStr, ImVec2
};

pub struct WindowOptions {
    pos: (f32, f32),
    pos_cond: ImGuiSetCond,
    size: (f32, f32),
    size_cond: ImGuiSetCond,
    bg_alpha: f32,
    flags: ImGuiWindowFlags,
}

impl WindowOptions {
    pub fn new() -> WindowOptions {
        WindowOptions {
            pos: (0.0, 0.0),
            pos_cond: ImGuiSetCond::empty(),
            size: (0.0, 0.0),
            size_cond: ImGuiSetCond::empty(),
            bg_alpha: -1.0,
            flags: ImGuiWindowFlags::empty(),
        }
    }
    #[inline]
    pub fn position(&mut self, pos: (f32, f32), cond: ImGuiSetCond) -> &mut Self {
        self.pos = pos;
        self.pos_cond = cond;
        self
    }
    #[inline]
    pub fn size(&mut self, size: (f32, f32), cond: ImGuiSetCond) -> &mut Self {
        self.size = size;
        self.size_cond = cond;
        self
    }
    #[inline]
    pub fn bg_alpha(&mut self, bg_alpha: f32) -> &mut Self {
        self.bg_alpha = bg_alpha;
        self
    }
    #[inline]
    pub fn flags(&mut self, flags: ImGuiWindowFlags) -> &mut Self {
        self.flags = flags;
        self
    }
    #[inline]
    pub fn title_bar(&mut self, value: bool) -> &mut Self {
        self.flags = self.flags.with(ImGuiWindowFlags_NoTitleBar, !value);
        self
    }
    #[inline]
    pub fn resizable(&mut self, value: bool) -> &mut Self {
        self.flags = self.flags.with(ImGuiWindowFlags_NoResize, !value);
        self
    }
    #[inline]
    pub fn movable(&mut self, value: bool) -> &mut Self {
        self.flags = self.flags.with(ImGuiWindowFlags_NoMove, !value);
        self
    }
    #[inline]
    pub fn scroll_bar(&mut self, value: bool) -> &mut Self {
        self.flags = self.flags.with(ImGuiWindowFlags_NoScrollbar, !value);
        self
    }
    #[inline]
    pub fn scrollable(&mut self, value: bool) -> &mut Self {
        self.flags = self.flags.with(ImGuiWindowFlags_NoScrollWithMouse, !value);
        self
    }
    #[inline]
    pub fn collapsible(&mut self, value: bool) -> &mut Self {
        self.flags = self.flags.with(ImGuiWindowFlags_NoCollapse, !value);
        self
    }
    #[inline]
    pub fn always_auto_resize(&mut self, value: bool) -> &mut Self {
        self.flags = self.flags.with(ImGuiWindowFlags_AlwaysAutoResize, value);
        self
    }
    #[inline]
    pub fn show_borders(&mut self, value: bool) -> &mut Self {
        self.flags = self.flags.with(ImGuiWindowFlags_ShowBorders, value);
        self
    }
    #[inline]
    pub fn save_settings(&mut self, value: bool) -> &mut Self {
        self.flags = self.flags.with(ImGuiWindowFlags_NoSavedSettings, !value);
        self
    }
    #[inline]
    pub fn inputs(&mut self, value: bool) -> &mut Self {
        self.flags = self.flags.with(ImGuiWindowFlags_NoInputs, !value);
        self
    }
    #[inline]
    pub fn menu_bar(&mut self, value: bool) -> &mut Self {
        self.flags = self.flags.with(ImGuiWindowFlags_MenuBar, value);
        self
    }
    #[inline]
    pub fn horizontal_scrollbar(&mut self, value: bool) -> &mut Self {
        self.flags = self.flags.with(ImGuiWindowFlags_HorizontalScrollbar, value);
        self
    }
    #[inline]
    pub fn no_focus_on_appearing(&mut self, value: bool) -> &mut Self {
        self.flags = self.flags.with(ImGuiWindowFlags_NoFocusOnAppearing, value);
        self
    }
    #[inline]
    pub fn no_bring_to_front_on_focus(&mut self, value: bool) -> &mut Self {
        self.flags = self.flags.with(ImGuiWindowFlags_NoBringToFrontOnFocus, value);
        self
    }

    #[inline]
    pub fn window<'ui, 'p, F>(&self, ui: &'ui Ui, name: ImStr<'p>, f: F) where F: FnOnce() {
        self.window_opened_impl(ui, name, None, f)
    }
    pub fn window_opened<'ui, 'p, F>(&self, ui: &'ui Ui, name: ImStr<'p>, opened: &'p mut bool, f: F)
        where F: FnOnce() {
            self.window_opened_impl(ui, name, Some(opened), f)
        }
    fn window_opened_impl<'ui, 'p, F>(&self, _ui: &'ui Ui, name: ImStr<'p>, opened: Option<&'p mut bool>, f: F)
        where F: FnOnce() {
            let render = unsafe {
                if !self.pos_cond.is_empty() {
                    imgui_sys::igSetNextWindowPos(self.pos.into(), self.pos_cond);
                }
                if !self.size_cond.is_empty() {
                    imgui_sys::igSetNextWindowSize(self.size.into(), self.size_cond);
                }
                imgui_sys::igBegin2(
                    name.as_ptr(),
                    opened.map(|x| x as *mut bool).unwrap_or(ptr::null_mut()),
                    ImVec2::new(0.0, 0.0),
                    self.bg_alpha,
                    self.flags)
            };
            if render {
                f();
            }
            unsafe { imgui_sys::igEnd() };
        }
}

