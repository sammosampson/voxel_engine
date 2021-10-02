use crate::prelude::*;

#[system(for_each)]
pub fn set_editor_state_on_graph(
    editor: &mut EditorState,
    #[resource] state: &GameState,
    #[resource] graph: &mut EditorRenderGraph
) {
    let timed_block = start_timed_block(CycleCounter::SetEditorStateOnRenderer);
    graph.set_state(editor, *state == GameState::Editing);    
    timed_block.stop();
}