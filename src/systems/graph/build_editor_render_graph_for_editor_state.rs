use crate::prelude::*;

#[system(for_each)]
pub fn build_editor_render_graph_for_editor_state(editor: &EditorState,  #[resource] graph: &mut EditorRenderGraph) {
    let timed_block = start_timed_block(CycleCounter::BuildEditorRenderGraphForEditorState);
    graph.add_boolean_data(EditorRenderGraphDataItem::CameraWindowVisibiity, editor.is_window_visible(CAMERA_WINDOW_NAME));
    graph.add_boolean_data(EditorRenderGraphDataItem::MeasurementWindowVisibiity, editor.is_window_visible(MEASUREMENTS_WINDOW_NAME));    
    graph.add_boolean_data(EditorRenderGraphDataItem::EntityWindowVisibiity, editor.is_window_visible(ENTITIES_WINDOW_NAME));    
    timed_block.stop();
}
