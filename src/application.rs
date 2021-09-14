use std::time::Instant;
use legion::*;
use crate::debug;
use crate::events;
use crate::rendering;
use crate::systems;

pub struct Application {
    pub world: World, 
    pub resources: Resources,
    schedule: Schedule,
    event_loop: events::SystemEventLoop,
    screen_renderer: rendering::ScreenRenderer
}

impl Application {
    pub fn build() -> Self {
        debug::initialise_debugging();
        let event_loop = events::create_system_event_loop();
        let screen_renderer = rendering::ScreenRenderer::new(&event_loop.get_loop());

        let world = build_world();
        let resources = build_resources();
        let schedule = build_schedule();
       
        Self {
            world,
            resources, 
            schedule,
            event_loop,
            screen_renderer
        }
    }

    pub fn run(&mut self) {
        loop {
            if self.should_exit() {
                return;
            }
    
            self.run_loop();
        }
    }

    fn run_loop(&mut self) {
        self.process_events();
        self.execute_schedule();        
        self.render();
    }

    fn process_events(&mut self) {
        let mut event_producer = &mut self.resources.get_mut::<events::SystemEventProducer>().unwrap();
        let mut event_channel = &mut self.resources.get_mut::<shrev::EventChannel::<events::SystemEvent>>().unwrap();
        self.event_loop.run(&mut event_producer, &mut event_channel, &mut self.screen_renderer);
    }

    fn execute_schedule(&mut self) {
        &mut self.schedule.execute(&mut self.world, &mut self.resources);
    }

    fn render(&mut self) {
        let mut event_producer = &mut self.resources.get_mut::<events::SystemEventProducer>().unwrap();
        let mut world_graph = &mut self.resources.get_mut::<rendering::WorldRenderGraph>().unwrap();
        let mut editor_graph = &mut self.resources.get_mut::<rendering::EditorRenderGraph>().unwrap();
        self.screen_renderer.render(&mut editor_graph, &mut world_graph, &mut event_producer);
    }
    
    pub fn should_exit(&mut self) -> bool {
        ExitStateNotifier::should_exit(&mut self.resources)
    }
}

fn build_world() -> World {
    let mut world = World::default();
    systems::add_debugging_to_world(&mut world);
    systems::add_lighting_to_world(&mut world);
    systems::add_terrain_to_world(&mut world);
    systems::add_hero_to_world(&mut world);
    world
}

fn build_resources() -> Resources {
    let exit_state_notifier = create_exit_state_notifier();
    let system_event_producer = events::create_system_event_producer();
    let mut system_event_channel = events::create_system_event_channel();
    let event_channel_registrar = events::create_system_event_channel_registrar(&mut system_event_channel);
    let editor_render_graph = rendering::EditorRenderGraph::new();
    let world_render_graph = rendering::WorldRenderGraph::new();
        
    let mut resources = Resources::default();
    &mut resources.insert(exit_state_notifier);
    &mut resources.insert(system_event_producer);
    &mut resources.insert(system_event_channel);
    &mut resources.insert(event_channel_registrar);
    &mut resources.insert(editor_render_graph);
    &mut resources.insert(world_render_graph);
    resources
}

fn build_schedule() -> Schedule {
    Schedule::builder()
        .add_thread_local(systems::set_editor_controls_system())
        .add_system(systems::spawn_hero_system())
        .add_system(systems::process_window_events_system())
        .add_thread_local(systems::calculate_elapsed_time_system(Instant::now()))
        .add_system(systems::mouse_input_system())
        .add_system(systems::editor_visibility_from_input_system())
        .add_system(systems::world_entity_selection_from_input_system())
        .add_system(systems::editor_state_from_input_system())
        .add_system(systems::position_chunks_system())
        .add_system(systems::tesselate_chunk_front_faces_system())
        .add_system(systems::tesselate_chunk_back_faces_system())
        .add_system(systems::tesselate_chunk_top_faces_system())
        .add_system(systems::tesselate_chunk_left_faces_system())
        .add_system(systems::tesselate_chunk_right_faces_system())
        .flush()
        .add_thread_local(systems::merge_chunk_mesh_system())
        .add_thread_local(systems::build_world_graph_for_mesh_system())
        .flush()
        .add_thread_local(systems::build_editor_render_graph_for_editor_state_system())
        .add_thread_local(systems::build_editor_render_graph_for_statistics_system())
        .add_thread_local(systems::build_editor_render_graph_for_camera_system())
        .add_thread_local(systems::build_editor_render_graph_for_measurements_system())
        .add_thread_local(systems::build_editor_render_graph_for_world_entities_system())
        .add_system(systems::constant_rotation_system())
        .add_system(systems::move_camera_from_editor_system())
        .add_system(systems::move_hero_from_mouse_action_system())
        .add_system(systems::move_editor_camera_from_mouse_action_system())
        .add_system(systems::follow_with_attached_camera_system())
        .add_system(systems::apply_heading_and_velocity_to_position_system())
        .add_thread_local(systems::set_world_node_rotation_system())
        .add_thread_local(systems::set_world_node_position_system())
        .add_thread_local(systems::set_camera_to_render_view_matrix_system())
        .add_thread_local(systems::set_render_lighting_system())
        .add_thread_local(systems::set_editor_state_on_graph_system())
        .add_system(systems::aggregate_statistics_system())
        .build()
}

fn create_exit_state_notifier() -> ExitStateNotifier {
    ExitStateNotifier::default()
}

#[derive(Default)]
pub struct ExitStateNotifier {
    pub should_exit: bool
}

impl ExitStateNotifier {
    fn should_exit(resources: &mut legion::Resources) -> bool {
        let notifier = resources
            .get::<ExitStateNotifier>()
            .unwrap();
    
        notifier.should_exit
    }
}