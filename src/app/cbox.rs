use eframe::egui::{self, Response, Ui};

pub fn cbox(ui: &mut ui, variable: &mut T, label: &str)
where
    T: Enum,
{
    egui::ComboBox::from_label(label)
        .selected_text(format!("{:?}", variable))
        .show_ui(ui, |ui| {
            for i in T {
                ui.selectable_value(&mut self.account_fields.account_type, i, format!("{:?}", i));
            }
        });
}

// egui::ComboBox::from_label("Select one!")
//     .selected_text(format!("{:?}", self.account_fields.account_type))
//     .show_ui(ui, |ui| {
//         ui.selectable_value(
//             &mut self.account_fields.account_type,
//             account::AccountType::Account,
//             "Common Account",
//         );
//         ui.selectable_value(
//             &mut self.account_fields.account_type,
//             account::AccountType::Cash,
//             "Cash",
//         );
//         ui.selectable_value(
//             &mut self.account_fields.account_type,
//             account::AccountType::DebetCard,
//             "DebetCard",
//         );
//         ui.selectable_value(
//             &mut self.account_fields.account_type,
//             account::AccountType::CreditCard,
//             "CreditCard",
//         );
//         ui.selectable_value(
//             &mut self.account_fields.account_type,
//             account::AccountType::CreditAccount,
//             "CreditAccount",
//         );
//         ui.selectable_value(
//             &mut self.account_fields.account_type,
//             account::AccountType::AccumulativeAccount,
//             "AccumulativeAccount",
//         );
//         ui.selectable_value(
//             &mut self.account_fields.account_type,
//             account::AccountType::Deposit,
//             "Deposit",
//         );
//     });
