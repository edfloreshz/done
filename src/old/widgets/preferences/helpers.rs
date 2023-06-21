use anyhow::Result;
use libset::{format::FileFormat, project::Project};
use relm4::{adw, AsyncComponentSender};

use crate::widgets::preferences::messages::PreferencesComponentOutput;

use super::model::{ColorScheme, Preferences, PreferencesComponentModel};

