use imgui_sys;
use std::ptr;

use super::{
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

pub struct WindowParams<'p> {
    pos: (f32, f32),
    pos_cond: ImGuiSetCond,
    size: (f32, f32),
    size_cond: ImGuiSetCond,
    opened: Option<&'p mut bool>,
    bg_alpha: f32,
    flags: ImGuiWindowFlags,
}

impl<'p> WindowParams<'p> {
    pub fn new() -> WindowParams<'p> {
        WindowParams {
            pos: (0.0, 0.0),
            pos_cond: ImGuiSetCond::empty(),
            size: (0.0, 0.0),
            size_cond: ImGuiSetCond::empty(),
            opened: None,
            bg_alpha: -1.0,
            flags: ImGuiWindowFlags::empty(),
        }
    }
    #[inline]
    pub fn position(self, pos: (f32, f32), cond: ImGuiSetCond) -> Self {
        WindowParams {
            pos: pos,
            pos_cond: cond,
            .. self
        }
    }
    #[inline]
    pub fn size(self, size: (f32, f32), cond: ImGuiSetCond) -> Self {
        WindowParams {
            size: size,
            size_cond: cond,
            .. self
        }
    }
    #[inline]
    pub fn opened(self, opened: &'p mut bool) -> Self {
        WindowParams {
            opened: Some(opened),
            .. self
        }
    }
    #[inline]
    pub fn bg_alpha(self, bg_alpha: f32) -> Self {
        WindowParams {
            bg_alpha: bg_alpha,
            .. self
        }
    }
    #[inline]
    pub fn flags(self, flags: ImGuiWindowFlags) -> Self {
        WindowParams {
            flags: flags,
            .. self
        }
    }
    #[inline]
    pub fn title_bar(self, value: bool) -> Self {
        WindowParams {
            flags: self.flags.with(ImGuiWindowFlags_NoTitleBar, !value),
            .. self
        }
    }
    #[inline]
    pub fn resizable(self, value: bool) -> Self {
        WindowParams {
            flags: self.flags.with(ImGuiWindowFlags_NoResize, !value),
            .. self
        }
    }
    #[inline]
    pub fn movable(self, value: bool) -> Self {
        WindowParams {
            flags: self.flags.with(ImGuiWindowFlags_NoMove, !value),
            .. self
        }
    }
    #[inline]
    pub fn scroll_bar(self, value: bool) -> Self {
        WindowParams {
            flags: self.flags.with(ImGuiWindowFlags_NoScrollbar, !value),
            .. self
        }
    }
    #[inline]
    pub fn scrollable(self, value: bool) -> Self {
        WindowParams {
            flags: self.flags.with(ImGuiWindowFlags_NoScrollWithMouse, !value),
            .. self
        }
    }
    #[inline]
    pub fn collapsible(self, value: bool) -> Self {
        WindowParams {
            flags: self.flags.with(ImGuiWindowFlags_NoCollapse, !value),
            .. self
        }
    }
    #[inline]
    pub fn always_auto_resize(self, value: bool) -> Self {
        WindowParams {
            flags: self.flags.with(ImGuiWindowFlags_AlwaysAutoResize, value),
            .. self
        }
    }
    #[inline]
    pub fn show_borders(self, value: bool) -> Self {
        WindowParams {
            flags: self.flags.with(ImGuiWindowFlags_ShowBorders, value),
            .. self
        }
    }
    #[inline]
    pub fn save_settings(self, value: bool) -> Self {
        WindowParams {
            flags: self.flags.with(ImGuiWindowFlags_NoSavedSettings, !value),
            .. self
        }
    }
    #[inline]
    pub fn inputs(self, value: bool) -> Self {
        WindowParams {
            flags: self.flags.with(ImGuiWindowFlags_NoInputs, !value),
            .. self
        }
    }
    #[inline]
    pub fn menu_bar(self, value: bool) -> Self {
        WindowParams {
            flags: self.flags.with(ImGuiWindowFlags_MenuBar, value),
            .. self
        }
    }
    #[inline]
    pub fn horizontal_scrollbar(self, value: bool) -> Self {
        WindowParams {
            flags: self.flags.with(ImGuiWindowFlags_HorizontalScrollbar, value),
            .. self
        }
    }
    #[inline]
    pub fn no_focus_on_appearing(self, value: bool) -> Self {
        WindowParams {
            flags: self.flags.with(ImGuiWindowFlags_NoFocusOnAppearing, value),
            .. self
        }
    }
    #[inline]
    pub fn no_bring_to_front_on_focus(self, value: bool) -> Self {
        WindowParams {
            flags: self.flags.with(ImGuiWindowFlags_NoBringToFrontOnFocus, value),
            .. self
        }
    }
}

pub fn window<'p, F>(name: ImStr<'p>, params: WindowParams, f: F) where F: FnOnce() {
    let render = unsafe {
        if !params.pos_cond.is_empty() {
            imgui_sys::igSetNextWindowPos(params.pos.into(), params.pos_cond);
        }
        if !params.size_cond.is_empty() {
            imgui_sys::igSetNextWindowSize(params.size.into(), params.size_cond);
        }
        imgui_sys::igBegin2(
            name.as_ptr(),
            params.opened.map(|x| x as *mut bool).unwrap_or(ptr::null_mut()),
            ImVec2::new(0.0, 0.0),
            params.bg_alpha,
            params.flags)
    };
    if render {
        f();
    }
    unsafe { imgui_sys::igEnd() };
}
