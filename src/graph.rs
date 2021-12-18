use crate::events;
use crate::rendering;

const MAIN: &str = "Main";
pub const CAMERA_WINDOW_NAME: &str = "Camera";
pub const MEASUREMENTS_WINDOW_NAME: &str = "Measurements";
pub const ENTITIES_WINDOW_NAME: &str = "Entities";
pub const CHUNKS_WINDOW_NAME: &str = "Chunks";
const MEASUREMENTS_SCROLL: &str = "MeasurementsScroll";
const ENTITIES_SCROLL: &str = "EntitiesScroll";
const CHUNKS_SCROLL: &str = "ChunksScroll";

pub fn create_main_sidebar() -> rendering::EditorRenderGraphNode {
    let elapsed_time = rendering::EditorRenderGraphNode::Numeric { 
        item: rendering::EditorRenderGraphDataItem::ElapsedTime
    };

    let camera_toggle = rendering::EditorRenderGraphNode::Toggle {
        item: rendering::EditorRenderGraphDataItem::CameraWindowVisibiity,
        click_handler: Box::new(| visible | events::EditorEvent::SetWindowVisibility(visible, CAMERA_WINDOW_NAME.to_string()))

    };
    let measurements_toggle = rendering::EditorRenderGraphNode::Toggle {
        item: rendering::EditorRenderGraphDataItem::MeasurementWindowVisibiity,
        click_handler: Box::new(| visible | events::EditorEvent::SetWindowVisibility(visible, MEASUREMENTS_WINDOW_NAME.to_string()))

    };
    let entities_toggle = rendering::EditorRenderGraphNode::Toggle {
        item: rendering::EditorRenderGraphDataItem::EntityWindowVisibiity,
        click_handler: Box::new(| visible | events::EditorEvent::SetWindowVisibility(visible, ENTITIES_WINDOW_NAME.to_string()))

    };
    let chunks_toggle = rendering::EditorRenderGraphNode::Toggle {
        item: rendering::EditorRenderGraphDataItem::ChunksWindowVisibiity,
        click_handler: Box::new(| visible | events::EditorEvent::SetWindowVisibility(visible, CHUNKS_WINDOW_NAME.to_string()))

    };
    rendering::EditorRenderGraphNode::SideBar {
        name: MAIN.to_string(),
        children: vec!(elapsed_time, camera_toggle, measurements_toggle, entities_toggle, chunks_toggle)
    }
}

pub fn create_camera_window() -> rendering::EditorRenderGraphNode {
    let position_label = rendering::EditorRenderGraphNode::Label { 
        item: rendering::EditorRenderGraphDataItem::CameraPosition
    };
    let position = rendering::EditorRenderGraphNode::Vector { 
        item: rendering::EditorRenderGraphDataItem::CameraPosition,
        change_handler: Box::new(| position | events::EditorEvent::CameraPositionChanged(position))
    };

    let direction_label = rendering::EditorRenderGraphNode::Label { 
        item: rendering::EditorRenderGraphDataItem::CameraDirection
    };
    let direction = rendering::EditorRenderGraphNode::Vector { 
        item: rendering::EditorRenderGraphDataItem::CameraDirection,
        change_handler: Box::new(| direction | events::EditorEvent::CameraDirectionChanged(direction))
    };

    let up_label = rendering::EditorRenderGraphNode::Label { 
        item: rendering::EditorRenderGraphDataItem::CameraUp
    };
    let up = rendering::EditorRenderGraphNode::Vector { 
        item: rendering::EditorRenderGraphDataItem::CameraUp,
        change_handler: Box::new(| up | events::EditorEvent::CameraUpChanged(up))
    };

    let camera_grid = rendering::EditorRenderGraphNode::Grid { 
        name: CAMERA_WINDOW_NAME.to_string(),
        children: vec!(
            rendering::EditorRenderGraphNode::Row { children: vec!(position_label, position) },
            rendering::EditorRenderGraphNode::Row { children: vec!(direction_label, direction) }, 
            rendering::EditorRenderGraphNode::Row { children: vec!(up_label, up) } 
        )
    };

    rendering::EditorRenderGraphNode::Window {
        name: CAMERA_WINDOW_NAME.to_string(),
        children: vec!(camera_grid)
    }
}

pub fn create_measurements_window() -> rendering::EditorRenderGraphNode {    
    let elapsed_time_label = rendering::EditorRenderGraphNode::Label { 
        item: rendering::EditorRenderGraphDataItem::ElapsedTime
    };
    let elapsed_time = rendering::EditorRenderGraphNode::Numeric { 
        item: rendering::EditorRenderGraphDataItem::ElapsedTime
    };
    let time_row = rendering::EditorRenderGraphNode::Row { 
        children: vec!(elapsed_time_label, elapsed_time)
    };
    let measure_name = rendering::EditorRenderGraphNode::Text { 
        item: rendering::EditorRenderGraphDataItem::MeasurementName
    };
    let cycles = rendering::EditorRenderGraphNode::Numeric { 
        item: rendering::EditorRenderGraphDataItem::CycleMeasurement
    };
    let cycles_percentage = rendering::EditorRenderGraphNode::Numeric { 
        item: rendering::EditorRenderGraphDataItem::CyclePercentage
    };
    let hits = rendering::EditorRenderGraphNode::Numeric { 
        item: rendering::EditorRenderGraphDataItem::HitMeasurement
    };
    let measurement_rows = rendering::EditorRenderGraphNode::Rows { 
        item: rendering::EditorRenderGraphDataItem::MeasurementRow,
        titles: vec!(
            rendering::EditorRenderGraphDataItem::MeasurementName,
            rendering::EditorRenderGraphDataItem::CycleMeasurement,
            rendering::EditorRenderGraphDataItem::HitMeasurement,
            rendering::EditorRenderGraphDataItem::CyclePercentage
        ),
        children: vec!(measure_name, cycles, hits, cycles_percentage)
    };
    
    let measurement_grid = rendering::EditorRenderGraphNode::Grid { 
        name: MEASUREMENTS_WINDOW_NAME.to_string(),
        children: vec!(time_row, measurement_rows)
    };
    
    let measurement_scroll = rendering::EditorRenderGraphNode::ScrollArea {
        id: MEASUREMENTS_SCROLL.to_string(),
        children: vec!(measurement_grid)
    };
    
    rendering::EditorRenderGraphNode::Window { 
        name: MEASUREMENTS_WINDOW_NAME.to_string(),
        children: vec!(measurement_scroll)
    }
}

pub fn create_entities_window() -> rendering::EditorRenderGraphNode {    
    let entities_tree = rendering::EditorRenderGraphNode::Tree { 
        item: rendering::EditorRenderGraphDataItem::EntityNode,
        item_click_handler: Box::new(| id | events::EditorEvent::EntityNodeSelect(id))
    };
    
    let entities_scroll = rendering::EditorRenderGraphNode::ScrollArea {
        id: ENTITIES_SCROLL.to_string(),
        children: vec!(entities_tree)
    };
    
    rendering::EditorRenderGraphNode::Window { 
        name: ENTITIES_WINDOW_NAME.to_string(),
        children: vec!(entities_scroll)
    }
}

pub fn create_chunks_window() -> rendering::EditorRenderGraphNode {    
    let chunk_x = rendering::EditorRenderGraphNode::Numeric { 
        item: rendering::EditorRenderGraphDataItem::ChunkX
    };

    let chunk_y = rendering::EditorRenderGraphNode::Numeric { 
        item: rendering::EditorRenderGraphDataItem::ChunkZ
    };

    let chunks_rows = rendering::EditorRenderGraphNode::Rows { 
        item: rendering::EditorRenderGraphDataItem::ChunkRow,
        titles: vec!(
            rendering::EditorRenderGraphDataItem::ChunkX,
            rendering::EditorRenderGraphDataItem::ChunkZ,
        ),
        children: vec!(chunk_x, chunk_y)
    };
    
    let chunks_grid = rendering::EditorRenderGraphNode::Grid { 
        name: CHUNKS_WINDOW_NAME.to_string(),
        children: vec!(chunks_rows)
    };
    
    let chunks_scroll = rendering::EditorRenderGraphNode::ScrollArea {
        id: CHUNKS_SCROLL.to_string(),
        children: vec!(chunks_grid)
    };
    
    rendering::EditorRenderGraphNode::Window { 
        name: CHUNKS_WINDOW_NAME.to_string(),
        children: vec!(chunks_scroll)
    }
}