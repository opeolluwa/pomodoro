use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub enum FocusTemplateKind {
    #[default]
    Work,
    Study,
    Personal,
    Fitness,
}
impl ToString for FocusTemplateKind {
    fn to_string(&self) -> String {
        let kind = match self {
            FocusTemplateKind::Work => "Work",
            FocusTemplateKind::Study => "Study",
            FocusTemplateKind::Personal => "Personal",
            FocusTemplateKind::Fitness => "Fitness",
        };
        kind.to_string()
    }
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct FocusTemplateTimerConfig {
    pub count: u8,
    pub short_break: u8,
    pub long_break: u8,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FocusTemplateCardOptions {
    pub kind: FocusTemplateKind,
    pub title: String,
    pub description: String,
    pub timer: FocusTemplateTimerConfig,
    pub key: u8,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StoredTemplates {
    pub templates: Vec<FocusTemplateCardOptions>,
}
