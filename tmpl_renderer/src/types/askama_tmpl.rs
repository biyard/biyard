use askama::Template;

#[derive(Debug, Clone, Default)]
pub enum IndexType {
    #[default]
    IndexOnlyThisPage, // index, nofollow
    IndexAllPages,   // index, follow
    NoIndexThisPage, // noindex, follow
    NoIndexAllPages, // noindex, nofollow
    Block,           // disallow
}

#[derive(Debug, Template)]
#[template(path = "index.html")]
pub struct IndexTmpl {
    title: String,
    index_js: String,
    index_css: String,
    boot_json: String,
    index_type: IndexType,
    canonical_url: Option<String>,
    description: Option<String>,
    image_url: Option<String>,
}
// api/src/templates
impl IndexTmpl {
    pub fn new(title: impl Into<String>) -> Self {
        IndexTmpl {
            title: title.into(),
            index_js: "index.js".to_string(),
            index_css: "index.css".to_string(),
            boot_json: "{}".to_string(),
            index_type: IndexType::IndexOnlyThisPage,
            canonical_url: None,
            description: None,
            image_url: None,
        }
    }

    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = title.into();
        self
    }

    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn with_image_url(mut self, image_url: impl Into<String>) -> Self {
        self.image_url = Some(image_url.into());
        self
    }

    pub fn with_boot_json(mut self, boot_json: impl Into<String>) -> Self {
        self.boot_json = boot_json.into();
        self
    }

    pub fn with_index_type(mut self, index_type: IndexType) -> Self {
        self.index_type = index_type;
        self
    }

    pub fn with_canonical_url(mut self, canonical_url: impl Into<String>) -> Self {
        self.canonical_url = Some(canonical_url.into());
        self
    }

    pub fn with_index_js(mut self, index_js: impl Into<String>) -> Self {
        self.index_js = index_js.into();
        self
    }

    pub fn with_index_css(mut self, index_css: impl Into<String>) -> Self {
        self.index_css = index_css.into();
        self
    }

    pub fn to_html(&self) -> Result<String, RenderError> {
        self.render()
    }
}

pub type RenderError = askama::Error;
