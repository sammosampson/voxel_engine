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

fn main() {
    application::Application::build().run();
}