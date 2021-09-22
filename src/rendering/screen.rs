use glium::*;
use glium::glutin::event_loop::*;
use glium::glutin::window::*;
use glium::glutin::*;
use crate::events;
use crate::math;
use crate::debug;
use super::editor;
use super::world;

fn create_display(event_loop: &EventLoop<()>) -> Display {
    Display::new(
        WindowBuilder::new(),
        ContextBuilder::new().with_depth_buffer(24),
        event_loop)
    .unwrap()
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct WindowSize {
    pub size: math::Size2d
}

pub struct ScreenRenderer {
    display: Display,
    editor_renderer: editor::EditorRenderer,
    world_renderer: world::WorldRenderer
}

impl ScreenRenderer {
    pub fn new(event_loop: &EventLoop<()>) -> Self {
        let display = create_display(event_loop);
        let editor_renderer = editor::EditorRenderer::new(&display);
        let world_renderer = world::WorldRenderer::new();
        
        Self {
            display,
            editor_renderer,
            world_renderer
        }
    }
   

    pub fn process_event(&mut self, event: &glium::glutin::event::WindowEvent) {
        self.editor_renderer.process_event(event);
    }

    pub fn render(
        &mut self,
        editor_graph: &mut editor::EditorRenderGraph,
        world_graph: &mut world::WorldRenderGraph,
        event_producer: &mut events::SystemEventProducer
    ) {

        let timed_block = debug::start_timed_block(debug::CycleCounter::Render);

        let starting_timed_block = debug::start_timed_block(debug::CycleCounter::RenderStart);
        let mut target = self.display.draw();
        target.clear_color_and_depth((0.0, 0.0, 1.0, 1.0), 1.0);
        let target_dimensions = target.get_dimensions();
        let mut results = vec!();
        starting_timed_block.stop();

        results.push(self.world_renderer.render(world_graph, target_dimensions, &mut self.display, &mut target));
        results.push(self.editor_renderer.render(editor_graph, event_producer, &self.display, &mut target));

        let stopping_timed_block = debug::start_timed_block(debug::CycleCounter::RenderEnd);
        target.finish().unwrap();
        stopping_timed_block.stop();

        if results.contains(&super::SubRendererResult::Quit) {
            event_producer.push(events::SystemEvent::CloseRequested);
        }

        if results.contains(&super::SubRendererResult::NeedsRepaint) {
            self.display.gl_window().window().request_redraw();
        }
    
        timed_block.stop();
    }
}