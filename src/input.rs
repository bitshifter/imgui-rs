use imgui_sys;
use libc::size_t;
use std::ptr;

use super::{
    Ui,
    ImGuiInputTextFlags, ImGuiInputTextFlags_CharsDecimal, ImGuiInputTextFlags_CharsHexadecimal,
    ImGuiInputTextFlags_CharsUppercase, ImGuiInputTextFlags_CharsNoBlank,
    ImGuiInputTextFlags_AutoSelectAll, ImGuiInputTextFlags_EnterReturnsTrue,
    ImGuiInputTextFlags_CallbackCompletion, ImGuiInputTextFlags_CallbackHistory,
    ImGuiInputTextFlags_CallbackAlways, ImGuiInputTextFlags_CallbackCharFilter,
    ImGuiInputTextFlags_AllowTabInput, //ImGuiInputTextFlags_CtrlEnterForNewLine,
    ImGuiInputTextFlags_NoHorizontalScroll, ImGuiInputTextFlags_AlwaysInsertMode,
    ImStr
};

macro_rules! impl_text_flags {
    ($InputType:ident) => {
        #[inline]
        pub fn flags(&mut self, flags: ImGuiInputTextFlags) -> &mut Self {
            self.flags = flags;
            self
        }

        #[inline]
        pub fn chars_decimal(&mut self, value: bool) -> &mut Self {
            self.flags = self.flags.with(ImGuiInputTextFlags_CharsDecimal, value);
            self
        }

        #[inline]
        pub fn chars_hexadecimal(&mut self, value: bool) -> &mut Self {
            self.flags = self.flags.with(ImGuiInputTextFlags_CharsHexadecimal, value);
            self
        }

        #[inline]
        pub fn chars_uppercase(&mut self, value: bool) -> &mut Self {
            self.flags = self.flags.with(ImGuiInputTextFlags_CharsUppercase, value);
            self
        }

        #[inline]
        pub fn chars_noblank(&mut self, value: bool) -> &mut Self {
            self.flags = self.flags.with(ImGuiInputTextFlags_CharsNoBlank, value);
            self
        }

        #[inline]
        pub fn auto_select_all(&mut self, value: bool) -> &mut Self {
            self.flags = self.flags.with(ImGuiInputTextFlags_AutoSelectAll, value);
            self
        }

        #[inline]
        pub fn enter_returns_true(&mut self, value: bool) -> &mut Self {
            self.flags = self.flags.with(ImGuiInputTextFlags_EnterReturnsTrue, value);
            self
        }

        #[inline]
        pub fn callback_completion(&mut self, value: bool) -> &mut Self {
            self.flags = self.flags.with(ImGuiInputTextFlags_CallbackCompletion, value);
            self
        }

        #[inline]
        pub fn callback_history(&mut self, value: bool) -> &mut Self {
            self.flags = self.flags.with(ImGuiInputTextFlags_CallbackHistory, value);
            self
        }

        #[inline]
        pub fn callback_always(&mut self, value: bool) -> &mut Self {
            self.flags = self.flags.with(ImGuiInputTextFlags_CallbackAlways, value);
            self
        }

        #[inline]
        pub fn callback_char_filter(&mut self, value: bool) -> &mut Self {
            self.flags = self.flags.with(ImGuiInputTextFlags_CallbackCharFilter, value);
            self
        }

        #[inline]
        pub fn allow_tab_input(&mut self, value: bool) -> &mut Self {
            self.flags = self.flags.with(ImGuiInputTextFlags_AllowTabInput, value);
            self
        }

        #[inline]
        pub fn no_horizontal_scroll(&mut self, value: bool) -> &mut Self {
            self.flags = self.flags.with(ImGuiInputTextFlags_NoHorizontalScroll, value);
            self
        }

        #[inline]
        pub fn always_insert_mode(&mut self, value: bool) -> &mut Self {
            self.flags = self.flags.with(ImGuiInputTextFlags_AlwaysInsertMode, value);
            self
        }

    }
}

macro_rules! impl_step_params {
    ($InputType:ident, $Value:ty) => {
        #[inline]
        pub fn step(&mut self, value: $Value) -> &mut Self {
            self.step = value;
            self
        }

        #[inline]
        pub fn step_fast(&mut self, value: $Value) -> &mut Self {
            self.step_fast = value;
            self
        }
    }
}

macro_rules! impl_precision_params {
    ($InputType:ident) => {
        #[inline]
        pub fn decimal_precision(&mut self, value: i32) -> &mut Self {
            self.decimal_precision = value;
            self
        }
    }
}

#[must_use]
pub struct InputTextOptions {
    flags: ImGuiInputTextFlags
}

impl InputTextOptions {
    pub fn new() -> Self {
        InputTextOptions {
            flags: ImGuiInputTextFlags::empty(),
        }
    }

    impl_text_flags!(InputTextOptions);

    // TODO: boxed closure...?
    // pub fn callback(self) -> Self { }

    pub fn input_text<'ui, 'p>(&self, _ui: &'ui Ui, label: ImStr<'p>, buf: &'p mut str) -> bool {
        unsafe {
            imgui_sys::igInputText(
                label.as_ptr(),
                // TODO: this is evil. Perhaps something else than &mut str is better
                buf.as_ptr() as *mut i8,
                buf.len() as size_t,
                self.flags,
                None,
                ptr::null_mut())
        }
    }
}

#[must_use]
pub struct InputIntOptions {
    step: i32,
    step_fast: i32,
    flags: ImGuiInputTextFlags,
}

macro_rules! impl_input_intn {
    ($input_intn:ident, $N:expr, $igInputIntN:ident) => {
        pub fn $input_intn<'ui, 'p>(&self, _ui: &'ui Ui, label: ImStr<'p>, value: &'p mut [i32;$N]) -> bool {
            unsafe {
                imgui_sys::$igInputIntN(
                    label.as_ptr(),
                    value.as_mut_ptr(),
                    self.flags)
            }
        }
    }
}

impl InputIntOptions {
    pub fn new() -> Self {
        InputIntOptions {
            step: 1,
            step_fast: 100,
            flags: ImGuiInputTextFlags::empty(),
        }
    }

    impl_step_params!(InputIntOptions, i32);
    impl_text_flags!(InputIntOptions);

    pub fn input_int<'ui, 'p>(&self, _ui: &'ui Ui, label: ImStr<'p>, value: &'p mut i32) -> bool {
        unsafe {
            imgui_sys::igInputInt(
                label.as_ptr(),
                value as *mut i32,
                self.step,
                self.step_fast,
                self.flags)
        }
    }

    impl_input_intn!(input_int2, 2, igInputInt2);
    impl_input_intn!(input_int3, 3, igInputInt3);
    impl_input_intn!(input_int4, 4, igInputInt4);
}

#[must_use]
pub struct InputFloatOptions {
    step: f32,
    step_fast: f32,
    decimal_precision: i32,
    flags: ImGuiInputTextFlags,
}

macro_rules! impl_input_floatn {
    ($input_floatn:ident, $N:expr, $igInputFloatN:ident) => {
        pub fn $input_floatn<'ui, 'p>(&self, _ui: &'ui Ui, label: ImStr<'p>, value: &'p mut [f32;$N]) -> bool {
            unsafe {
                imgui_sys::$igInputFloatN(
                    label.as_ptr(),
                    value.as_mut_ptr(),
                    self.decimal_precision,
                    self.flags)
            }
        }
    }
}

impl InputFloatOptions {
    pub fn new() -> Self {
        InputFloatOptions {
            step: 0.0,
            step_fast: 0.0,
            decimal_precision: -1,
            flags: ImGuiInputTextFlags::empty(),
        }
    }

    impl_step_params!(InputFloatOptions, f32);
    impl_precision_params!(InputFloatOptions);
    impl_text_flags!(InputFloatOptions);

    pub fn input_float<'ui, 'p>(&self, _ui: &'ui Ui, label: ImStr<'p>, value: &'p mut f32) -> bool {
        unsafe {
            imgui_sys::igInputFloat(
                label.as_ptr(),
                value as *mut f32,
                self.step,
                self.step_fast,
                self.decimal_precision,
                self.flags)
        }
    }

    impl_input_floatn!(input_float2, 2, igInputFloat2);
    impl_input_floatn!(input_float3, 3, igInputFloat3);
    impl_input_floatn!(input_float4, 4, igInputFloat4);
}


pub fn color_edit3<'ui, 'p>(_ui: &'ui Ui, label: ImStr<'p>, value: &'p mut [f32;3]) -> bool {
    unsafe {
        imgui_sys::igColorEdit3(
            label.as_ptr(),
            value.as_mut_ptr())
    }
}

pub fn color_edit4<'ui, 'p>(_ui: &'ui Ui, label: ImStr<'p>, value: &'p mut [f32;4], show_alpha: bool) -> bool {
    unsafe {
        imgui_sys::igColorEdit4(
            label.as_ptr(),
            value.as_mut_ptr(),
            show_alpha)
    }
}
