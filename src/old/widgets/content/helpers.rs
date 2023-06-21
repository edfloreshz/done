use anyhow::Result;
use chrono::{DateTime, Utc};
use core_done::models::{status::Status, task::Task};
use core_done::service::Service;
use relm4::ComponentController;

use crate::factories::task::model::TaskInit;
use crate::widgets::sidebar::model::SidebarList;
use crate::widgets::task_input::messages::TaskInputInput;

use super::widget::{ContentModel, ContentState};

