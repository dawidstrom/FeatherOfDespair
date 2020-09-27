use crate::utils::{Position};

pub trait Clickable {
    fn is_clicked(&self, position_clicked: &Position) -> bool;
}
