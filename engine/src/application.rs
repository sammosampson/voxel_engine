use std::time::Instant;
use legion::*;
use voxel_engine_app::events;
use voxel_engine_app::rendering;
use crate::systems;

pub struct Application {
    pub world: World, 
    pub resources: Resources,
    schedule: Schedule
}

impl Into<voxel_engine_app::HotLoadableApplicationState> for Application {
    fn into(self) -> voxel_engine_app::HotLoadableApplicationState {
        voxel_engine_app::HotLoadableApplicationState::new(self.world, self.resources, self.schedule)
    }
}

impl Application {
    pub fn build() -> Self {
        let world = build_world();
        let resources = build_resources();
        let schedule = build_schedule();
       
        Self {
            world,
            resources, 
            schedule
        }
    }

    pub fn reload(state: &mut voxel_engine_app::HotLoadableApplicationState) {
        let resources = build_resources();
        let schedule = build_schedule();
        state.reload(resources, schedule);
    }
}

fn build_world() -> World {
    let mut world = World::default();
    systems::add_debugging_to_world(&mut world);
    systems::add_cameras_to_world(&mut world);
    systems::add_lighting_to_world(&mut world);
    systems::add_terrain_to_world(&mut world);
    world
}

fn build_resources() -> Resources {
    let exit_state_notifier = voxel_engine_app::create_exit_state_notifier();
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
        .add_system(systems::process_window_events_system())
        .add_thread_local(systems::calculate_elapsed_time_system(Instant::now()))
        .add_system(systems::mouse_input_system())
        .add_system(systems::editor_state_from_input_system())
        .add_system(systems::position_chunks_system())
        .add_system(systems::tesselate_chunk_front_faces_system())
        .add_system(systems::tesselate_chunk_back_faces_system())
        .add_system(systems::tesselate_chunk_top_faces_system())
        .add_system(systems::tesselate_chunk_bottom_faces_system())
        .add_system(systems::tesselate_chunk_left_faces_system())
        .add_system(systems::tesselate_chunk_right_faces_system())
        .flush()
        .add_thread_local(systems::merge_chunk_mesh_system())
        .flush()
        .add_thread_local(systems::build_editor_render_graph_for_statistics_system())
        .add_thread_local(systems::build_editor_render_graph_for_camera_system())
        .add_thread_local(systems::build_editor_render_graph_for_measurements_system())
        .add_system(systems::constant_rotation_system())
        .add_system(systems::move_camera_from_editor_system())
        .add_system(systems::move_camera_from_mouse_input_system())
        .add_thread_local(systems::set_world_node_orientation_system())
        .add_thread_local(systems::set_camera_to_render_view_matrix_system())
        .add_thread_local(systems::set_render_lighting_system())
        .add_thread_local(systems::set_editor_state_on_graph_system())
        .add_system(systems::aggregate_statistics_system())
        .build()
}