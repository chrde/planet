extern crate piston;
extern crate piston_window;
extern crate opengl_graphics;
#[macro_use]
extern crate gfx;
extern crate gfx_device_gl;
extern crate shader_version;
extern crate camera_controllers;
extern crate vecmath;
extern crate nalgebra;
extern crate rand;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use piston_window::*;
use opengl_graphics::{GlGraphics, OpenGL};
use shader_version::Shaders;
use shader_version::glsl::GLSL;
use gfx::traits::*;
use camera_controllers::*;

mod random;
mod glsl;
mod globe;
mod chunk;

pub struct App {
    gl: GlGraphics,
    t: f64,
}

impl App {
    fn render(&mut self, args: &RenderArgs) {

        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
        let random_number = 0.5;
        let color: [f32; 4] = [random_number, random_number, random_number, 1.0];

        let square = rectangle::square(0.0, 0.0, 50.0);
        let (x, y) = ((args.width / 2) as f64, (args.height / 2) as f64);

        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(GREEN, gl);

            let transform = c.transform.trans(x, y).trans(-25.0, -25.0);

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
    let random = random::RandomGenerator::new(12);
    let mut window: PistonWindow = WindowSettings::new("planet", [1200, 1200])
        .opengl(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut app = App {
        gl: GlGraphics::new(opengl),
        t: 0.0,
    };

    let mut events = Events::new(EventSettings::new());

    let globe = globe::Globe::new_example();

    let (vertex_data, index_data) = globe.make_geometry();

    let ref mut factory = window.factory.clone();

    let (vbuf, slice) = factory
        .create_vertex_buffer_with_slice(&vertex_data, index_data.as_slice());

    let texels = [[0xFF, 0xFF, 0x00, 0x00]];

    let (_, texture_view) = factory.create_texture_immutable::<gfx::format::Rgba8>(
        gfx::texture::Kind::D2(1, 1, gfx::texture::AaMode::Single),
        &[&texels]).unwrap();

    let get_projection = |w: &PistonWindow| {
        let draw_size = w.window.draw_size();
        CameraPerspective {
            fov: 90.0,
            near_clip: 0.1,
            far_clip: 1000.0,
            aspect_ratio: (draw_size.width as f32) / (draw_size.height as f32),
        }.projection()
    };

    let mut projection = get_projection(&window);
    let mut first_person = FirstPerson::new([0.5, 0.5, 4.0], FirstPersonSettings::keyboard_wasd());

    let sinfo = gfx::texture::SamplerInfo::new(gfx::texture::FilterMethod::Bilinear,
                                               gfx::texture::WrapMode::Clamp);

    let mut data = pipe::Data {
        vbuf: vbuf.clone(),
        u_model_view_proj: [[0.0; 4]; 4],
        t_color: (texture_view, factory.create_sampler(sinfo)),
        out_color: window.output_color.clone(),
        out_depth: window.output_stencil.clone(),
    };

    let glsl = opengl.to_glsl();
    let pso = factory
        .create_pipeline_simple(Shaders::new()
                                    .set(GLSL::V1_50, include_str!("glsl/shaders/cube_150.glslv"))
                                    .get(glsl)
                                    .unwrap()
                                    .as_bytes(),
                                Shaders::new()
                                    .set(GLSL::V1_50, include_str!("glsl/shaders/cube_150.glslf"))
                                    .get(glsl)
                                    .unwrap()
                                    .as_bytes(),
                                pipe::new())
        .unwrap();

    let model = vecmath::mat4_id();
    while let Some(e) = events.next(&mut window) {
        // if let Some(r) = e.render_args() {
        //     app.render(&r);
        // }
        // if let Some(u) = e.update_args() {
        //     app.update(&u);
        // }
        first_person.event(&e);
        if let Some(Button::Keyboard(key)) = e.press_args(){
            if key == Key::Up {
                println!("+");
            } else if key == Key::Down {
                println!("-");
            }
        }

        window.draw_3d(&e, |window| {
            let args = e.render_args().unwrap();

            window
                .encoder
                .clear(&window.output_color, [0.3, 0.3, 0.3, 1.0]);
            window.encoder.clear_depth(&window.output_stencil, 1.0);

            data.u_model_view_proj =
                model_view_projection(model,
                                      first_person.camera(args.ext_dt).orthogonal(),
                                      projection);
            window.encoder.draw(&slice, &pso, &data);
        });

        if let Some(_) = e.resize_args() {
            projection = get_projection(&window);
            data.out_color = window.output_color.clone();
            data.out_depth = window.output_stencil.clone();
        }
    }
}

gfx_vertex_struct!(_Vertex {
                       a_pos: [f32; 4] = "a_pos",
                       a_tex_coord: [i8; 2] = "a_tex_coord",
                       a_color: [f32; 3] = "a_color",
                   });

pub type Vertex = _Vertex;

impl Vertex {
    fn new(pos: [f32; 3], color: [f32; 3]) -> Vertex {
        Vertex {
            a_pos: [pos[0], pos[1], pos[2], 1.0],
            a_tex_coord: [0, 0],
            a_color: color,
        }
    }
}

gfx_pipeline!( pipe {
    vbuf: gfx::VertexBuffer<Vertex> = (),
    u_model_view_proj: gfx::Global<[[f32; 4]; 4]> = "u_model_view_proj",
    t_color: gfx::TextureSampler<[f32; 4]> = "t_color",
    out_color: gfx::RenderTarget<::gfx::format::Srgba8> = "o_color",
    out_depth: gfx::DepthTarget<::gfx::format::DepthStencil> =
        gfx::preset::depth::LESS_EQUAL_WRITE,
});
