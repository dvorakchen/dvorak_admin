pub mod consts;

use serde::{Deserialize, Serialize};
use std::fmt::{self, Display};

/// representing the user information
#[derive(Serialize, Deserialize, Clone)]
pub struct User {
    pub id: String,
    pub username: String,
}

impl User {
    pub fn to_json(&self) -> String {
        serde_json::to_string(self).expect("serialize User fail")
    }
}

pub enum UserError {
    NotExist,
}

impl fmt::Display for UserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UserError::NotExist => write!(f, "user not exist"),
        }
    }
}

pub type MenuList = Vec<Menu>;

#[derive(Serialize, Deserialize, Clone)]
pub struct Menu {
    pub id: usize,
    pub title: String,
    pub icon: String,
    pub sub_menu: Vec<SubMenu>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct SubMenu {
    pub id: usize,
    pub title: String,
    pub link: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub enum LeaveType {
    Personal,
    Sick,
    Annual,
}

impl Display for LeaveType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LeaveType::Annual => write!(f, "Annual"),
            LeaveType::Sick => write!(f, "Sick"),
            LeaveType::Personal => write!(f, "Personal"),
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct LeaveRequest {
    pub id: usize,
    pub user: User,
    pub leave_type: LeaveType,
    pub start_date: String,
    pub end_date: String,
    pub remark: String,
}

pub type LeaveList = Vec<LeaveRequest>;
