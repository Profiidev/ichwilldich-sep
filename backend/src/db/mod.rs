use centaurus::db::init::Connection;

use crate::db::vacation::VacationTable;

pub mod vacation;

pub trait DBTrait {
  fn vacation(&self) -> VacationTable<'_>;
}

impl DBTrait for Connection {
  fn vacation(&self) -> VacationTable<'_> {
    VacationTable::new(self)
  }
}
