use super::*;
use chrono::prelude::*;

mod project {
  pub struct ProjectItem {
    id: i32,
    manager_name: String,
    title: String,
    reg_date: NaiveDateTime,
  }
  pub struct ProjectRow {
    id: i32,
    manager: i32,
    manager_name: String,
    title: String,
    start_date: Option<NaiveDateTime>,
    end_date: Option<NaiveDateTime>,
    description: String,
    reg_date: NaiveDateTime,
    mod_date: Option<NaiveDateTime>
  }
}

mod post {
  pub struct PostItem {
    id: i32,
    author_name: String,
    manager_name: Option<String>,
    status: i32,
    title: String,
    noticed: bool,
    reg_date: NaiveDateTime,
  }
  pub struct PostRow {
    id: i32,
    author: i32,
    author_name: String,
    subject: String,
    manager: Option<i32>,
    manager_name: Option<String>,
    status: i8,
    start_date: Option<NaiveDateTime>,
    end_date: Option<NaiveDateTime>,
    content: String,
    shared: bool,
    noticed: bool,
    reg_date: NaiveDateTime,
    mod_date: Option<NaiveDateTime>,
  }
}

mod comment {
  pub struct Comment {
    id: i32,
    author: i32,
    author_name: String,
    content: String,
    reg_date: NaiveDateTime,
    mod_date: Option<NaiveDateTime>,
  }
}