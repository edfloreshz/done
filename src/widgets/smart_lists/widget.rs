use gtk::traits::{BoxExt, WidgetExt};
use relm4::{
	component::{
		AsyncComponent, AsyncComponentController, AsyncComponentParts,
		AsyncController,
	},
	gtk, AsyncComponentSender,
};

use super::{
	all::model::AllModel, next7days::model::Next7DaysModel,
	sidebar::model::SmartList, starred::model::StarredModel,
	today::model::TodayModel,
};

pub struct SmartListContainerModel {
	pub all: AsyncController<AllModel>,
	pub today: AsyncController<TodayModel>,
	pub starred: AsyncController<StarredModel>,
	pub next7days: AsyncController<Next7DaysModel>,
	pub selected_smart_list: Option<SmartList>,
}

#[derive(Debug)]
pub enum SmartListContainerInput {
	SetSmartList(SmartList),
}

#[derive(Debug)]
pub enum SmartListContainerOutput {}

#[derive(derive_new::new)]
pub struct SmartListContainerInit {
	pub selected_smart_list: Option<SmartList>,
}

#[relm4::component(pub async)]
impl AsyncComponent for SmartListContainerModel {
	type CommandOutput = ();
	type Input = SmartListContainerInput;
	type Output = SmartListContainerOutput;
	type Init = SmartListContainerInit;

	view! {
				#[root]
				gtk::Box {
						gtk::Box {
								#[watch]
								set_visible: model.selected_smart_list.is_some() && model.selected_smart_list.as_ref().unwrap() == &SmartList::All,
								append: model.all.widget()
						},
						gtk::Box {
								#[watch]
								set_visible: model.selected_smart_list.is_some() && model.selected_smart_list.as_ref().unwrap() == &SmartList::Today,
								append: model.today.widget()
						},
						gtk::Box {
								#[watch]
								set_visible: model.selected_smart_list.is_some() && model.selected_smart_list.as_ref().unwrap() == &SmartList::Starred,
								append: model.starred.widget()
						},
						gtk::Box {
								#[watch]
								set_visible: model.selected_smart_list.is_some() && model.selected_smart_list.as_ref().unwrap() == &SmartList::Next7Days,
								append: model.next7days.widget()
						}
				}
	}

	async fn init(
		init: Self::Init,
		root: Self::Root,
		sender: AsyncComponentSender<Self>,
	) -> AsyncComponentParts<Self> {
		let all = AllModel::builder()
			.launch(())
			.forward(sender.input_sender(), |message| match message {});
		let today = TodayModel::builder()
			.launch(())
			.forward(sender.input_sender(), |message| match message {});
		let starred = StarredModel::builder()
			.launch(())
			.forward(sender.input_sender(), |message| match message {});
		let next7days = Next7DaysModel::builder()
			.launch(())
			.forward(sender.input_sender(), |message| match message {});

		let model = SmartListContainerModel {
			all,
			today,
			starred,
			next7days,
			selected_smart_list: init.selected_smart_list,
		};
		let widgets = view_output!();
		AsyncComponentParts { model, widgets }
	}

	async fn update(
		&mut self,
		message: Self::Input,
		_sender: AsyncComponentSender<Self>,
		_root: &Self::Root,
	) {
		match message {
			SmartListContainerInput::SetSmartList(list) => {
				self.selected_smart_list = Some(list)
			},
		}
	}
}
