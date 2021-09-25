
use crate::prelude::*;

#[system(for_each)]
pub fn build_editor_render_graph_for_world_entities(
    world_entity_id: &WorldEntityId, 
    world_entity_selected: Option<&WorldEntitySelected>,
    #[resource] graph: &mut EditorRenderGraph
) {
    let timed_block = start_timed_block(CycleCounter::BuildEditorRenderGraphForWorldEntities);

    graph.add_list_item(
        EditorRenderGraphDataItem::EntityNode,
        world_entity_id.name.clone(),
        world_entity_selected.is_some()
    );
    
    timed_block.stop();
}