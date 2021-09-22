use std::collections::HashMap;
use glium::glutin::event::WindowEvent;
use crate::rendering::*;
use crate::events;
use crate::debug;
use super::graph;

pub struct EditorRenderer {
    egui: egui_glium::EguiGlium
}

impl EditorRenderer {
    pub fn new(display: &glium::Display) -> Self {
        Self {
            egui: egui_glium::EguiGlium::new(&display)
        }
    }

    pub fn process_event(&mut self, event: &WindowEvent) {
        self.egui.on_event(event);
    }

    pub fn render(
        &mut self,
        graph: &mut graph::EditorRenderGraph,
        event_producer: &mut events::SystemEventProducer,
        display: &glium::Display,
        target: &mut glium::Frame
    ) -> SubRendererResult {        
        if !graph.editor_visible() {
            return SubRendererResult::None;
        }

        let timed_block = debug::start_timed_block(debug::CycleCounter::RenderEditor);

        self.egui.begin_frame(display);
        let mut visuals = egui::Visuals::dark();
        visuals.widgets.noninteractive.bg_fill = egui::Color32::from_rgba_premultiplied(0, 0, 0, 220);
        self.egui.ctx().set_visuals(visuals);
        
        for node in graph.controls() {
            self.render_topmost(graph, &node, event_producer)
        }

        graph.clear_data();

        let (needs_repaint, shapes) = self.egui.end_frame(&display);
        self.egui.paint(&display, target, shapes);

        timed_block.stop();

        if needs_repaint {
            return SubRendererResult::NeedsRepaint;
        }
        return SubRendererResult::None;
    }
    
    fn render_topmost(
        &self,
        graph: &graph::EditorRenderGraph,
        node: &graph::EditorRenderGraphNode,
        event_producer: &mut events::SystemEventProducer) {
        match node {
            graph::EditorRenderGraphNode::SideBar { name, children } => {
                egui::SidePanel::left(name)
                    .resizable(false)
                    .show(self.egui.ctx(), |ui| {
                        for node in children {
                            self.render_node(ui, &graph.data(), node, event_producer);
                        }
                    });
            },
            graph::EditorRenderGraphNode::Window { name, children } => {
                let window = egui::Window::new(name)
                    .resizable(false);
                if !graph.is_window_visible(name) {
                    return;
                }
                window
                    .show(self.egui.ctx(), |ui| {
                        for node in children {
                            self.render_node(ui, &graph.data(), node, event_producer);
                        }
                    });
            },
            _ => {}
        }
    }

    fn render_node(
        &self,
        ui: &mut egui::Ui,
        data: &HashMap<graph::EditorRenderGraphDataItem, graph::EditorRenderGraphData>,
        node: &graph::EditorRenderGraphNode,
        event_producer: &mut events::SystemEventProducer) {
        match node {
            graph::EditorRenderGraphNode::ScrollArea { id, children } => {
                egui::ScrollArea::auto_sized()
                    .id_source(id)
                    .show(ui, |ui| {
                        for group_child_node in children {
                            self.render_node(ui, data, group_child_node, event_producer);
                        }    
                    });
            },
            graph::EditorRenderGraphNode::Grid { name, children } => {
                egui::Grid::new(name)
                    .spacing([40.0, 4.0])
                    .min_col_width(100.0)
                    .striped(true)
                    .show(ui, |ui| {
                        for group_child_node in children {
                            self.render_node(ui, data, group_child_node, event_producer);
                        }    
                    });
            },
            graph::EditorRenderGraphNode::Rows { children, titles, item } => {
                for title in titles {
                    ui.centered_and_justified(|ui| ui.label(title.to_string()));
                }
                ui.end_row();

                let data_item = data.get(item).unwrap();
                
                match data_item {
                    graph::EditorRenderGraphData::Cells { data } => {
                        for data_item in data {
                            for cell in children {
                                self.render_data(ui, data_item, cell, event_producer);
                            }
                            ui.end_row();
                        }                                  
                    },
                    _ => {}
                }
            },
            graph::EditorRenderGraphNode::Tree { item, item_click_handler } => {
                let data_item = data.get(item).unwrap();
                match data_item {
                    graph::EditorRenderGraphData::Nodes { data } => {
                        let mut sorted = data.clone();
                        sorted.sort();
                        for list_item in sorted {
                            let response = egui::CollapsingHeader
                                ::new(list_item.0.clone())
                                .default_open(false)
                                .selectable(true)
                                .selected(list_item.1)
                                .show(ui, |_ui| {});
                            if response.header_response.clicked() {
                                event_producer.push(events::SystemEvent::EditorChange(item_click_handler(list_item.0.clone())));
                            }
                        }                                  
                    },
                    _ => {}
                }
            },
            
            graph::EditorRenderGraphNode::Row { children} => {
                for cell in children {
                    self.render_data(ui, data, cell, event_producer);
                }
                ui.end_row();
            }
            _ => self.render_data(ui, data, node, event_producer)
        }
    }

    fn render_data(
        &self,
        ui: &mut egui::Ui,
        data: &HashMap<graph::EditorRenderGraphDataItem, graph::EditorRenderGraphData>,
        node: &graph::EditorRenderGraphNode,
        event_producer: &mut events::SystemEventProducer) {
        match node {
            graph::EditorRenderGraphNode::Toggle { item, click_handler } => {
                if let Some(data_item) = data.get(item) {
                    match data_item {
                        graph::EditorRenderGraphData::Boolean { mut value } => {            
                            if self.render_editable_bool(ui, &item.to_string(), &mut value) {
                                event_producer.push(events::SystemEvent::EditorChange(click_handler(value)));
                            }
                        },
                        _ => {}
                    }
                }
            },
            graph::EditorRenderGraphNode::Label { item } => {
                ui.label(item.to_string());
            },
            graph::EditorRenderGraphNode::Vector { item, change_handler } => {
                if let Some(data_item) = data.get(item) {
                    match data_item {
                        graph::EditorRenderGraphData::Vector4 { mut value } => {
                            ui.horizontal(|ui| {
                                if self.render_editable_float(ui, "x:", &mut value.x)
                                    || self.render_editable_float(ui, "y:", &mut value.y)
                                    || self.render_editable_float(ui, "z:", &mut value.z)
                                    || self.render_editable_float(ui, "w:", &mut value.w
                                ) {
                                    event_producer.push(events::SystemEvent::EditorChange(change_handler(value)));
                                }
                            });
                        },
                        _ => {}
                    }
                }
            },
            graph::EditorRenderGraphNode::Text { item } => {
                if let Some(data_item) = data.get(item) {
                    match data_item {
                        graph::EditorRenderGraphData::String { value } => {
                            ui.label(value);
                        }
                        _ => {}
                    }
                }
            },
            graph::EditorRenderGraphNode::Numeric { item } => {
                if let Some(data_item) = data.get(item) {
                    match data_item {
                        graph::EditorRenderGraphData::Float { mut value } => {
                            self.render_float(ui, &mut value);
                        },
                        graph::EditorRenderGraphData::Int { mut value } => {
                            self.render_int(ui, &mut value);
                        },
                        _ => {}
                    }
                }
            },
            _ => {}
        }
    }

    fn render_editable_float(&self, ui: &mut egui::Ui, label: &str, value: &mut f32) -> bool {
        ui.label(label);
        ui.add(egui::DragValue::new(value)).changed()
    }
    
    fn render_editable_bool(&self, ui: &mut egui::Ui, label: &str, value: &mut bool) -> bool {
        ui.add(egui::Checkbox::new(value, label)).changed()
    }

    fn render_float(&self, ui: &mut egui::Ui, value: &mut f32) {
        ui.with_layout(egui::Layout::top_down_justified(egui::Align::Center), |ui| ui.label(format!("{:.2}", value)));
    }

    fn render_int(&self, ui: &mut egui::Ui, value: &mut u64) {
        ui.with_layout(egui::Layout::top_down_justified(egui::Align::Center), |ui| ui.label(value.to_string()));
    }
}