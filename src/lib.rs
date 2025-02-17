use zed_extension_api as zed;

struct TomorrowDarkTheme {}

impl zed::Extension for TomorrowDarkTheme {
    fn new() -> Self {
        TomorrowDarkTheme {}
    }
}

zed::register_extension!(TomorrowDarkTheme);
