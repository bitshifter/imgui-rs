use imgui_sys;
use imgui_sys::ImVec2;
use std::{f32, mem, ptr};
use libc::c_float;
#[must_use]
pub struct PlotLines<'p> {
    label: &'p str,
    values: &'p [f32],
    values_offset: usize,
    overlay_text: Option<&'p str>,
    scale_min: f32,
    scale_max: f32,
    graph_size: ImVec2,
}

impl<'p> PlotLines<'p> {
    pub fn new(label: &'p str, values: &'p [f32]) -> Self {
        PlotLines {
            label: label,
            values: values,
            values_offset: 0usize,
            overlay_text: None,
            scale_min: f32::MAX,
            scale_max: f32::MAX,
            graph_size: ImVec2::new(0.0f32, 0.0f32),
        }
    }

    #[inline]
    pub fn values_offset(self, values_offset: usize) -> Self {
        PlotLines { values_offset: values_offset, ..self }
    }

    #[inline]
    pub fn overlay_text(self, overlay_text: &'p str) -> Self {
        PlotLines { overlay_text: Some(overlay_text), ..self }
    }

    #[inline]
    pub fn scale_min(self, scale_min: f32) -> Self {
        PlotLines { scale_min: scale_min, ..self }
    }

    #[inline]
    pub fn scale_max(self, scale_max: f32) -> Self {
        PlotLines { scale_max: scale_max, ..self }
    }

    #[inline]
    pub fn graph_size(self, graph_size: ImVec2) -> Self {
        PlotLines { graph_size: graph_size, ..self }
    }

    pub fn build(self) {
        unsafe {
            imgui_sys::igPlotLines(imgui_sys::ImStr::from(self.label),
                                   self.values.as_ptr() as *const c_float,
                                   self.values.len() as i32,
                                   self.values_offset as i32,
                                   self.overlay_text.map( |x| imgui_sys::ImStr::from(x)).unwrap_or(
                                       imgui_sys::ImStr::null()),
                                   self.scale_min,
                                   self.scale_max,
                                   self.graph_size,
                                   mem::size_of::<f32>() as i32);
        }
    }
}