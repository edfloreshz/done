use self::{
	settings::Config,
	ui::{theme, theme::Theme, widget::Element, widgets::header_bar::header_bar},
};
use iced::{
	executor, font,
	widget::{column, container},
	Application, Length,
};
use iced_aw::graphics::icons::ICON_FONT_BYTES;
use log::{error, info};

pub mod environment;
pub mod logger;
pub mod notifications;
pub mod settings;
pub mod ui;

#[derive(Debug)]
pub enum Page {
	Welcome(ui::pages::Welcome),
	Home(ui::pages::Home),
	Help(ui::pages::Help),
}

impl Default for Page {
	fn default() -> Self {
		Page::Welcome(ui::pages::Welcome::default())
	}
}

#[derive(Debug, Default)]
pub struct Done {
	page: Page,
	theme: Theme,
	config: Config,
}

#[derive(Debug, Clone)]
pub enum Message {
	Welcome(ui::pages::welcome::Message),
	Home(ui::pages::home::Message),
	Help(ui::pages::help::Message),
	FontsLoaded(Result<(), font::Error>),
	Minimize,
	Maximize,
	Close,
}

impl Application for Done {
	type Executor = executor::Default;
	type Message = Message;
	type Theme = Theme;
	type Flags = Config;

	fn new(flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
		(
			Done::default(),
			iced::Command::batch(vec![
				font::load(ICON_FONT_BYTES).map(Message::FontsLoaded)
			]),
		)
	}

	fn title(&self) -> String {
		String::from("Done")
	}

	fn update(&mut self, message: Self::Message) -> iced::Command<Self::Message> {
		match message {
			Message::Welcome(message) => match message {},
			Message::Home(message) => match message {},
			Message::Help(message) => match message {},
			Message::FontsLoaded(result) => match result {
				Ok(_) => info!("Fonts loaded."),
				Err(e) => error!("{e:?}"),
			},
			Message::Minimize => return iced_runtime::window::minimize(true),
			Message::Maximize => return iced_runtime::window::toggle_maximize(),
			Message::Close => return iced_runtime::window::close(),
		}
		iced::Command::none()
	}

	fn view(&self) -> Element<Self::Message> {
		let content = match &self.page {
			Page::Welcome(welcome) => welcome.view().map(Message::Welcome),
			Page::Home(home) => home.view().map(Message::Home),
			Page::Help(help) => help.view().map(Message::Help),
		};

		container(column(vec![
			header_bar()
				.title("Done")
				.on_minimize(Message::Minimize)
				.on_maximize(Message::Maximize)
				.on_close(Message::Close)
				.into(),
			container(content).padding(10.0).into(),
		]))
		.width(Length::Fill)
		.height(Length::Fill)
		.style(theme::Container::Primary)
		.into()
	}

	fn theme(&self) -> Theme {
		self.theme.clone()
	}
}
