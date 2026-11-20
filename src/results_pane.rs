use gpui::{
    App, ClipboardItem, Context, MouseButton, MouseUpEvent, Render, Window, div, prelude::*, px,
};

use crate::theme::Theme;

#[derive(Clone, Debug)]
pub enum LineResult {
    None,
    /// display_text, full_precision_copy_text
    Value(String, String),
    /// "---" subtotal: dimmed, short rule above, shown as "subtotal: x"
    Subtotal(String, String),
    /// Line using `sum`: like Value, shown as "= x" with a rule above
    Sum(String, String),
}

pub struct ResultsPane {
    results: Vec<LineResult>,
    has_diagnostic: Vec<bool>,
    pub copy_full_precision: bool,
    pub line_visual_counts: Vec<usize>,
    pub theme: Theme,
}

impl ResultsPane {
    pub fn new(theme: Theme, copy_full_precision: bool) -> Self {
        ResultsPane {
            results: Vec::new(),
            has_diagnostic: Vec::new(),
            copy_full_precision,
            line_visual_counts: Vec::new(),
            theme,
        }
    }

    /// Result text per line as shown in the column ("" for lines without a result).
    pub fn display_texts(&self) -> Vec<String> {
        self.results.iter().map(|r| match r {
            LineResult::None => String::new(),
            LineResult::Value(display, _) => display.clone(),
            LineResult::Subtotal(display, _) => format!("subtotal: {display}"),
            LineResult::Sum(display, _) => format!("= {display}"),
        }).collect()
    }

    pub fn set_results_with_diagnostics(
        &mut self,
        results: Vec<LineResult>,
        diagnostics: &[Option<String>],
        cx: &mut Context<Self>,
    ) {
        self.has_diagnostic = diagnostics.iter().map(|d| d.is_some()).collect();
        self.results = results;
        cx.notify();
    }
}

impl Render for ResultsPane {
    fn render(&mut self, window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let line_height = window.line_height();
        let theme = self.theme.clone();
        let results = self.results.clone();

        let has_diag = self.has_diagnostic.clone();

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
                        LineResult::Subtotal(display, _) => (format!("subtotal: {display}"), theme.result.opacity(0.6)),
                        LineResult::Sum(display, _) => (format!("= {display}"), theme.result),
                    };
                    let is_subtotal = matches!(result, LineResult::Subtotal(..));
                    let is_sum = matches!(result, LineResult::Sum(..));

                    let text_for_copy = if self.copy_full_precision {
                        match &result {
                            LineResult::Value(_, full) | LineResult::Sum(_, full) | LineResult::Subtotal(_, full) => full.clone(),
                            _ => text.clone(),
                        }
                    } else {
                        match &result {
                            // Without the "= " and "subtotal: " labels
                            LineResult::Sum(display, _) | LineResult::Subtotal(display, _) => display.clone(),
                            _ => text.clone(),
                        }
                    };
                    let visual_count = self.line_visual_counts.get(i).copied().unwrap_or(1);
                    let row_height = line_height * visual_count as f32;
                    children.push(
                        div()
                            .relative()
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
                            // Sum: band and a rule across the whole row, matching the editor side
                            .when(is_sum, |el| {
                                el.bg(theme.result.opacity(0.05)).child(
                                    div().absolute().top_0().left_0().right_0().h(px(1.))
                                        .bg(theme.text_dimmed.opacity(0.5)),
                                )
                            })
                            .when(!is_subtotal, |el| el.child(text.clone()))
                            // Subtotal: short rule over the value only
                            .when(is_subtotal, |el| {
                                el.child(
                                    div()
                                        .border_t_1()
                                        .border_color(theme.text_dimmed.opacity(0.5))
                                        .child(text.clone()),
                                )
                            })
                            .into_any_element(),
                    );

                    // Insert empty spacer if this line has a diagnostic inlay
                    if has_diag.get(i).copied().unwrap_or(false) {
                        children.push(
                            div()
                                .w_full()
                                .h(line_height * 0.8) // proportional diagnostic spacer
                                .into_any_element(),
                        );
                    }
                }
                children
            })
    }
}
