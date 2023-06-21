use gettextrs::LocaleCategory;

use super::info::{GETTEXT_PACKAGE, LOCALEDIR};

pub(crate) fn init() {
	gettextrs::setlocale(LocaleCategory::LcAll, "");
	gettextrs::bindtextdomain(GETTEXT_PACKAGE, LOCALEDIR)
		.expect("Unable to bind the text domain");
	gettextrs::textdomain(GETTEXT_PACKAGE)
		.expect("Unable to switch to the text domain");
}
