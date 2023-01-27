use rand::{random, thread_rng, Rng};
use std::f32::consts::PI;
use std::iter::once;

use wgpu::{Color, CommandEncoder, PresentMode, RenderPipeline, TextureView};
use wgpu_noboiler::app::{AppCreator, AppData};
use wgpu_shapes::shape::shapes::BasicShape;

use wgpu_shapes::shape_renderer::ShapeRenderer;

struct State {
    shape_renderer: Option<ShapeRenderer>,
    animals: Vec<Animal>,
}

struct Animal {
    pos: (f32, f32),
    dir: f32,
}

fn main() {
    AppCreator::new(State {
        shape_renderer: None,
        animals: vec![],
    })
    .render(render)
    .init(init)
    .update(update)
    .resizable(false)
    .title("Display simple shapes")
    .present_mode(PresentMode::Immediate)
    .run();
}

fn update(app_data: &AppData, state: &mut State) {
    let mut rng = thread_rng();

    for animal in state.animals.iter_mut() {
        animal.dir += rng.gen_range(-25.0..25.0) * app_data.delta_time as f32;
        animal.dir %= PI * 2.0;

        animal.pos.0 += animal.dir.cos() * 100.0 * app_data.delta_time as f32;
        animal.pos.1 += animal.dir.sin() * 100.0 * app_data.delta_time as f32;
    }

    println!(
        "{} : {} | {}",
        app_data.update_time, app_data.render_time, app_data.fps
    );
}

fn render(
    data: &AppData,
    state: &mut State,
    mut encoder: CommandEncoder,
    texture_view: TextureView,
) {
    let shape_renderer = state.shape_renderer.as_mut().unwrap();

    shape_renderer.clear();

    for animal in &state.animals {
        shape_renderer.oval().pos(animal.pos.0, animal.pos.1);
    }

    shape_renderer.render(&mut encoder, &texture_view, &data.device);

    data.queue.submit(once(encoder.finish()));
}

fn init(data: &AppData, state: &mut State, _: &mut Vec<RenderPipeline>) {
    state.shape_renderer = Some(ShapeRenderer::new(&data.device, &data.config));
    state
        .shape_renderer
        .as_mut()
        .unwrap()
        .background_color(Color::GREEN);

    let mut rng = thread_rng();
    for _ in 0..10 {
        state.animals.push(Animal {
            pos: (0.0, 0.0),
            dir: rng.gen_range(0.0..(PI * 2.0)),
        });
    }
}
