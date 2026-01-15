use std::{
    env::current_exe,
    process::{Command, Stdio},
};

use gpui::{
    App, AppContext, Context, Entity, IntoElement, ParentElement, Render, Styled, Window, div,
};
use gpui_component::{
    Disableable, DivInspector, StyledExt,
    button::{Button, ButtonGroup},
    label::Label,
};

use crate::{cli::daemon_stop, macros::ui::MacroEntryUI};

/// Struct for if the clicker is activated or not.
pub struct Activate {
    is_clicking: bool,
}
impl Activate {
    pub fn view(window: &mut Window, cx: &mut App) -> Entity<Self> {
        cx.new(|cx| Self::new(window, cx))
    }
    pub fn new(_: &mut Window, _: &mut Context<Self>) -> Self {
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
                    |view, clicks: &Vec<usize>, _: &mut Window, _: &mut Context<Self>| {
                        if clicks.contains(&0) {
                            println!("Start clicking");
                            view.is_clicking = true;
                            // Command::new(program);
                            let myself = current_exe().unwrap();
                            let _ = Command::new(myself)
                                .arg("start")
                                .stdout(Stdio::inherit())
                                .spawn();
                        }
                        if clicks.contains(&1) {
                            println!("Stop clicking");
                            view.is_clicking = false;
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
            .v_flex()
            .items_center()
            .justify_center()
            .child(self.inspector.clone())
            .child(Label::new("WayClicker").text_3xl().font_bold())
            .child(self.activate.clone())
            .child(self.macros.clone())
    }
}
