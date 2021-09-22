use crate::rendering;
use crate::debug;
use super::graph;

pub struct WorldRenderer {
}

impl WorldRenderer {
    pub fn new() -> Self {
        Self {}
    }

    pub fn render(&mut self, graph: &mut graph::WorldRenderGraph, target_dimensions: (u32, u32), display: &mut glium::Display, target: &mut glium::Frame) -> rendering::SubRendererResult {
        let timed_block = debug::start_timed_block(debug::CycleCounter::RenderWorld);
        graph.draw(target_dimensions, display, target);
        timed_block.stop();

        return rendering::SubRendererResult::None
    }
}