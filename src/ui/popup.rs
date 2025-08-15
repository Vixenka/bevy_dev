/*!
 * All things related to popups.
 *
 * Example view:
 *
 * ![Popup](https://raw.githubusercontent.com/Vixenka/bevy_dev/master/images/debug_camera/switching.webp)
 * ![Popup](https://raw.githubusercontent.com/Vixenka/bevy_dev/master/images/debug_camera/switching_without_preview.webp)
 */

use std::sync::{Arc, Mutex};

use bevy::prelude::*;
use bevy_egui::{
    EguiContext, EguiPrimaryContextPass, PrimaryEguiContext,
    egui::{self, Align2, Frame, Id, Ui},
};

use crate::{prelude::DebugCameraActive, ui::UiContextPass};

static STORAGE: Mutex<Option<PopupEvent>> = Mutex::new(None);

pub(crate) struct PopupPlugin;

impl Plugin for PopupPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PopupEvent>()
            .add_systems(
                PostUpdate,
                write_storage.before(render_primary).before(render_debug),
            )
            .add_systems(EguiPrimaryContextPass, render_primary)
            .add_systems(UiContextPass, render_debug);
    }
}

/// Position of the popup.
#[derive(Debug, Clone, Default)]
pub enum PopupPosition {
    #[default]
    Center,
    BelowCenter,
}

/// Event for showing a popup.
#[derive(Event, Clone)]
pub struct PopupEvent {
    position: PopupPosition,
    duration: f32,
    add_contents: Arc<dyn Fn(&mut Ui) + Send + Sync>,
    is_static: bool,
}

impl PopupEvent {
    /// Creates a new popup event with the given position, time and UI contents.
    /// # Remarks
    /// Handly create value should be sent to the world via [`EventWriter<PopupEvent>`] to show it on the screen,
    /// otherwise [`super::popup`] can be used to show it without the events.
    pub fn new(
        position: PopupPosition,
        duration: f32,
        add_contents: impl Fn(&mut Ui) + Send + Sync + 'static,
    ) -> Self {
        Self {
            position,
            duration,
            add_contents: Arc::new(add_contents),
            is_static: false,
        }
    }

    /// Changes a duration of display.
    pub fn time(mut self, duration: f32) -> Self {
        self.duration = duration;
        self.fetch_if_needed()
    }

    /// Changes a position of pupup.
    pub fn position(mut self, position: PopupPosition) -> Self {
        self.position = position;
        self.fetch_if_needed()
    }

    pub(crate) fn fetch(mut self) -> Self {
        self.is_static = true;
        STORAGE
            .lock()
            .expect("unable to get popup storage")
            .replace(self.clone());
        self
    }

    fn fetch_if_needed(self) -> Self {
        match self.is_static {
            true => self.fetch(),
            false => self,
        }
    }
}

#[derive(Default)]
struct RenderData {
    last: Option<PopupEvent>,
}

fn write_storage(mut events: EventWriter<PopupEvent>) {
    if let Some(event) = STORAGE.lock().expect("unable to get popup storage").take() {
        events.write(event);
    }
}

fn get_last_popup(events: &mut EventReader<PopupEvent>) -> Option<PopupEvent> {
    let mut last = None;
    for event in events.read() {
        last = Some(event);
    }
    last.cloned()
}

fn render_primary(
    mut ctx: Single<(&mut EguiContext, &Camera), With<PrimaryEguiContext>>,
    mut events: EventReader<PopupEvent>,
    mut local: Local<RenderData>,
    time: Res<Time>,
) {
    if ctx.1.is_active {
        render(
            &[ctx.0.get_mut()],
            get_last_popup(&mut events),
            &mut local,
            &time,
        );
    }
}

fn render_debug(
    mut ctx: Single<&mut EguiContext, With<DebugCameraActive>>,
    mut events: EventReader<PopupEvent>,
    mut local: Local<RenderData>,
    time: Res<Time>,
) {
    render(
        &[ctx.get_mut()],
        get_last_popup(&mut events),
        &mut local,
        &time,
    );
}

fn render(
    ctx: &[&egui::Context],
    event: Option<PopupEvent>,
    local: &mut Local<RenderData>,
    time: &Res<Time>,
) {
    match event {
        Some(event) => {
            for ctx in ctx {
                render_element(ctx, &event);
            }
            local.last = Some(event);
        }
        None => {
            if let Some(last) = &mut local.last {
                for ctx in ctx {
                    render_element(ctx, last);
                }
                last.duration -= time.delta_secs();
                if last.duration <= 0.0 {
                    local.last = None;
                }
            }
        }
    }
}

fn render_element(ctx: &egui::Context, event: &PopupEvent) {
    let mut area = egui::Area::new(Id::new("popup")).movable(false);
    area = match event.position {
        PopupPosition::Center => area.anchor(Align2::CENTER_CENTER, egui::Vec2::ZERO),
        PopupPosition::BelowCenter => {
            area.anchor(Align2::CENTER_CENTER, egui::Vec2::new(0.0, 10.0))
        }
    };

    area.show(ctx, |ui| {
        let frame = Frame::popup(ui.style());
        frame.show(ui, |ui| (event.add_contents)(ui)).inner
    });
}
