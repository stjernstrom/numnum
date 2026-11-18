use std::sync::Arc;

use gpui::{App, Context, MouseButton, MouseUpEvent, Render, Window, div, prelude::*, px, svg};

use crate::theme::Theme;

pub struct StatusBar {
    line: usize,
    col: usize,
    running_total: String,
    /// Sum of the selected lines, shown next to the total when set
    selected_total: Option<String>,
    /// Short message shown instead of the total for a moment (e.g. "Copied")
    flash: Option<String>,
    flash_epoch: usize,
    pub theme: Theme,
    on_settings_click: Option<Arc<dyn Fn(&mut Window, &mut App)>>,
    on_copy_click: Option<Arc<dyn Fn(&mut Window, &mut App)>>,
}

impl StatusBar {
    pub fn new(
        theme: Theme,
        on_settings_click: Option<Arc<dyn Fn(&mut Window, &mut App)>>,
        on_copy_click: Option<Arc<dyn Fn(&mut Window, &mut App)>>,
    ) -> Self {
        StatusBar {
            line: 1,
            col: 1,
            running_total: String::new(),
            selected_total: None,
            flash: None,
            flash_epoch: 0,
            theme,
            on_settings_click,
            on_copy_click,
        }
    }

    pub fn set_cursor(&mut self, line: usize, col: usize, cx: &mut Context<Self>) {
        self.line = line + 1; // 1-indexed display
        self.col = col + 1;
        cx.notify();
    }

    /// Show `message` in the centre of the status bar for 1.5 seconds.
    pub fn flash(&mut self, message: &str, cx: &mut Context<Self>) {
        self.flash = Some(message.to_string());
        self.flash_epoch += 1;
        let epoch = self.flash_epoch;
        cx.spawn(async move |this, cx| {
            cx.background_executor().timer(std::time::Duration::from_millis(1500)).await;
            if let Some(this) = this.upgrade() {
                this.update(cx, |bar, cx| {
                    if bar.flash_epoch == epoch {
                        bar.flash = None;
                        cx.notify();
                    }
                });
            }
        }).detach();
        cx.notify();
    }

    pub fn set_selected_total(&mut self, total: Option<String>, cx: &mut Context<Self>) {
        self.selected_total = total;
        cx.notify();
    }

    pub fn set_running_total(&mut self, total: String, cx: &mut Context<Self>) {
        self.running_total = total;
        cx.notify();
    }
}

impl Render for StatusBar {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let on_settings = self.on_settings_click.clone();
        let on_copy = self.on_copy_click.clone();
        let icon_color = self.theme.text_dimmed;

        div()
            .flex()
            .flex_row()
            .w_full()
            .h(px(28.))
            .flex_shrink_0()
            .bg(self.theme.background)
            .px(px(12.))
            .items_center()
            .text_size(px(12.))
            // Left: Settings gear icon
            .child(
                div()
                    .id("settings-btn")
                    .flex_shrink_0()
                    .cursor_pointer()
                    .flex()
                    .items_center()
                    .h_full()
                    .px(px(4.))
                    .child(
                        svg()
                            .path("icons/settings.svg")
                            .size(px(14.))
                            .text_color(icon_color)
                    )
                    .hover(|s| s.opacity(0.7))
                    .when_some(on_settings, |el, cb| {
                        el.on_mouse_up(
                            MouseButton::Left,
                            move |_: &MouseUpEvent, window: &mut Window, cx: &mut App| {
                                cb(window, cx);
                            },
                        )
                    }),
            )
            // Copy lines with results (all, or the selection)
            .child(
                div()
                    .id("copy-btn")
                    .flex_shrink_0()
                    .cursor_pointer()
                    .flex()
                    .items_center()
                    .h_full()
                    .px(px(4.))
                    .child(
                        svg()
                            .path("icons/copy.svg")
                            .size(px(14.))
                            .text_color(icon_color)
                    )
                    .hover(|s| s.opacity(0.7))
                    .when_some(on_copy, |el, cb| {
                        el.on_mouse_up(
                            MouseButton::Left,
                            move |_: &MouseUpEvent, window: &mut Window, cx: &mut App| {
                                cb(window, cx);
                            },
                        )
                    }),
            )
            // Center: total
            .child(
                div()
                    .flex_1()
                    .text_color(self.theme.text_muted)
                    .flex()
                    .justify_center()
                    .child(if let Some(message) = &self.flash {
                        message.clone()
                    } else {
                        let total = if self.running_total.is_empty() {
                            String::new()
                        } else {
                            format!("Sum total: {}", self.running_total)
                        };
                        match &self.selected_total {
                            Some(selected) => format!("{}    Selected rows: {}", total, selected).trim_start().to_string(),
                            None => total,
                        }
                    }),
            )
            // Right: Ln X, Col Y
            .child(
                div()
                    .flex_shrink_0()
                    .w(px(80.))
                    .flex()
                    .justify_end()
                    .text_color(self.theme.text_dimmed)
                    .child(format!("Ln {}, Col {}", self.line, self.col)),
            )
    }
}
