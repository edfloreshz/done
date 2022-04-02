use std::cell::RefCell;
use std::rc::Rc;
use std::sync::{Arc, Mutex};

use tokio::sync::mpsc::{Receiver, Sender};

use crate::models::list::List;
use crate::services::microsoft::task::Task;

pub mod handler;

#[derive(Debug)]
pub enum UiEvent {
    Fetch,
    Login,
    Uri(String),
    AddListEntry(String),
    AddTaskEntry(String, String),
    // AddList(String),
    ListSelected(usize),
    TaskCompleted(String, String, bool),
    TaskSelected(String, String),
}

#[derive(Debug)]
pub enum DataEvent {
    Login,
    UpdateTasks(String, Vec<Task>),
    UpdateLists(Vec<List>),
    UpdateDetails(String, Box<Task>),
}

#[derive(Clone)]
pub struct EventHandler {
    pub ui_tx: Rc<RefCell<Sender<UiEvent>>>,
    pub ui_rv: Arc<Mutex<Receiver<UiEvent>>>,
    pub data_tx: Arc<Mutex<Sender<DataEvent>>>,
    pub data_rv: Rc<RefCell<Option<Receiver<DataEvent>>>>,
}

impl EventHandler {
    pub fn new(
        ui: (Sender<UiEvent>, Receiver<UiEvent>),
        data: (Sender<DataEvent>, Receiver<DataEvent>),
    ) -> Self {
        Self {
            ui_tx: Rc::new(RefCell::new(ui.0)),
            ui_rv: Arc::new(Mutex::new(ui.1)),
            data_tx: Arc::new(Mutex::new(data.0)),
            data_rv: Rc::new(RefCell::new(Some(data.1))),
        }
    }
}
