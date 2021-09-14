
use std::collections::HashMap;
use legion::*;
use crate::cameras;
use crate::world;
use crate::rendering;
use crate::time;
use crate::debug;
use crate::graph;
use crate::mesh;

#[system(for_each)]
#[filter(!component::<rendering::RenderGraphSet>())]
pub fn build_world_graph_for_mesh(
    entity: &Entity, 
    mesh: &mesh::Mesh, 
    #[resource] graph: &mut rendering::WorldRenderGraph,
    buffer: &mut legion::systems::CommandBuffer,
) {
    let timed_block = debug::TimedBlock::start(debug::CycleCounter::BuildWorldGraphForMesh);
    graph.add_mesh(*entity, mesh.data.clone());
    buffer.add_component(*entity, rendering::RenderGraphSet::default());
    timed_block.stop();
}

#[system(for_each)]
pub fn set_editor_state_on_graph(
    editor: &mut debug::EditorState,
    editor_visible: Option<&debug::EditorVisible>,
    #[resource] graph: &mut rendering::EditorRenderGraph
) {
    let timed_block = debug::TimedBlock::start(debug::CycleCounter::SetEditorStateOnRenderer);
    graph.set_state(editor, editor_visible.is_some());    
    timed_block.stop();
}

#[system(for_each)]
#[filter(component::<debug::Debug>())]
#[filter(!component::<debug::EditorState>())]
pub fn set_editor_controls (
    entity: &legion::Entity,
    buffer: &mut legion::systems::CommandBuffer,
    #[resource] graph: &mut rendering::EditorRenderGraph,

) {
    let timed_block = debug::TimedBlock::start(debug::CycleCounter::SetEditorStateOnRenderer);
    
    graph.add_control(graph::create_main_sidebar());
    graph.add_control(graph::create_camera_window());
    graph.add_control(graph::create_measurements_window());
    graph.add_control(graph::create_entities_window());
    
    buffer.add_component(*entity, debug::EditorState::default());
    buffer.add_component(*entity, debug::EditorVisibility::default());
    timed_block.stop();
}

#[system(for_each)]
pub fn build_editor_render_graph_for_statistics(
    time: &time::ElapsedTime, 
    #[resource] graph: &mut rendering::EditorRenderGraph
) {
    let timed_block = debug::TimedBlock::start(debug::CycleCounter::BuildEditorRenderGraphForStatistics);
    graph.add_float_data(rendering::EditorRenderGraphDataItem::ElapsedTime, time.seconds * 1000.0);    
    timed_block.stop();
}

#[system(for_each)]
pub fn build_editor_render_graph_for_editor_state(
    editor: &debug::EditorState, 
    #[resource] graph: &mut rendering::EditorRenderGraph
) {
    let timed_block = debug::TimedBlock::start(debug::CycleCounter::BuildEditorRenderGraphForEditorState);
    graph.add_boolean_data(rendering::EditorRenderGraphDataItem::CameraWindowVisibiity, editor.is_window_visible(graph::CAMERA_WINDOW_NAME));
    graph.add_boolean_data(rendering::EditorRenderGraphDataItem::MeasurementWindowVisibiity, editor.is_window_visible(graph::MEASUREMENTS_WINDOW_NAME));    
    graph.add_boolean_data(rendering::EditorRenderGraphDataItem::EntityWindowVisibiity, editor.is_window_visible(graph::MEASUREMENTS_WINDOW_NAME));    
    timed_block.stop();
}

#[system(for_each)]
pub fn build_editor_render_graph_for_measurements(
    measurement: &debug::BlockPerformanceMeasurement, 
    #[resource] graph: &mut rendering::EditorRenderGraph
) {
    let timed_block = debug::TimedBlock::start(debug::CycleCounter::BuildEditorRenderGraphForMeasurements);

    let mut row = HashMap::default();
    row.insert(rendering::EditorRenderGraphDataItem::MeasurementName, rendering::EditorRenderGraphData::String { value: measurement.counter.to_string() });
    row.insert(rendering::EditorRenderGraphDataItem::CycleMeasurement, rendering::EditorRenderGraphData::Int { value: measurement.cycles });
    row.insert(rendering::EditorRenderGraphDataItem::CyclePercentage, rendering::EditorRenderGraphData::Float { value: (measurement.cycles as f32 / measurement.total_cycles as f32) * 100.0 });
    row.insert(rendering::EditorRenderGraphDataItem::HitMeasurement, rendering::EditorRenderGraphData::Int { value: measurement.hits });
    graph.add_row_data(rendering::EditorRenderGraphDataItem::MeasurementRow, row);
    
    timed_block.stop();
}

#[system(for_each)]
pub fn build_editor_render_graph_for_world_entities(
    world_entity_id: &world::WorldEntityId, 
    world_entity_selected: Option<&world::WorldEntitySelected>,
    #[resource] graph: &mut rendering::EditorRenderGraph
) {
    let timed_block = debug::TimedBlock::start(debug::CycleCounter::BuildEditorRenderGraphForWorldEntities);

    graph.add_list_item(
        rendering::EditorRenderGraphDataItem::EntityNode,
        world_entity_id.name.clone(),
        world_entity_selected.is_some()
    );
    
    timed_block.stop();
}

#[system(for_each)]
pub fn build_editor_render_graph_for_camera(
    camera: &cameras::Camera, 
    #[resource] graph: &mut rendering::EditorRenderGraph
) {
    let timed_block = debug::TimedBlock::start(debug::CycleCounter::BuildEditorRenderGraphForCamera);
    graph.add_vector4_data(rendering::EditorRenderGraphDataItem::CameraPosition, camera.position);
    graph.add_vector4_data(rendering::EditorRenderGraphDataItem::CameraDirection, camera.direction);
    graph.add_vector4_data(rendering::EditorRenderGraphDataItem::CameraUp, camera.up);  
    timed_block.stop();
}
