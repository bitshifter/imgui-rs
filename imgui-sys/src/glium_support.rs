use glium::vertex::{Attribute, AttributeType, Vertex, VertexFormat};
use std::borrow::Cow;
use std::os::raw::c_float;

use super::{ImDrawVert, ImVec2, ImVec4};

unsafe impl Attribute for ImVec2 {
    fn get_type() -> AttributeType { <(c_float, c_float) as Attribute>::get_type() }
}

unsafe impl Attribute for ImVec4 {
    fn get_type() -> AttributeType {
        <(c_float, c_float, c_float, c_float) as Attribute>::get_type()
    }
}

impl Vertex for ImDrawVert {
    fn build_bindings() -> VertexFormat {
        Cow::Owned(vec![
            (
                "pos".into(),
                0,
                <ImVec2 as Attribute>::get_type(),
                false
            ),
            (
                "uv".into(),
                8,
                <ImVec2 as Attribute>::get_type(),
                false
            ),
            (
                "col".into(),
                16,
                AttributeType::U8U8U8U8,
                false
            ),
        ])
    }
}
