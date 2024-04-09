use std::sync::Arc;

use bevy::prelude::*;
use bevy_egui::{
    egui::{self, Align2, Frame, Id, Ui},
    EguiContexts,
};

pub(crate) struct PopupPlugin;

impl Plugin for PopupPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PopupEvent>()
            .add_systems(PostUpdate, render);
    }
}

#[derive(Debug, Clone)]
pub enum PopupPosition {
    Center,
    BelowCenter,
}

#[derive(Event, Clone)]
pub struct PopupEvent {
    position: PopupPosition,
    time: f32,
    add_contents: Arc<dyn Fn(&mut Ui) + Send + Sync>,
}

impl PopupEvent {
    pub fn new(
        position: PopupPosition,
        time: f32,
        add_contents: impl Fn(&mut Ui) + Send + Sync + 'static,
    ) -> Self {
        Self {
            position,
            time,
            add_contents: Arc::new(add_contents),
        }
    }
}

#[derive(Default)]
struct RenderData {
    last: Option<PopupEvent>,
}

fn render(
    mut ctx: EguiContexts,
    mut events: EventReader<PopupEvent>,
    mut local: Local<RenderData>,
    time: Res<Time>,
) {
    if events.is_empty() {
        if let Some(last) = &mut local.last {
            render_element(&mut ctx, last);
            last.time -= time.delta_seconds();
            if last.time <= 0.0 {
                local.last = None;
            }
        }
    }

    let size = events.len();
    for (_, event) in events.read().enumerate().filter(|x| x.0 + 1 == size) {
        render_element(&mut ctx, event);
        local.last = Some(event.clone());
    }
}

fn render_element(ctx: &mut EguiContexts, event: &PopupEvent) {
    let mut area = egui::Area::new(Id::new("popup"));
    area = match event.position {
        PopupPosition::Center => area.anchor(Align2::CENTER_CENTER, egui::Vec2::ZERO),
        PopupPosition::BelowCenter => {
            area.anchor(Align2::CENTER_CENTER, egui::Vec2::new(0.0, 10.0))
        }
    };

    area.show(ctx.ctx_mut(), |ui| {
        let frame = Frame::popup(ui.style());
        frame.show(ui, |ui| (event.add_contents)(ui)).inner
    });
}
