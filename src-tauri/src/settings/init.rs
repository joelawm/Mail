/*-------------
/settings/init.rs

Initializes the settings.toml file with a default value.
 
Changelog:
--- Version 1.0 - Joe Meyer
	-- Initial code release.
-------------*/

// Just a function that holds the initial settings value for the settings.toml file.
pub fn init_toml() -> &'static str {
	return r#"
	"hello" = 'toml!' # comment
	['a'.b]
	"#;
}