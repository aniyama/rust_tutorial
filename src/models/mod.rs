use core::panic;

use chrono::{Datelike, NaiveDate};
use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum IncomeCategpry {
    CU,
    FI,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum ExpenseCategory {
    Rent,
    Other,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum Category {
    Income(IncomeCategpry),
    Expense(ExpenseCategory),
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct Item {
    name: String,
    category: Category,
    price: u32,
    date: NaiveDate,
}

impl Item {
    pub fn new(name: String, category: Category, price: u32, date: NaiveDate) -> Self {
        Item {
            name,
            category,
            price,
            date,
        }
    }
    pub fn get_category(register_type: u8, category_type: u8) -> Category {
        if register_type == 0 {
            match category_type {
                0 => Category::Income(IncomeCategpry::CU),
                1 => Category::Income(IncomeCategpry::FI),
                _ => panic!("不正な入力値です"),
            }
        } else {
            match category_type {
                0 => Category::Expense(ExpenseCategory::Rent),
                1 => Category::Expense(ExpenseCategory::Other),
                _ => panic!("不正な入力値です"),
            }
        }
    }

    pub fn get_year(&self) -> i32 {
        self.date.year()
    }

    pub fn get_month(&self) -> u32 {
        self.date.month()
    }

    pub fn get_first_day(&self) -> NaiveDate {
        NaiveDate::from_ymd(self.get_year(), self.get_month(), 1)
    }

    pub fn get_price_summary(&self) -> i32 {
        match self.category {
            Category::Income(_) => self.price as i32,
            Category::Expense(_) => -1 * self.price as i32,
        }
    }
}
