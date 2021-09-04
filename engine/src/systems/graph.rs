
use std::collections::HashMap;
use legion::*;
use voxel_engine_app::cameras;
use voxel_engine_app::rendering;
use voxel_engine_app::time;
use voxel_engine_app::debug;
use crate::graph;

#[system(for_each)]
pub fn set_editor_state_on_graph(
    editor: &mut debug::Editor,
    #[resource] graph: &mut rendering::EditorRenderGraph
) {
    let timed_block = debug::TimedBlock::start(debug::CycleCounter::SetEditorStateOnRenderer);
    graph.set_state(editor);    
    timed_block.stop();
}

#[system(for_each)]
#[filter(component::<debug::Debug>())]
#[filter(!component::<debug::Editor>())]
pub fn set_editor_controls (
    entity: &legion::Entity,
    buffer: &mut legion::systems::CommandBuffer,
    #[resource] graph: &mut rendering::EditorRenderGraph,

) {
    let timed_block = debug::TimedBlock::start(debug::CycleCounter::SetEditorStateOnRenderer);
    
    graph.add_control(graph::create_camera_window());
    graph.add_control(graph::create_measurements_window());
    
    buffer.add_component(*entity, debug::Editor::default());
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