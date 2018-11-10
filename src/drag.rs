use std::marker::PhantomData;
use sys;

use super::Ui;

macro_rules! impl_display_format {
    ($InputType:ident) => {
        #[inline]
        pub fn display_format(mut self, display_format: &'p str) -> Self {
            self.display_format = display_format;
            self
        }
    }
}

macro_rules! impl_speed {
    ($InputType:ident) => {
        #[inline]
        pub fn speed(mut self, value: f32) -> Self {
            self.speed = value;
            self
        }
    }
}

macro_rules! impl_power {
    ($InputType:ident) => {
        #[inline]
        pub fn power(mut self, value: f32) -> Self {
            self.power = value;
            self
        }
    }
}

macro_rules! impl_min_max {
    ($InputType:ident, $Value:ty) => {
        #[inline]
        pub fn min(mut self, value: $Value) -> Self {
            self.min = value;
            self
        }

        #[inline]
        pub fn max(mut self, value: $Value) -> Self {
            self.max = value;
            self
        }
    }
}

#[must_use]
pub struct DragFloat<'ui, 'p> {
    label: &'p str,
    value: &'p mut f32,
    speed: f32,
    min: f32,
    max: f32,
    display_format: &'p str,
    power: f32,
    _phantom: PhantomData<&'ui Ui<'ui>>,
}

impl<'ui, 'p> DragFloat<'ui, 'p> {
    pub fn new(_: &Ui<'ui>, label: &'p str, value: &'p mut f32) -> Self {
        DragFloat {
            label,
            value,
            speed: 1.0,
            min: 0.0,
            max: 0.0,
            display_format: "%.3f",
            power: 1.0,
            _phantom: PhantomData,
        }
    }

    pub fn build(self) -> bool {
        unsafe {
            sys::igDragFloat(
                self.label.into(),
                self.value as *mut f32,
                self.speed,
                self.min,
                self.max,
                self.display_format.into(),
                self.power,
            )
        }
    }

    impl_display_format!(DragFloat);
    impl_min_max!(DragFloat, f32);
    impl_speed!(DragFloat);
    impl_power!(DragFloat);
}

macro_rules! impl_drag_floatn {
    ($DragFloatN:ident, $N:expr, $igDragFloatN:ident) => {
        #[must_use]
        pub struct $DragFloatN<'ui, 'p> {
            label: &'p str,
            value: &'p mut [f32; $N],
            speed: f32,
            min: f32,
            max: f32,
            display_format: &'p str,
            power: f32,
            _phantom: PhantomData<&'ui Ui<'ui>>,
        }

        impl<'ui, 'p> $DragFloatN<'ui, 'p> {
            pub fn new(_: &Ui<'ui>, label: &'p str, value: &'p mut [f32; $N]) -> Self {
                $DragFloatN {
                    label,
                    value,
                    speed: 1.0,
                    min: 0.0,
                    max: 0.0,
                    display_format: "%.3f",
                    power: 1.0,
                    _phantom: PhantomData,
                }
            }

            pub fn build(self) -> bool {
                unsafe {
                    sys::$igDragFloatN(
                        self.label.into(),
                        self.value.as_mut_ptr(),
                        self.speed,
                        self.min,
                        self.max,
                        self.display_format.into(),
                        self.power,
                    )
                }
            }

            impl_display_format!(DragFloat);
            impl_min_max!(DragFloat, f32);
            impl_speed!(DragFloat);
            impl_power!(DragFloat);
        }
    };
}

impl_drag_floatn!(DragFloat2, 2, igDragFloat2);
impl_drag_floatn!(DragFloat3, 3, igDragFloat3);
impl_drag_floatn!(DragFloat4, 4, igDragFloat4);

#[must_use]
pub struct DragFloatRange2<'ui, 'p> {
    label: &'p str,
    current_min: &'p mut f32,
    current_max: &'p mut f32,
    speed: f32,
    min: f32,
    max: f32,
    display_format: &'p str,
    display_format_max: Option<&'p str>,
    power: f32,
    _phantom: PhantomData<&'ui Ui<'ui>>,
}

impl<'ui, 'p> DragFloatRange2<'ui, 'p> {
    pub fn new(
        _: &Ui<'ui>,
        label: &'p str,
        current_min: &'p mut f32,
        current_max: &'p mut f32,
    ) -> Self {
        DragFloatRange2 {
            label,
            current_min,
            current_max,
            speed: 1.0,
            min: 0.0,
            max: 0.0,
            display_format: "%.3f",
            display_format_max: None,
            power: 1.0,
            _phantom: PhantomData,
        }
    }

    pub fn build(self) -> bool {
        unsafe {
            sys::igDragFloatRange2(
                self.label.into(),
                self.current_min as *mut f32,
                self.current_max as *mut f32,
                self.speed,
                self.min,
                self.max,
                self.display_format.into(),
                self.display_format_max.map_or(sys::ImStr::null(), |f| f.into()),
                self.power,
            )
        }
    }

    #[inline]
    pub fn display_format_max(mut self, display_format: Option<&'p str>) -> Self {
        self.display_format_max = display_format;
        self
    }

    impl_display_format!(DragFloatRange2);
    impl_min_max!(DragFloatRange2, f32);
    impl_speed!(DragFloatRange2);
    impl_power!(DragFloatRange2);
}

#[must_use]
pub struct DragInt<'ui, 'p> {
    label: &'p str,
    value: &'p mut i32,
    speed: f32,
    min: i32,
    max: i32,
    display_format: &'p str,
    _phantom: PhantomData<&'ui Ui<'ui>>,
}

impl<'ui, 'p> DragInt<'ui, 'p> {
    pub fn new(_: &Ui<'ui>, label: &'p str, value: &'p mut i32) -> Self {
        DragInt {
            label,
            value,
            speed: 1.0,
            min: 0,
            max: 0,
            display_format: "%.0f",
            _phantom: PhantomData,
        }
    }

    pub fn build(self) -> bool {
        unsafe {
            sys::igDragInt(
                self.label.into(),
                self.value as *mut i32,
                self.speed,
                self.min,
                self.max,
                self.display_format.into(),
            )
        }
    }

    impl_display_format!(DragInt);
    impl_min_max!(DragInt, i32);
    impl_speed!(DragInt);
}

macro_rules! impl_drag_intn {
    ($DragIntN:ident, $N:expr, $igDragIntN:ident) => {
        #[must_use]
        pub struct $DragIntN<'ui, 'p> {
            label: &'p str,
            value: &'p mut [i32; $N],
            speed: f32,
            min: i32,
            max: i32,
            display_format: &'p str,
            _phantom: PhantomData<&'ui Ui<'ui>>,
        }

        impl<'ui, 'p> $DragIntN<'ui, 'p> {
            pub fn new(_: &Ui<'ui>, label: &'p str, value: &'p mut [i32; $N]) -> Self {
                $DragIntN {
                    label,
                    value,
                    speed: 1.0,
                    min: 0,
                    max: 0,
                    display_format: "%.0f",
                    _phantom: PhantomData,
                }
            }

            pub fn build(self) -> bool {
                unsafe {
                    sys::$igDragIntN(
                        self.label.into(),
                        self.value.as_mut_ptr(),
                        self.speed,
                        self.min,
                        self.max,
                        self.display_format.into(),
                    )
                }
            }

            impl_display_format!(DragInt);
            impl_min_max!(DragInt, i32);
            impl_speed!(DragInt);
        }
    };
}

impl_drag_intn!(DragInt2, 2, igDragInt2);
impl_drag_intn!(DragInt3, 3, igDragInt3);
impl_drag_intn!(DragInt4, 4, igDragInt4);

#[must_use]
pub struct DragIntRange2<'ui, 'p> {
    label: &'p str,
    current_min: &'p mut i32,
    current_max: &'p mut i32,
    speed: f32,
    min: i32,
    max: i32,
    display_format: &'p str,
    display_format_max: Option<&'p str>,
    _phantom: PhantomData<&'ui Ui<'ui>>,
}

impl<'ui, 'p> DragIntRange2<'ui, 'p> {
    pub fn new(
        _: &Ui<'ui>,
        label: &'p str,
        current_min: &'p mut i32,
        current_max: &'p mut i32,
    ) -> Self {
        DragIntRange2 {
            label,
            current_min,
            current_max,
            speed: 1.0,
            min: 0,
            max: 0,
            display_format: "%.0f",
            display_format_max: None,
            _phantom: PhantomData,
        }
    }

    pub fn build(self) -> bool {
        unsafe {
            sys::igDragIntRange2(
                self.label.into(),
                self.current_min as *mut i32,
                self.current_max as *mut i32,
                self.speed,
                self.min,
                self.max,
                self.display_format.into(),
                self.display_format_max.map_or(sys::ImStr::null(), |f| f.into()),
            )
        }
    }

    #[inline]
    pub fn display_format_max(mut self, display_format: Option<&'p str>) -> Self {
        self.display_format_max = display_format;
        self
    }

    impl_display_format!(DragIntRange2);
    impl_min_max!(DragIntRange2, i32);
    impl_speed!(DragIntRange2);
}
