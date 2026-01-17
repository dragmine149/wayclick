use crate::{
    cli::{daemon_stop, pid_file_path},
    macros::ui::MacroEntryUI,
};
use gpui::{
    App, AppContext, Context, Entity, IntoElement, ParentElement, Render, Styled, Window, div,
};
use gpui_component::{
    Disableable, DivInspector, StyledExt,
    button::{Button, ButtonGroup},
    label::Label,
};
use notify::{Event, EventKind, RecursiveMode, Result, Watcher};
use std::{
    env::current_exe,
    path::PathBuf,
    process::{Command, Stdio},
};

/// Watch a specific file for changes, then send an update.
///
/// # Arguments
/// - cx: The relation to the entity.
/// - file: The file to watch
/// - callback: Function to call when an event has been triggered.
///
/// # Notes
/// We actually watch at the parent directory and only send events it we detect a file.
pub fn watch_fs<T: 'static>(
    cx: &mut Context<T>,
    file: PathBuf,
    callback: impl Fn(&mut T, &mut Context<T>, EventKind) + 'static,
) {
    let (tx, rx) = smol::channel::bounded(100);
    let mut watcher = notify::recommended_watcher(move |res: Result<Event>| {
        if let Ok(event) = &res {
            tx.send_blocking((event.kind, event.paths.clone()))
                .expect("Failed to send event details");
        }
    })
    .expect("Failed to initial watcher");

    cx.spawn(async move |this, cx| {
        watcher
            .watch(&file.parent().unwrap(), RecursiveMode::NonRecursive)
            .expect("Failed to watch file");

        while let Ok((event_kind, event_paths)) = rx.recv().await {
            // if it's not a valid path, no need to continue.
            if !event_paths.contains(&file) {
                continue;
            }

            this.update(cx, |this, cx| {
                callback(this, cx, event_kind);
            })
            .unwrap();
        }
    })
    .detach();
}

/// Struct for if the clicker is activated or not.
pub struct Activate {
    is_clicking: bool,
}

impl Activate {
    pub fn view(window: &mut Window, cx: &mut App) -> Entity<Self> {
        cx.new(|cx| Self::new(window, cx))
    }
    pub fn new(_: &mut Window, cx: &mut Context<Self>) -> Self {
        // change if we are clicking or not based on the pid file.
        watch_fs(cx, pid_file_path(), |this, cx, kind| {
            match kind {
                EventKind::Create(_) => this.is_clicking = true,
                EventKind::Remove(_) => this.is_clicking = false,
                _ => {}
            };
            cx.notify();
        });

        Self { is_clicking: false }
    }
}
impl Render for Activate {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div().size_full().child(
            ButtonGroup::new("activation-group")
                .children(vec![
                    Button::new("activate")
                        .label("Start Autoclicking")
                        .disabled(self.is_clicking),
                    Button::new("deactivate")
                        .label("Stop Autoclicking")
                        .disabled(!self.is_clicking),
                ])
                .on_click(cx.listener(
                    |_, clicks: &Vec<usize>, _: &mut Window, _: &mut Context<Self>| {
                        if clicks.contains(&0) {
                            println!("Start clicking");
                            let myself = current_exe().unwrap();
                            Command::new(myself)
                                .arg("start")
                                .stdout(Stdio::inherit())
                                .spawn()
                                .unwrap();
                        }
                        if clicks.contains(&1) {
                            println!("Stop clicking");
                            daemon_stop();
                        }
                    },
                )),
        )
    }
}

// Custom click UI to manage every other UI element
pub struct ClickUI {
    inspector: Entity<DivInspector>,
    activate: Entity<Activate>,

    macros: Entity<MacroEntryUI>,
}

impl ClickUI {
    pub fn view(window: &mut Window, cx: &mut App) -> Entity<Self> {
        cx.new(|cx| Self::new(window, cx))
    }

    fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let inspector = cx.new(|cx| DivInspector::new(window, cx));
        let activate = Activate::view(window, cx);

        let macros = MacroEntryUI::view(window, cx);

        Self {
            inspector,
            activate,
            macros,
        }
    }
}

impl Render for ClickUI {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        div()
            .p_1()
            .v_flex()
            .items_center()
            .justify_center()
            .child(self.inspector.clone())
            .child(self.activate.clone())
            .child(self.macros.clone())
    }
}
