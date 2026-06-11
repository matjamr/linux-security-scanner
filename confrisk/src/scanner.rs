/// Wspolny interfejs skanerow ekosystemow

use crate::model::Finding;

pub trait Scanner {
    /// Uruchamia wszystkie kontrole i zwraca surowe wyniki (scoring nakladany pozniej)
    fn scan(&self) -> Vec<Finding>;
}
