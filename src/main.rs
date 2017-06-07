extern crate piston;
extern crate piston_window;
extern crate opengl_graphics;
#[macro_use]
extern crate gfx;
extern crate gfx_device_gl;
extern crate shader_version;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use piston_window::*;
use opengl_graphics::{GlGraphics, OpenGL};
use shader_version::Shaders;
use shader_version::glsl::GLSL;
use gfx::traots::*;

mod random;
mod glsl;

pub struct App {
    gl: GlGraphics,
    t: f64,
    rnd: random::RandomGenerator,
}

impl App {
    fn render(&mut self, args: &RenderArgs){

        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
        let random_number = self.rnd.next(self.t) as f32;
        let color:   [f32; 4] = [random_number, random_number, random_number, 1.0];

        let square = rectangle::square(0.0, 0.0, 50.0);
        let (x, y) = ((args.width / 2) as f64,
                      (args.height / 2) as f64);

        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(GREEN, gl);

            let transform = c.transform.trans(x, y)
                                       .trans(-25.0, -25.0);

            // Draw a box rotating around the middle of the screen.
            rectangle(color, square, transform, gl);
        });
    }

    fn update(&mut self, args: &UpdateArgs) {
        self.t += 2.0 * args.dt;
    }
}

fn main() {
    let opengl = OpenGL::V3_2;
    let random= random::RandomGenerator::new();
    let mut window: PistonWindow = WindowSettings::new("planet", [200, 200])
    .opengl(opengl)
    .exit_on_esc(true)
    .build()
    .unwrap();

    let mut app = App {
        gl: GlGraphics::new(opengl),
        t: 0.0,
        rnd: random,
    };

    let mut events = Events::new(EventSettings::new());

    let vertex_data: Vec<Vertex> = glsl::icosahedron::VERTICES.iter()
    .map(|v| {
        let (x, y, z) = (v[0] as f32, v[1] as f32, v[2] as f32);
        Vertex::new([x, y, z])}
        ).collect();

    let index_data: Vec<u16> = glsl::icosahedron::TRIANGLE_LIST.iter()
    .flat_map(|t| vec![t[0], t[1], t[2]])
    .map(|v| v as u16)
    .collect();

    let ref mut factory = window.factory.clone();

    let (vbuf, slice) = factory.create_vertex_buffer_with_slice(&vertex_data, index_data);

    let texels = [[0xFF, 0xFF, 0x00, 0x00]];

    let (_, texture_view) = factory.create_texture_immutable::<gfx::format::Rgba8>(
        gfx::texture::Kind::D2(1, 1, gfx::texture::AaMode::Single),
        &[&texels]).unwrap();

    let sinfo = gfx::texture::SamplerInfo::new(
        gfx::texture::FilterMethod::Bilinear,
        gfx::texture::WrapMode::Clamp);

    let glsl = opengl.to_glsl();
    let pso = factory.create_pipeline_simple(
            Shaders::new()
                .set(GLSL::V1_50, include_str!("glsl/shaders/cube_150.glslv"))
                .get(glsl).unwrap().as_bytes(),
            Shaders::new()
                .set(GLSL::V1_50, include_str!("glsl/shaders/cube_150.glslf"))
                .get(glsl).unwrap().as_bytes(),
            pipe::new()
        ).unwrap();

    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            app.render(&r);
        }
        if let Some(u) = e.update_args() {
            app.update(&u);
        }
    }
}

gfx_vertex_struct!( Vertex {
    a_pos: [f32; 4] = "a_pos",
    a_tex_coord: [i8; 2] = "a_tex_coord",
});

impl Vertex {
    fn new(pos: [f32; 3]) -> Vertex {
        Vertex {
            a_pos: [pos[0], pos[1], pos[2], 1.0],
            a_tex_coord: [0, 0],
        }
    }
}

gfx_pipeline!( pipe {
    vbuf: gfx::VertexBuffer<Vertex> = (),
    u_model_view_proj: gfx::Global<[[f32; 4]; 4]> = "u_model_view_proj",
    t_color: gfx::TextureSampler<[f32; 4]> = "t_color",
    out_color: gfx::RenderTarget<::gfx::format::Srgba8> = "o_Color",
    out_depth: gfx::DepthTarget<::gfx::format::DepthStencil> =
        gfx::preset::depth::LESS_EQUAL_WRITE,
});