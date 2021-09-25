use crate::prelude::*;

#[system(for_each)]
pub fn set_editor_state_on_graph(
    editor: &mut EditorState,
    editor_visible: Option<&EditorVisible>,
    #[resource] graph: &mut EditorRenderGraph
) {
    let timed_block = start_timed_block(CycleCounter::SetEditorStateOnRenderer);
    graph.set_state(editor, editor_visible.is_some());    
    timed_block.stop();
}