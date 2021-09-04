use crate::events;
use crate::rendering;

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
        name: "Camera".to_string(),
        children: vec!(
            rendering::EditorRenderGraphNode::Row { children: vec!(position_label, position) },
            rendering::EditorRenderGraphNode::Row { children: vec!(direction_label, direction) }, 
            rendering::EditorRenderGraphNode::Row { children: vec!(up_label, up) } 
        )
    };

    rendering::EditorRenderGraphNode::Window {
        name: "Camera".to_string(),
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
        name: "Measurements".to_string(),
        children: vec!(time_row, measurement_rows)
    };
    
    let measurement_scroll = rendering::EditorRenderGraphNode::ScrollArea {
        id: "MeasurementsScroll".to_string(),
        children: vec!(measurement_grid)
    };
    
    rendering::EditorRenderGraphNode::Window { 
        name: "Measurements".to_string(),
        children: vec!(measurement_scroll)
    }
}