
use crate::prelude::*;

#[system(for_each)]
pub fn build_editor_render_graph_for_camera(camera: &Camera,  #[resource] graph: &mut EditorRenderGraph) {
    let timed_block = start_timed_block(CycleCounter::BuildEditorRenderGraphForCamera);
    graph.add_vector4_data(EditorRenderGraphDataItem::CameraPosition, camera.position);
    graph.add_vector4_data(EditorRenderGraphDataItem::CameraDirection, camera.direction);
    graph.add_vector4_data(EditorRenderGraphDataItem::CameraUp, camera.up);  
    timed_block.stop();
}
