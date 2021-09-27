mod calculate_elapsed_time;
mod process_window_events;
mod orientation;
mod graph;
mod camera;
mod lighting;
mod input;
mod debug;
mod terrain;
mod hero;
mod apply_heading_and_velocity_to_position;
mod world;
mod set_render_style;

use crate::prelude::*;

pub fn build_world() -> World {
    let mut world = World::default();
    debug::add_debugging_to_world(&mut world);
    lighting::add_lighting_to_world(&mut world);
    hero::add_hero_to_world(&mut world);
    world
}

pub fn build_schedule() -> Schedule {
    Schedule::builder()
        .add_thread_local(graph::set_editor_controls_system())
        .add_system(hero::spawn_hero_system())
        .add_system(process_window_events::process_window_events_system())
        .add_thread_local(calculate_elapsed_time::calculate_elapsed_time_system(Instant::now()))
        .add_system(input::mouse_input_system())
        .add_system(input::editor_visibility_from_input_system())
        .add_system(world::world_entity_selection_from_input_system())
        .add_system(input::editor_state_from_input_system())
        .add_thread_local(terrain::reveal_terrain_system())
        .flush()
        .add_system(terrain::position_chunks_system())
        .add_system(terrain::tesselate_chunk_front_faces_system())
        .add_system(terrain::tesselate_chunk_back_faces_system())
        .add_system(terrain::tesselate_chunk_top_faces_system())
        .add_system(terrain::tesselate_chunk_left_faces_system())
        .add_system(terrain::tesselate_chunk_right_faces_system())
        .flush()
        .add_system(world::world_entity_selection_effects_entities_system())
        .add_thread_local(terrain::merge_chunk_mesh_system())
        .add_thread_local(graph::build_world_graph_for_mesh_system())
        .flush()
        .add_thread_local(graph::build_editor_render_graph_for_editor_state_system())
        .add_thread_local(graph::build_editor_render_graph_for_statistics_system())
        .add_thread_local(graph::build_editor_render_graph_for_camera_system())
        .add_thread_local(graph::build_editor_render_graph_for_measurements_system())
        .add_thread_local(graph::build_editor_render_graph_for_world_entities_system())
        .add_system(orientation::constant_rotation_system())
        .add_system(camera::move_camera_from_editor_system())
        .add_system(hero::move_hero_from_mouse_action_system())
        .add_system(camera::move_editor_camera_from_mouse_action_system())
        .add_system(camera::follow_with_attached_camera_system())
        .add_system(apply_heading_and_velocity_to_position::apply_heading_and_velocity_to_position_system())
        .add_thread_local(world::set_world_visibility_system())
        .add_thread_local(set_render_style::set_render_style_system())
        .add_thread_local(orientation::set_world_node_rotation_system())
        .add_thread_local(orientation::set_world_node_position_system())
        .add_thread_local(camera::set_camera_to_render_view_matrix_system())
        .add_thread_local(lighting::set_render_lighting_system())
        .add_thread_local(graph::set_editor_state_on_graph_system())
        .add_system(debug::aggregate_statistics_system())
        .build()
}