use glium::{implement_vertex, Surface, uniform};
use glium::uniforms::MagnifySamplerFilter;

use crate::sprite::Sprite;

static VERTEX_SHADER_SRC: &str = r#"
    #version 140
    in vec2 position;
    in vec2 tex_coords;
    out vec2 v_tex_coords;
    uniform mat4 matrix;
    void main() {
        v_tex_coords = tex_coords;
        gl_Position = matrix * vec4(position, 0.0, 1.0);
    }
"#;

static FRAGMENT_SHADER_SRC: &str = r#"
    #version 140
    in vec2 v_tex_coords;
    out vec4 color;
    uniform sampler2D tex;
    void main() {
        vec4 texColor = texture(tex, v_tex_coords);
        texColor.rgb = pow(texColor.rgb, vec3(1.0 / 2.2)); // 감마 보정 적용
        color = texture(tex, v_tex_coords);
    }
"#;

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
    tex_coords: [f32; 2],
}

implement_vertex!(Vertex, position, tex_coords);

pub fn render(display: &glium::Display, sprites: Vec<Sprite>) {
    let mut frame = display.draw();
    frame.clear_color(3.0 / 255.0, 3.0 / 255.0, 3.0 / 255.0, 1.0);

    let blend = glium::draw_parameters::Blend::alpha_blending();
    let draw_parameters = glium::DrawParameters {
        blend,
        ..glium::DrawParameters::default()
    };

    // 쉐이더 프로그램 생성
    let program = glium::Program::from_source(display, VERTEX_SHADER_SRC, FRAGMENT_SHADER_SRC, None).unwrap();

    for sprite in sprites {
        let vertices = [
            Vertex { position: [sprite.position.0, sprite.position.1], tex_coords: [0.0, 1.0] },
            Vertex { position: [sprite.position.0 + sprite.size.0, sprite.position.1], tex_coords: [1.0, 1.0] },
            Vertex { position: [sprite.position.0 + sprite.size.0, sprite.position.1 + sprite.size.1], tex_coords: [1.0, 0.0] },
            Vertex { position: [sprite.position.0, sprite.position.1 + sprite.size.1], tex_coords: [0.0, 0.0] },
        ];

        let vertex_buffer = glium::VertexBuffer::new(display, &vertices).unwrap();
        let indices = glium::index::NoIndices(glium::index::PrimitiveType::TriangleFan);

        // 텍스처 유니폼
        let uniforms = uniform! {
            matrix: [
                [2.0 / display.get_framebuffer_dimensions().0 as f32, 0.0, 0.0, 0.0],
                [0.0, -2.0 / display.get_framebuffer_dimensions().1 as f32, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [-1.0, 1.0, 0.0, 1.0],
            ],
            tex: sprite.texture.sampled().magnify_filter(MagnifySamplerFilter::Nearest)
        };

        frame.draw(&vertex_buffer, &indices, &program, &uniforms, &draw_parameters).unwrap();
    }

    frame.finish().unwrap();
}
