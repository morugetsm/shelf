use super::*;
use chrono::prelude::*;

mod project {
  pub struct ProjectItem {
    idx: i32,
    manager_name: String,
    title: String,
    rdate: NaiveDateTime,
  }
  pub struct ProjectRow {
    idx: i32,
    manager: i32,
    manager_name: String,
    sdate: Option<NaiveDateTime>,
    edate: Option<NaiveDateTime>,
    title: String,
    content: Option<String>,
    rdate: NaiveDateTime,
    udate: Option<NaiveDateTime>
  }
}

mod post {
  pub struct PostItem {
    idx: i32,
    author_name: String,
    manager_name: Option<String>,
    status: i32,
    title: String,
    noticed: bool,
    rdate: NaiveDateTime,
  }
  pub struct PostRow {
    idx: i32,
    author: i32,
    author_name: String,
    manager: Option<i32>,
    manager_name: Option<String>,
    status: i32,
    progress: Option<i32>,
    sdate: Option<NaiveDateTime>,
    edate: Option<NaiveDateTime>,
    title: String,
    content: Option<String>,
    shared: bool,
    noticed: bool,
    rdate: NaiveDateTime,
    udate: Option<NaiveDateTime>,
  }
}

mod comment {
  pub struct Comment {
    idx: i32,
    author: i32,
    author_name: String,
    content: Option<String>,
    rdate: NaiveDateTime,
    udate: Option<NaiveDateTime>,
  }
}