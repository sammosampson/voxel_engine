
use crate::prelude::*;

#[system(for_each)]
#[filter(component::<Debug>())]
#[filter(!component::<EditorState>())]
pub fn set_editor_controls (entity: &Entity, buffer: &mut CommandBuffer, #[resource] graph: &mut EditorRenderGraph) {
    let timed_block = start_timed_block(CycleCounter::SetEditorStateOnRenderer);
    
    graph.add_control(create_main_sidebar());
    graph.add_control(create_camera_window());
    graph.add_control(create_measurements_window());
    graph.add_control(create_entities_window());
    graph.add_control(create_chunks_window());
    
    buffer.add_component(*entity, EditorState::default());
    timed_block.stop();
}
