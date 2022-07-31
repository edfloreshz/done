use i18n_embed::DesktopLanguageRequester;
use i18n_embed::{
	fluent::{fluent_language_loader, FluentLanguageLoader},
	DefaultLocalizer, LanguageLoader, Localizer,
};
use once_cell::sync::Lazy;
use rust_embed::RustEmbed;
use anyhow::Result;

#[derive(RustEmbed)]
#[folder = "locale/i18n/"]
struct Localizations;

pub static LANGUAGE_LOADER: Lazy<FluentLanguageLoader> = Lazy::new(|| {
	let loader: FluentLanguageLoader = fluent_language_loader!();

	loader
		.load_fallback_language(&Localizations)
		.expect("Error while loading fallback language");

	loader
});

#[macro_export]
macro_rules! fl {
    ($message_id:literal) => {{
        &i18n_embed_fl::fl!($crate::application::fluent::LANGUAGE_LOADER, $message_id)
    }};

    ($message_id:literal, $($args:expr),*) => {{
        &i18n_embed_fl::fl!($crate::application::fluent::LANGUAGE_LOADER, $message_id, $($args), *)
    }};
}

// Get the `Localizer` to be used for localizing this library.
pub fn localizer() -> Box<dyn Localizer> {
	Box::new(DefaultLocalizer::new(&*LANGUAGE_LOADER, &Localizations))
}

pub fn setup_fluent() -> Result<()> {
	let localizer = crate::application::fluent::localizer();
	let requested_languages = DesktopLanguageRequester::requested_languages();

	if let Err(error) = localizer.select(&requested_languages) {
		eprintln!(
			"Error while loading language for pop-desktop-widget {}",
			error
		);
	}
	Ok(())
}
