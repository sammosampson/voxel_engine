use crate::prelude::*;

#[system(for_each)]
#[filter(component::<Chunk>())]
pub fn build_editor_render_graph_for_chunks(
    chunk_position: &ChunkPosition,
    #[resource] graph: &mut EditorRenderGraph
) {
    let timed_block = start_timed_block(CycleCounter::BuildEditorRenderGraphForChunks);

    let mut row = HashMap::default();
    row.insert(EditorRenderGraphDataItem::ChunkX, EditorRenderGraphData::Int { value: chunk_position.x });
    row.insert(EditorRenderGraphDataItem::ChunkZ, EditorRenderGraphData::Int { value: chunk_position.z });
    graph.add_row_data(EditorRenderGraphDataItem::ChunkRow, row);
    
    timed_block.stop();
}