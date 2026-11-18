use gpui::{
    App, ClipboardItem, Context, MouseButton, MouseUpEvent, Render, Window, div, prelude::*, px,
};

use crate::editor::Inlay;
use crate::theme::Theme;

#[derive(Clone, Debug)]
pub enum LineResult {
    None,
    /// display_text, full_precision_copy_text
    Value(String, String),
}

pub struct ResultsPane {
    results: Vec<LineResult>,
    inlays: Vec<Option<Inlay>>,
    pub copy_full_precision: bool,
    pub line_visual_counts: Vec<usize>,
    pub theme: Theme,
}

impl ResultsPane {
    pub fn new(theme: Theme, copy_full_precision: bool) -> Self {
        ResultsPane {
            results: Vec::new(),
            inlays: Vec::new(),
            copy_full_precision,
            line_visual_counts: Vec::new(),
            theme,
        }
    }

    pub fn set_results_with_diagnostics(
        &mut self,
        results: Vec<LineResult>,
        diagnostics: &[Option<Inlay>],
        cx: &mut Context<Self>,
    ) {
        self.inlays = diagnostics.to_vec();
        self.results = results;
        cx.notify();
    }
}

impl Render for ResultsPane {
    fn render(&mut self, window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let line_height = window.line_height();
        let theme = self.theme.clone();
        let results = self.results.clone();

        let inlays = self.inlays.clone();

        div()
            .flex()
            .flex_col()
            .w_full()
            .bg(theme.background)
            .pt(window.rem_size() * 0.75)  // Match editor top padding (font_size * 0.75)
            .children({
                let mut children: Vec<gpui::AnyElement> = Vec::new();
                for (i, result) in results.into_iter().enumerate() {
                    let (text, color) = match &result {
                        LineResult::None => (String::new(), theme.text_dimmed),
                        LineResult::Value(display, _) => (display.clone(), theme.result),
                    };

                    let text_for_copy = if self.copy_full_precision {
                        match &result {
                            LineResult::Value(_, full) => full.clone(),
                            _ => text.clone(),
                        }
                    } else {
                        text.clone()
                    };
                    let visual_count = self.line_visual_counts.get(i).copied().unwrap_or(1);
                    let row_height = line_height * visual_count as f32;
                    children.push(
                        div()
                            .h(row_height)
                            .w_full()
                            .flex()
                            .items_end()
                            .justify_end()
                            .px(px(8.))
                            .text_color(color)
                            .when(!text_for_copy.is_empty(), |el| {
                                el.cursor_pointer()
                                    .on_mouse_up(
                                        MouseButton::Left,
                                        move |_: &MouseUpEvent, _window: &mut Window, cx: &mut App| {
                                            if !text_for_copy.is_empty() {
                                                cx.write_to_clipboard(ClipboardItem::new_string(
                                                    text_for_copy.clone(),
                                                ));
                                            }
                                        },
                                    )
                            })
                            .child(text)
                            .into_any_element(),
                    );

                    // Spacer matching the editor's inlay row. A "---" subtotal is shown
                    // here, right-aligned with a rule above it.
                    if let Some(Some(inlay)) = inlays.get(i) {
                        let mut spacer = div()
                            .w_full()
                            .h(line_height * 0.8) // proportional diagnostic spacer
                            .flex()
                            .justify_end()
                            .px(px(8.));
                        if !inlay.is_error {
                            spacer = spacer.child(
                                div()
                                    .border_t_1()
                                    .border_color(theme.text_dimmed)
                                    .text_size(window.rem_size() * 0.75)
                                    .text_color(theme.result)
                                    .child(inlay.text.clone()),
                            );
                        }
                        children.push(spacer.into_any_element());
                    }
                }
                children
            })
    }
}
