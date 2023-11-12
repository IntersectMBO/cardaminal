use comfy_table::Table;
use serde::Serialize;

use crate::utils::OutputFormatter;
use crate::wallet::dal::entities::transaction::Model as TransactionModel;

#[derive(Debug, Serialize)]
pub struct TransactionView {
    pub id: i32,
    pub status: String,
    pub annotation: Option<String>,
}

impl OutputFormatter for Vec<TransactionView> {
    fn to_table(&self) {
        let mut table = Table::new();

        table.set_header(vec!["id", "status", "annotation"]);

        for transaction in self {
            table.add_row(vec![
                &transaction.id.to_string(),
                &transaction.status,
                transaction.annotation.as_ref().unwrap_or(&String::new()),
            ]);
        }

        println!("{table}");
    }

    fn to_json(&self) {
        let json = serde_json::to_string_pretty(self).unwrap();
        println!("{json}");
    }
}

impl From<TransactionModel> for TransactionView {
    fn from(value: TransactionModel) -> Self {
        Self {
            id: value.id,
            status: value.status.to_string(),
            annotation: value.annotation,
        }
    }
}
