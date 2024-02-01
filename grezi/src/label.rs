use std::sync::Arc;

use eframe::egui::{text_selection::LabelSelectionState, *};

/// Static text.
///
/// Usually it is more convenient to use [`Ui::label`].
///
/// ```
/// # egui::__run_test_ui(|ui| {
/// ui.label("Equivalent");
/// ui.add(egui::Label::new("Equivalent"));
/// ui.add(egui::Label::new("With Options").wrap(false));
/// ui.label(egui::RichText::new("With formatting").underline());
/// # });
/// ```
///
/// For full control of the text you can use [`crate::text::LayoutJob`]
/// as argument to [`Self::new`].
#[must_use = "You should put this widget in an ui with `ui.add(widget);`"]
pub struct Label {
    text: Arc<Galley>,
    sense: Option<Sense>,
    pos: Rect,
    opacity_factor: f32,
    selectable: Option<bool>,
}

impl Label {
    pub fn new(text: Arc<Galley>, pos: Rect, opacity_factor: f32) -> Self {
        Self {
            text,
            sense: None,
            pos,
            opacity_factor,
            selectable: None,
        }
    }

    pub fn text(&self) -> &str {
        self.text.text()
    }

    /// Make the label respond to clicks and/or drags.
    ///
    /// By default, a label is inert and does not respond to click or drags.
    /// By calling this you can turn the label into a button of sorts.
    /// This will also give the label the hover-effect of a button, but without the frame.
    ///
    /// ```
    /// # use egui::{Label, Sense};
    /// # egui::__run_test_ui(|ui| {
    /// if ui.add(Label::new("click me").sense(Sense::click())).clicked() {
    ///     /* … */
    /// }
    /// # });
    /// ```
    #[inline]
    pub fn sense(mut self, sense: Sense) -> Self {
        self.sense = Some(sense);
        self
    }

    /// Can the user select the text with the mouse?
    ///
    /// Overrides [`crate::style::Interaction::selectable_labels`].
    #[inline]
    pub fn selectable(mut self, selectable: bool) -> Self {
        self.selectable = Some(selectable);
        self
    }
}

impl Label {
    /// Do layout and position the galley in the ui, without painting it or adding widget info.
    pub fn layout_in_ui(self, ui: &mut Ui) -> (Pos2, Arc<Galley>, Response) {
        let selectable = self
            .selectable
            .unwrap_or_else(|| ui.style().interaction.selectable_labels);

        let mut sense = self.sense.unwrap_or_else(|| {
            if ui.memory(|mem| mem.options.screen_reader) {
                // We only want to focus labels if the screen reader is on.
                Sense::focusable_noninteractive()
            } else {
                Sense::hover()
            }
        });

        if selectable {
            // On touch screens (e.g. mobile in `eframe` web), should
            // dragging select text, or scroll the enclosing [`ScrollArea`] (if any)?
            // Since currently copying selected text in not supported on `eframe` web,
            // we prioritize touch-scrolling:
            let allow_drag_to_select = ui.input(|i| !i.any_touches());

            let select_sense = if allow_drag_to_select {
                Sense::click_and_drag()
            } else {
                Sense::click()
            };

            sense = sense.union(select_sense);
        }

        // If the user said "use this specific galley", then just use it:
        let response = ui.allocate_rect(self.pos, sense);
        let pos = match self.text.job.halign {
            Align::LEFT => self.pos.left_top(),
            Align::Center => self.pos.center_top(),
            Align::RIGHT => self.pos.right_top(),
        };
        return (pos, self.text, response);
    }
}

impl Widget for Label {
    fn ui(self, ui: &mut Ui) -> Response {
        let opacity_factor = self.opacity_factor;
        let selectable = self.selectable;
        let (pos, galley, mut response) = self.layout_in_ui(ui);
        response.widget_info(|| WidgetInfo::labeled(WidgetType::Label, galley.text()));

        if galley.elided {
            // Show the full (non-elided) text on hover:
            response = response.on_hover_text(galley.text());
        }

        if ui.is_rect_visible(response.rect) {
            let response_color = ui.style().interact(&response).text_color();

            let underline = if response.has_focus() || response.highlighted() {
                Stroke::new(1.0, response_color)
            } else {
                Stroke::NONE
            };

            ui.painter().add(
                epaint::TextShape::new(pos, Arc::clone(&galley), response_color)
                    .with_underline(underline)
                    .with_opacity_factor(opacity_factor),
            );
            let selectable = selectable.unwrap_or_else(|| ui.style().interaction.selectable_labels);
            if selectable {
                LabelSelectionState::label_text_selection(ui, &response, pos, &galley);
            }
        }

        response
    }
}
