mod application;
mod systems;
mod terrain;
mod graph;
mod rendering;
mod events;
mod debug;
mod math;
mod time;
mod mesh;
mod input;
mod cameras;
mod lighting;
mod position;
mod rotation;
mod hero;
mod physics;

fn main() {
    application::Application::build().run();
}
