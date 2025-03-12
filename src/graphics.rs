use sokol::gfx as sg;
use std::{ffi::CStr, path::Path};

use crate::generated::{self, simple_shader_desc};

#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct Vertex {
    pub position: [f32; 2],
    pub uv: [f32; 2],
}

pub struct Pipeline {
    pub pip: sg::Pipeline,
    pub shd: sg::Shader,
    pub bind: sg::Bindings,
}

impl Pipeline {
    pub fn new() -> Self {
        let shd_desc = simple_shader_desc(sg::query_backend());
        let shd = sg::make_shader(&shd_desc);

        let pip_desc = sg::PipelineDesc {
            shader: shd,
            layout: {
                let mut layout = sg::VertexLayoutState::new();
                layout.buffers[0].stride = 28;

                layout.attrs[generated::ATTR_SIMPLE_POSITION].format = sg::VertexFormat::Float2;
                layout.attrs[generated::ATTR_SIMPLE_TEXCOORD].format = sg::VertexFormat::Float2;

                layout
            },
            ..Default::default()
        };

        let pip = sg::make_pipeline(&pip_desc);
        let bind = sg::Bindings::new();

        Self { pip, shd, bind }
    }
}
