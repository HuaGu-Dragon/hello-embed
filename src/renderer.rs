use esp_hal::DriverMode;
use mini_renderer::{
    graphics::{Face, primitive::PrimitiveState, topology::PrimitiveTopology},
    math::Vec4,
    pipeline::shader::{FragmentShader, VertexOutput, VertexShader},
    renderer::{Renderer, create_render_pipeline},
};

use crate::lcd::{Lcd, Rect};

extern crate alloc;

struct MyVertex {
    position: (f32, f32, f32),
    color: (f32, f32, f32),
}

#[derive(Clone, Copy, mini_renderer::Varying)]
struct MyVarying {
    color: (f32, f32, f32),
}

struct MyVertexShader;

impl VertexShader for MyVertexShader {
    type Vertex = MyVertex;
    type Varying = MyVarying;
    type Uniform = f32;

    fn vs_main(
        &self,
        _index: usize,
        vertex: &Self::Vertex,
        _uniform: &Self::Uniform,
    ) -> VertexOutput<Self::Varying> {
        VertexOutput {
            position: Vec4::new(vertex.position.0, vertex.position.1, 0.0, 1.0),
            varying: MyVarying {
                color: vertex.color,
            },
        }
    }
}

struct MyFragmentShader;

impl FragmentShader for MyFragmentShader {
    type Varying = MyVarying;
    type Output = u16;
    type Uniform = f32;

    fn fs_main(&self, varying: &Self::Varying, uniform: &Self::Uniform) -> Option<Self::Output> {
        let (r, g, b) = varying.color;
        let r = (libm::sinf(r + uniform) * 31.0) as u16;
        let g = (libm::sinf(g + uniform) * 63.0) as u16;
        let b = (libm::sinf(b + uniform) * 31.0) as u16;
        Some((r << 11) | (g << 5) | b)
        // Some(0xffff)
    }
}

pub async fn render<D: DriverMode>(lcd: &mut Lcd<'_, D>, timer: f32) {
    let vertices = [
        MyVertex {
            position: (-0.5, -0.5, 0.0),
            color: (1.0, 0.0, 0.0),
        },
        MyVertex {
            position: (0.5, -0.5, 0.0),
            color: (0.0, 1.0, 0.0),
        },
        MyVertex {
            position: (0.0, 0.5, 0.0),
            color: (0.0, 0.0, 1.0),
        },
    ];
    // let vertices = get_vertices();
    // let indices = [0usize, 1, 2];
    let mut framebuffer = [0u16; 240 * 240];
    // let mut depth_buffer = [1.0; 240 * 240];

    let renderer = Renderer::new(240, 240);
    let mut pipeline = create_render_pipeline(
        MyVertexShader,
        MyFragmentShader,
        PrimitiveState::default().with_cull_mode(Face::Back),
    );

    renderer
        .begin_render_pass()
        .set_pipeline(&mut pipeline)
        // .with_depth(&mut depth_buffer)
        // .with_blend()
        .draw(&vertices, &mut framebuffer, &timer);
    // .draw_indexed(&vertices, indices.into_iter(), &mut framebuffer, &());

    lcd.draw_buf(
        Rect {
            start_x: 0,
            end_x: 239,
            start_y: 0,
            end_y: 239,
        },
        &framebuffer,
    );
}

// fn get_vertices() -> alloc::vec::Vec<MyVertex> {
//     let f = include_str!("../teapot");
//     let mut vertices = alloc::vec::Vec::with_capacity(1024);
//     let mut lines = f.lines().take(2048);
//     while let Some(line) = lines.next() {
//         let mut iter = line.split_whitespace();
//         if let Some(x) = iter.next() {
//             if let Some(y) = iter.next() {
//                 if let Some(z) = iter.next() {
//                     vertices.push(MyVertex {
//                         position: (x.parse().unwrap(), y.parse().unwrap(), z.parse().unwrap()),
//                         // color: (0.0, 0.0, 0.0),
//                     });
//                 }
//             }
//         }
//         lines.next();
//     }

//     vertices
// }
