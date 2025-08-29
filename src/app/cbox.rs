use eframe::egui::{self, Ui};
use std::fmt::Debug;
use strum::IntoEnumIterator;

// trait Enum: Debug {}

pub fn cbox<T>(ui: &mut Ui, variable: &mut T, label: &str)
where
    T: IntoEnumIterator + Debug + PartialEq,
{
    egui::ComboBox::from_label(label)
        .selected_text(format!("{:?}", variable))
        .show_ui(ui, |ui| {
            for i in T::iter() {
                let text = format!("{:?}", i);
                ui.selectable_value(variable, i, text);
            }
        });
}
