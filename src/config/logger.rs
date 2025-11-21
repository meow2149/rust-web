use serde::Deserialize;

#[derive(Debug, Default, Deserialize)]
pub struct LoggerConfig {
    level: Option<String>,
    #[serde(default)]
    format: FormatConfig,
}

#[derive(Debug, Default, Deserialize)]
pub struct FormatConfig {
    #[serde(default = "default_true")]
    level: bool,
    #[serde(default = "default_false")]
    file: bool,
    #[serde(default = "default_false")]
    line_number: bool,
    #[serde(default = "default_false")]
    target: bool,
    #[serde(default = "default_false")]
    thread_ids: bool,
    #[serde(default = "default_false")]
    thread_names: bool,
    #[serde(default = "default_true")]
    ansi: bool,
}

fn default_true() -> bool {
    true
}

fn default_false() -> bool {
    false
}

impl LoggerConfig {
    pub fn level(&self) -> &str {
        self.level.as_deref().unwrap_or("info")
    }

    pub fn format(&self) -> &FormatConfig {
        &self.format
    }
}

impl FormatConfig {
    pub fn level(&self) -> bool {
        self.level
    }

    pub fn file(&self) -> bool {
        self.file
    }

    pub fn line_number(&self) -> bool {
        self.line_number
    }

    pub fn target(&self) -> bool {
        self.target
    }

    pub fn thread_ids(&self) -> bool {
        self.thread_ids
    }

    pub fn thread_names(&self) -> bool {
        self.thread_names
    }

    pub fn ansi(&self) -> bool {
        self.ansi
    }
}
