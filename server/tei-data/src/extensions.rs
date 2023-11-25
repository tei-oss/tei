use std::fmt;

use tokio_postgres::{row::RowIndex, Row};

pub trait FromRow {
    fn from_row(row: &tokio_postgres::Row) -> Self;
}

pub trait RowEx {
    fn get_boxed_str<I>(&self, idx: I) -> Box<str>
    where
        I: RowIndex + fmt::Display;

    fn get_opt_boxed_str<I>(&self, idx: I) -> Option<Box<str>>
    where
        I: RowIndex + fmt::Display;
}

impl RowEx for Row {
    fn get_boxed_str<I>(&self, idx: I) -> Box<str>
    where
        I: RowIndex + fmt::Display,
    {
        let value: String = self.get(idx);
        value.into_boxed_str()
    }

    fn get_opt_boxed_str<I>(&self, idx: I) -> Option<Box<str>>
    where
        I: RowIndex + fmt::Display,
    {
        let value: Option<String> = self.get(idx);
        value.map(|v| v.into_boxed_str())
    }
}
