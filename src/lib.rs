#[cfg(feature = "glium")]
#[macro_use]
extern crate glium;

extern crate imgui_sys;

extern crate libc;

use libc::{c_float, c_int, c_uchar, c_void, uintptr_t};
use std::convert::From;
use std::ffi::{CStr, CString};
use std::mem;
use std::ptr;
use std::slice;
use std::str;

pub use imgui_sys::{ImDrawIdx, ImDrawVert, ImGuiInputTextFlags, ImGuiInputTextFlags_AllowTabInput,
                    ImGuiInputTextFlags_AlwaysInsertMode, ImGuiInputTextFlags_AutoSelectAll,
                    ImGuiInputTextFlags_CallbackAlways, ImGuiInputTextFlags_CallbackCharFilter,
                    ImGuiInputTextFlags_CallbackCompletion, ImGuiInputTextFlags_CallbackHistory,
                    ImGuiInputTextFlags_CharsDecimal, ImGuiInputTextFlags_CharsHexadecimal,
                    ImGuiInputTextFlags_CharsNoBlank, ImGuiInputTextFlags_CharsUppercase,
                    ImGuiInputTextFlags_CtrlEnterForNewLine, ImGuiInputTextFlags_EnterReturnsTrue,
                    ImGuiInputTextFlags_NoHorizontalScroll, ImGuiInputTextFlags_Password,
                    ImGuiInputTextFlags_ReadOnly, ImGuiKey, ImGuiSelectableFlags,
                    ImGuiSelectableFlags_DontClosePopups, ImGuiSelectableFlags_SpanAllColumns,
                    ImGuiSetCond, ImGuiSetCond_Always, ImGuiSetCond_Appearing,
                    ImGuiSetCond_FirstUseEver, ImGuiSetCond_Once, ImGuiStyle, ImGuiTreeNodeFlags,
                    ImGuiTreeNodeFlags_AllowOverlapMode, ImGuiTreeNodeFlags_Bullet,
                    ImGuiTreeNodeFlags_CollapsingHeader, ImGuiTreeNodeFlags_DefaultOpen,
                    ImGuiTreeNodeFlags_Framed, ImGuiTreeNodeFlags_Leaf,
                    ImGuiTreeNodeFlags_NoAutoOpenOnLog, ImGuiTreeNodeFlags_NoTreePushOnOpen,
                    ImGuiTreeNodeFlags_OpenOnArrow, ImGuiTreeNodeFlags_OpenOnDoubleClick,
                    ImGuiTreeNodeFlags_Selected, ImGuiWindowFlags,
                    ImGuiWindowFlags_AlwaysAutoResize, ImGuiWindowFlags_AlwaysHorizontalScrollbar,
                    ImGuiWindowFlags_AlwaysUseWindowPadding,
                    ImGuiWindowFlags_AlwaysVerticalScrollbar,
                    ImGuiWindowFlags_HorizontalScrollbar, ImGuiWindowFlags_MenuBar,
                    ImGuiWindowFlags_NoBringToFrontOnFocus, ImGuiWindowFlags_NoCollapse,
                    ImGuiWindowFlags_NoFocusOnAppearing, ImGuiWindowFlags_NoInputs,
                    ImGuiWindowFlags_NoMove, ImGuiWindowFlags_NoResize,
                    ImGuiWindowFlags_NoSavedSettings, ImGuiWindowFlags_NoScrollWithMouse,
                    ImGuiWindowFlags_NoScrollbar, ImGuiWindowFlags_NoTitleBar,
                    ImGuiWindowFlags_ShowBorders, ImVec2, ImVec4};
pub use input::{ColorEdit3, ColorEdit4, InputFloat, InputFloat2, InputFloat3, InputFloat4,
                InputInt, InputInt2, InputInt3, InputInt4, InputText};
pub use menus::{Menu, MenuItem};
pub use plothistogram::PlotHistogram;
pub use plotlines::PlotLines;
pub use sliders::{SliderFloat, SliderInt};
pub use trees::{CollapsingHeader, TreeNode};
pub use window::Window;

mod input;
mod menus;
mod plothistogram;
mod plotlines;
mod sliders;
mod trees;
mod window;

#[cfg(feature = "glium")]
pub mod glium_renderer;

pub struct ImGui {
    // We need to keep ownership of the CString values to ensure the *const char pointer
    // lives long enough
    ini_filename: Option<CString>,
    log_filename: Option<CString>,
}

pub struct TextureHandle<'a> {
    pub width: u32,
    pub height: u32,
    pub pixels: &'a [c_uchar],
}

pub fn get_version() -> &'static str {
    unsafe {
        let bytes = CStr::from_ptr(imgui_sys::igGetVersion()).to_bytes();
        str::from_utf8_unchecked(bytes)
    }
}

impl ImGui {
    pub fn init() -> ImGui {
        ImGui {
            ini_filename: None,
            log_filename: None,
        }
    }
    fn io(&self) -> &imgui_sys::ImGuiIO { unsafe { mem::transmute(imgui_sys::igGetIO()) } }
    fn io_mut(&mut self) -> &mut imgui_sys::ImGuiIO {
        unsafe { mem::transmute(imgui_sys::igGetIO()) }
    }
    pub fn style(&self) -> &ImGuiStyle { unsafe { mem::transmute(imgui_sys::igGetStyle()) } }
    pub fn style_mut(&self) -> &mut ImGuiStyle {
        unsafe { mem::transmute(imgui_sys::igGetStyle()) }
    }
    pub fn prepare_texture<'a, F, T>(&mut self, f: F) -> T
        where F: FnOnce(TextureHandle<'a>) -> T
    {
        let io = self.io();
        let mut pixels: *mut c_uchar = ptr::null_mut();
        let mut width: c_int = 0;
        let mut height: c_int = 0;
        let mut bytes_per_pixel: c_int = 0;
        unsafe {
            imgui_sys::ImFontAtlas_GetTexDataAsRGBA32(io.fonts,
                                                      &mut pixels,
                                                      &mut width,
                                                      &mut height,
                                                      &mut bytes_per_pixel);
            f(TextureHandle {
                width: width as u32,
                height: height as u32,
                pixels: slice::from_raw_parts(pixels, (width * height * bytes_per_pixel) as usize),
            })
        }
    }
    pub fn set_texture_id(&mut self, value: uintptr_t) {
        unsafe {
            (*self.io_mut().fonts).tex_id = value as *mut c_void;
        }
    }
    pub fn set_ini_filename(&mut self, value: Option<&str>) {
        match value {
            Some(s) => {
                self.ini_filename = Some(CString::new(s).unwrap());
                self.io_mut().ini_filename = self.ini_filename.as_ref().unwrap().as_ptr();
            }
            None => {
                self.ini_filename = None;
                self.io_mut().ini_filename = ptr::null();
            }
        }
    }
    pub fn set_log_filename(&mut self, value: Option<&str>) {
        match value {
            Some(s) => {
                self.log_filename = Some(CString::new(s).unwrap());
                self.io_mut().log_filename = self.log_filename.as_ref().unwrap().as_ptr();
            }
            None => {
                self.log_filename = None;
                self.io_mut().log_filename = ptr::null();
            }
        }
    }
    pub fn set_ini_saving_rate(&mut self, value: f32) {
        let io = self.io_mut();
        io.ini_saving_rate = value;
    }
    pub fn set_mouse_double_click_time(&mut self, value: f32) {
        let io = self.io_mut();
        io.mouse_double_click_time = value;
    }
    pub fn set_mouse_double_click_max_dist(&mut self, value: f32) {
        let io = self.io_mut();
        io.mouse_double_click_max_dist = value;
    }
    pub fn set_mouse_drag_threshold(&mut self, value: f32) {
        let io = self.io_mut();
        io.mouse_drag_threshold = value;
    }
    pub fn set_key_repeat_delay(&mut self, value: f32) {
        let io = self.io_mut();
        io.key_repeat_delay = value;
    }
    pub fn set_key_repeat_rate(&mut self, value: f32) {
        let io = self.io_mut();
        io.key_repeat_rate = value;
    }
    pub fn display_size(&self) -> (f32, f32) {
        let io = self.io();
        (io.display_size.x, io.display_size.y)
    }
    pub fn display_framebuffer_scale(&self) -> (f32, f32) {
        let io = self.io();
        (io.display_framebuffer_scale.x, io.display_framebuffer_scale.y)
    }
    pub fn mouse_pos(&self) -> (f32, f32) {
        let io = self.io();
        (io.mouse_pos.x, io.mouse_pos.y)
    }
    pub fn set_mouse_pos(&mut self, x: f32, y: f32) {
        let io = self.io_mut();
        io.mouse_pos.x = x;
        io.mouse_pos.y = y;
    }
    pub fn set_mouse_down(&mut self, states: &[bool; 5]) {
        let io = self.io_mut();
        io.mouse_down = *states;
    }
    pub fn set_mouse_wheel(&mut self, value: f32) {
        let io = self.io_mut();
        io.mouse_wheel = value;
    }
    pub fn set_mouse_draw_cursor(&mut self, value: bool) {
        let io = self.io_mut();
        io.mouse_draw_cursor = value;
    }
    pub fn set_key_ctrl(&mut self, value: bool) {
        let io = self.io_mut();
        io.key_ctrl = value;
    }
    pub fn set_key_shift(&mut self, value: bool) {
        let io = self.io_mut();
        io.key_shift = value;
    }
    pub fn set_key_alt(&mut self, value: bool) {
        let io = self.io_mut();
        io.key_alt = value;
    }
    pub fn set_key_super(&mut self, value: bool) {
        let io = self.io_mut();
        io.key_super = value;
    }
    pub fn set_key(&mut self, key: u8, pressed: bool) {
        let io = self.io_mut();
        io.keys_down[key as usize] = pressed;
    }
    pub fn set_imgui_key(&mut self, key: ImGuiKey, mapping: u8) {
        let io = self.io_mut();
        io.key_map[key as usize] = mapping as i32;
    }
    pub fn add_input_character(&mut self, character: char) {
        // TODO: This is slightly better. We should use char::encode_utf8 when it stabilizes
        // to allow us to skip the string intermediate since we can then go directly
        // to bytes
        let mut string = String::new();
        string.push(character);
        let s : &str = &string;
        unsafe {
            imgui_sys::ImGuiIO_AddInputCharactersUTF8(imgui_sys::ImStr::from(s));
        }
    }
    pub fn get_time(&self) -> f32 { unsafe { imgui_sys::igGetTime() } }
    pub fn get_frame_count(&self) -> i32 { unsafe { imgui_sys::igGetFrameCount() } }
    pub fn get_frame_rate(&self) -> f32 { self.io().framerate }
    pub fn frame<'ui, 'a: 'ui>(&'a mut self,
                               size_points: (u32, u32),
                               size_pixels: (u32, u32),
                               delta_time: f32)
                               -> Ui<'ui> {
        {
            let io = self.io_mut();
            io.display_size.x = size_points.0 as c_float;
            io.display_size.y = size_points.1 as c_float;
            io.display_framebuffer_scale.x = if size_points.0 > 0 {
                size_pixels.0 as c_float / size_points.0 as c_float
            } else {
                0.0
            };
            io.display_framebuffer_scale.y = if size_points.1 > 0 {
                size_pixels.1 as c_float / size_points.1 as c_float
            } else {
                0.0
            };
            io.delta_time = delta_time;
        }
        unsafe {
            imgui_sys::igNewFrame();
            CURRENT_UI = Some(Ui { imgui: mem::transmute(self as &'a ImGui) });
        }
        Ui { imgui: self }
    }
}

impl Drop for ImGui {
    fn drop(&mut self) {
        unsafe {
            CURRENT_UI = None;
            imgui_sys::igShutdown();
        }
    }
}

static mut CURRENT_UI: Option<Ui<'static>> = None;

pub struct DrawList<'a> {
    pub cmd_buffer: &'a [imgui_sys::ImDrawCmd],
    pub idx_buffer: &'a [imgui_sys::ImDrawIdx],
    pub vtx_buffer: &'a [imgui_sys::ImDrawVert],
}

pub struct Ui<'ui> {
    imgui: &'ui ImGui,
}

impl<'ui> Ui<'ui> {
    pub fn imgui(&self) -> &ImGui { self.imgui }
    pub fn want_capture_mouse(&self) -> bool {
        let io = self.imgui.io();
        io.want_capture_mouse
    }
    pub fn want_capture_keyboard(&self) -> bool {
        let io = self.imgui.io();
        io.want_capture_keyboard
    }
    pub fn framerate(&self) -> f32 {
        let io = self.imgui.io();
        io.framerate
    }
    pub fn metrics_allocs(&self) -> i32 {
        let io = self.imgui.io();
        io.metrics_allocs
    }
    pub fn metrics_render_vertices(&self) -> i32 {
        let io = self.imgui.io();
        io.metrics_render_vertices
    }
    pub fn metrics_render_indices(&self) -> i32 {
        let io = self.imgui.io();
        io.metrics_render_indices
    }
    pub fn metrics_active_windows(&self) -> i32 {
        let io = self.imgui.io();
        io.metrics_active_windows
    }
    pub fn render<F, E>(self, mut f: F) -> Result<(), E>
        where F: FnMut(&Ui, DrawList) -> Result<(), E>
    {
        unsafe {
            imgui_sys::igRender();

            let draw_data = imgui_sys::igGetDrawData();
            for &cmd_list in (*draw_data).cmd_lists() {
                let draw_list = DrawList {
                    cmd_buffer: (*cmd_list).cmd_buffer.as_slice(),
                    idx_buffer: (*cmd_list).idx_buffer.as_slice(),
                    vtx_buffer: (*cmd_list).vtx_buffer.as_slice(),
                };
                try!(f(&self, draw_list));
            }
            CURRENT_UI = None;
        }
        Ok(())
    }
    pub fn show_user_guide(&self) { unsafe { imgui_sys::igShowUserGuide() }; }
    pub fn show_default_style_editor(&self) {
        unsafe { imgui_sys::igShowStyleEditor(ptr::null_mut()) };
    }
    pub fn show_style_editor<'p>(&self, style: &'p mut ImGuiStyle) {
        unsafe {
            imgui_sys::igShowStyleEditor(style as *mut ImGuiStyle);
        }
    }
    pub fn show_test_window(&self, opened: &mut bool) {
        unsafe {
            imgui_sys::igShowTestWindow(opened);
        }
    }
    pub fn show_metrics_window(&self, opened: &mut bool) {
        unsafe {
            imgui_sys::igShowMetricsWindow(opened);
        }
    }
}

impl<'a> Ui<'a> {
    pub unsafe fn current_ui() -> Option<&'a Ui<'a>> { CURRENT_UI.as_ref() }
}

// Window
impl<'ui> Ui<'ui> {
    pub fn window<'p>(&self, name: &'p str) -> Window<'ui, 'p> { Window::new(name) }
}

// Layout
impl<'ui> Ui<'ui> {
    pub fn separator(&self) { unsafe { imgui_sys::igSeparator() }; }
    pub fn same_line(&self, pos_x: f32) { unsafe { imgui_sys::igSameLine(pos_x, -1.0f32) } }
    pub fn same_line_spacing(&self, pos_x: f32, spacing_w: f32) {
        unsafe { imgui_sys::igSameLine(pos_x, spacing_w) }
    }
    pub fn spacing(&self) { unsafe { imgui_sys::igSpacing() }; }

    pub fn columns<'p>(&self, count: i32, id: &'p str, border: bool) {
        unsafe { imgui_sys::igColumns(count, imgui_sys::ImStr::from(id), border) }
    }

    pub fn next_column(&self) { unsafe { imgui_sys::igNextColumn() } }

    pub fn get_column_index(&self) -> i32 { unsafe { imgui_sys::igGetColumnIndex() } }

    pub fn get_column_offset(&self, column_index: i32) -> f32 {
        unsafe { imgui_sys::igGetColumnOffset(column_index) }
    }

    pub fn set_column_offset(&self, column_index: i32, offset_x: f32) {
        unsafe { imgui_sys::igSetColumnOffset(column_index, offset_x) }
    }

    pub fn get_column_width(&self, column_index: i32) -> f32 {
        unsafe { imgui_sys::igGetColumnWidth(column_index) }
    }

    pub fn get_columns_count(&self) -> i32 { unsafe { imgui_sys::igGetColumnsCount() } }
}

// Widgets
impl<'ui> Ui<'ui> {
    pub fn text<'p>(&self, text: &'p str) {
        // TODO: use igTextUnformatted
        unsafe {
            imgui_sys::igText1(imgui_sys::ImStr::from(text));
        }
    }
    pub fn text_colored<'p, A>(&self, col: A, text: &'p str)
        where A: Into<ImVec4>
    {
        unsafe {
            imgui_sys::igTextColored1(col.into(), imgui_sys::ImStr::from(text));
        }
    }
    pub fn text_disabled<'p>(&self, text: &'p str) {
        unsafe {
            imgui_sys::igTextDisabled1(imgui_sys::ImStr::from(text));
        }
    }
    pub fn text_wrapped<'p>(&self, text: &'p str) {
        unsafe {
            imgui_sys::igTextWrapped1(imgui_sys::ImStr::from(text));
        }
    }
    pub fn label_text<'p>(&self, label: &'p str, text: &'p str) {
        unsafe {
            imgui_sys::igLabelText1(imgui_sys::ImStr::from(label), imgui_sys::ImStr::from(text));
        }
    }
    pub fn bullet(&self) {
        unsafe {
            imgui_sys::igBullet();
        }
    }
    pub fn bullet_text<'p>(&self, text: &'p str) {
        unsafe {
            imgui_sys::igBulletText1(imgui_sys::ImStr::from(text));
        }
    }
    pub fn button<'p>(&self, label: &'p str, size: ImVec2) -> bool {
        unsafe { imgui_sys::igButton(imgui_sys::ImStr::from(label), size) }
    }
    pub fn small_button<'p>(&self, label: &'p str) -> bool {
        unsafe { imgui_sys::igSmallButton(imgui_sys::ImStr::from(label)) }
    }
    pub fn checkbox<'p>(&self, label: &'p str, value: &'p mut bool) -> bool {
        unsafe { imgui_sys::igCheckbox(imgui_sys::ImStr::from(label), value) }
    }
}

// Widgets: Input
impl<'ui> Ui<'ui> {
    pub fn color_edit3<'p>(&self, label: &'p str, value: &'p mut [f32; 3]) -> ColorEdit3<'ui, 'p> {
        ColorEdit3::new(label, value)
    }
    pub fn color_edit4<'p>(&self, label: &'p str, value: &'p mut [f32; 4]) -> ColorEdit4<'ui, 'p> {
        ColorEdit4::new(label, value)
    }
    pub fn input_text<'p>(&self, label: &'p str, buf: &'p mut str) -> InputText<'ui, 'p> {
        InputText::new(label, buf)
    }
    pub fn input_float<'p>(&self, label: &'p str, value: &'p mut f32) -> InputFloat<'ui, 'p> {
        InputFloat::new(label, value)
    }
    pub fn input_float2<'p>(&self,
                            label: &'p str,
                            value: &'p mut [f32; 2])
                            -> InputFloat2<'ui, 'p> {
        InputFloat2::new(label, value)
    }
    pub fn input_float3<'p>(&self,
                            label: &'p str,
                            value: &'p mut [f32; 3])
                            -> InputFloat3<'ui, 'p> {
        InputFloat3::new(label, value)
    }
    pub fn input_float4<'p>(&self,
                            label: &'p str,
                            value: &'p mut [f32; 4])
                            -> InputFloat4<'ui, 'p> {
        InputFloat4::new(label, value)
    }
    pub fn input_int<'p>(&self, label: &'p str, value: &'p mut i32) -> InputInt<'ui, 'p> {
        InputInt::new(label, value)
    }
    pub fn input_int2<'p>(&self, label: &'p str, value: &'p mut [i32; 2]) -> InputInt2<'ui, 'p> {
        InputInt2::new(label, value)
    }
    pub fn input_int3<'p>(&self, label: &'p str, value: &'p mut [i32; 3]) -> InputInt3<'ui, 'p> {
        InputInt3::new(label, value)
    }
    pub fn input_int4<'p>(&self, label: &'p str, value: &'p mut [i32; 4]) -> InputInt4<'ui, 'p> {
        InputInt4::new(label, value)
    }
}

// Widgets: Sliders
impl<'ui> Ui<'ui> {
    pub fn slider_float<'p>(&self,
                            label: &'p str,
                            value: &'p mut f32,
                            min: f32,
                            max: f32)
                            -> SliderFloat<'ui, 'p> {
        SliderFloat::new(label, value, min, max)
    }
    pub fn slider_int<'p>(&self,
                          label: &'p str,
                          value: &'p mut i32,
                          min: i32,
                          max: i32)
                          -> SliderInt<'ui, 'p> {
        SliderInt::new(label, value, min, max)
    }
}

// Widgets: Trees
impl<'ui> Ui<'ui> {
    pub fn tree_node<'p>(&self, id: &'p str) -> TreeNode<'ui, 'p> { TreeNode::new(id) }
    pub fn collapsing_header<'p>(&self, label: &'p str) -> CollapsingHeader<'ui, 'p> {
        CollapsingHeader::new(label)
    }
}

// Widgets: Selectable / Lists
impl<'ui> Ui<'ui> {
    pub fn selectable<'p>(&self,
                          label: &'p str,
                          selected: bool,
                          flags: ImGuiSelectableFlags,
                          size: ImVec2)
                          -> bool {
        unsafe { imgui_sys::igSelectable(imgui_sys::ImStr::from(label), selected, flags, size) }
    }
}

// Widgets: Menus
impl<'ui> Ui<'ui> {
    pub fn main_menu_bar<F>(&self, f: F)
        where F: FnOnce()
    {
        let render = unsafe { imgui_sys::igBeginMainMenuBar() };
        if render {
            f();
            unsafe { imgui_sys::igEndMainMenuBar() };
        }
    }
    pub fn menu_bar<F>(&self, f: F)
        where F: FnOnce()
    {
        let render = unsafe { imgui_sys::igBeginMenuBar() };
        if render {
            f();
            unsafe { imgui_sys::igEndMenuBar() };
        }
    }
    pub fn menu<'p>(&self, label: &'p str) -> Menu<'ui, 'p> { Menu::new(label) }
    pub fn menu_item<'p>(&self, label: &'p str) -> MenuItem<'ui, 'p> { MenuItem::new(label) }
}

// Widgets: Popups
impl<'ui> Ui<'ui> {
    pub fn open_popup<'p>(&self, str_id: &'p str) {
        unsafe { imgui_sys::igOpenPopup(imgui_sys::ImStr::from(str_id)) };
    }
    pub fn popup<'p, F>(&self, str_id: &'p str, f: F)
        where F: FnOnce()
    {
        let render = unsafe { imgui_sys::igBeginPopup(imgui_sys::ImStr::from(str_id)) };
        if render {
            f();
            unsafe { imgui_sys::igEndPopup() };
        }
    }
    pub fn close_current_popup(&self) {
        unsafe { imgui_sys::igCloseCurrentPopup() };
    }
}

// Widgets: Combos
impl<'ui> Ui<'ui> {
    pub fn combo<'p>(&self,
                     label: &'p str,
                     current_item: &mut i32,
                     items: &'p [&'p str],
                     height_in_items: i32)
                     -> bool {
        let items_inner: Vec<imgui_sys::ImStr> = items.into_iter()
            .map(|item| imgui_sys::ImStr::from(*item))
            .collect();
        unsafe {
            imgui_sys::igCombo(imgui_sys::ImStr::from(label),
                               current_item,
                               items_inner.as_ptr(),
                               items_inner.len() as i32,
                               height_in_items)
        }
    }
}

// Widgets: ListBox
impl<'ui> Ui<'ui> {
    pub fn list_box<'p>(&self,
                        label: &'p str,
                        current_item: &mut i32,
                        items: &'p [&'p str],
                        height_in_items: i32)
                        -> bool {
        let items_inner: Vec<imgui_sys::ImStr> = items.into_iter()
            .map(|item| imgui_sys::ImStr::from(*item))
            .collect();
        unsafe {
            imgui_sys::igListBox(imgui_sys::ImStr::from(label),
                                 current_item,
                                 items_inner.as_ptr(),
                                 items_inner.len() as i32,
                                 height_in_items)
        }
    }
}

impl<'ui> Ui<'ui> {
    pub fn plot_lines<'p>(&self, label: &'p str, values: &'p [f32]) -> PlotLines<'p> {
        PlotLines::new(label, values)
    }
}

impl<'ui> Ui<'ui> {
    pub fn plot_histogram<'p>(&self, label: &'p str, values: &'p [f32]) -> PlotHistogram<'p> {
        PlotHistogram::new(label, values)
    }
}
