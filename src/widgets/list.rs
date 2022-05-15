use relm4::{Sender, view};
use relm4::factory::{DynamicIndex, FactoryComponent};

use crate::gtk;
use crate::gtk::prelude::{
    BoxExt,
    OrientableExt,
    WidgetExt,
};
use crate::models::list::List;
use crate::widgets::sidebar::SidebarInput;

#[derive(Debug)]
pub enum ListInput {
    Rename(String),
    UpdateCount(i32),
    ChangeIcon(String),
}

pub enum ListOutput {
    RemoveList(DynamicIndex),
}

pub struct ListWidgets {
    icon: gtk::Image,
    name: gtk::Label,
    count: gtk::Label,
}

impl FactoryComponent<gtk::ListBox, SidebarInput> for List {
    type Command = ();
    type CommandOutput = ();
    type Input = ListInput;
    type Output = ListOutput;
    type InitParams = List;
    type Root = gtk::Box;
    type Widgets = ListWidgets;

    fn output_to_parent_msg(output: Self::Output) -> Option<SidebarInput> {
        Some(match output {
            ListOutput::RemoveList(index) => SidebarInput::RemoveList(index)
        })
    }

    fn init_model(params: Self::InitParams, index: &DynamicIndex, input: &Sender<Self::Input>, output: &Sender<Self::Output>) -> Self {
        params
    }

    fn init_root() -> Self::Root {
        view! {
            list_box = &gtk::Box {
                set_orientation: gtk::Orientation::Horizontal,
            }
        }
        list_box
    }

    fn init_widgets(&mut self, index: &DynamicIndex, root: &Self::Root, _returned_widget: &gtk::ListBoxRow, input: &Sender<Self::Input>, output: &Sender<Self::Output>) -> Self::Widgets {
        view! {
            icon = &gtk::Image {
                set_from_icon_name: Some(self.icon_name.as_ref().unwrap())
            }
        }
        view! {
            name = &gtk::Label {
                set_halign: gtk::Align::Start,
                set_hexpand: true,
                set_text: self.display_name.as_str(),
                set_margin_top: 10,
                set_margin_bottom: 10,
                set_margin_start: 15,
                set_margin_end: 15,
            }
        }
        view! {
            count = &gtk::Label {
                set_halign: gtk::Align::End,
                set_css_classes: &["dim-label", "caption"],
                set_text: track!(model.changed(List::count()), self.count.to_string().as_str()),
                set_margin_top: 10,
                set_margin_bottom: 10,
                set_margin_start: 15,
                set_margin_end: 15,
            }
        }
        root.append(&icon);
        root.append(&name);
        root.append(&count);
        ListWidgets {
            icon,
            name,
            count,
        }
    }

    fn update(&mut self, message: Self::Input, input: &Sender<Self::Input>, output: &Sender<Self::Output>) -> Option<Self::Command> {
        match message {
            ListInput::Rename(name) => self.display_name = name,
            ListInput::UpdateCount(count) => self.count = count,
            ListInput::ChangeIcon(icon) => {
                if icon.is_empty() {
                    self.icon_name = None
                } else {
                    self.icon_name = Some(icon)
                }
            }
        }
        None
    }

    fn update_view(&self, widgets: &mut Self::Widgets, input: &Sender<Self::Input>, output: &Sender<Self::Output>) {
        widgets.name.set_text(self.display_name.as_str());
        if let Some(icon) = &self.icon_name {
            widgets.icon.set_from_icon_name(Some(icon.as_str()));
        }
        widgets.count.set_text(self.count.to_string().as_str());
    }
}