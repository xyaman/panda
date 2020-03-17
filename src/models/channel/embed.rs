use serde::{Deserialize, Serialize};
use std::default::Default;

#[derive(Default, Debug, Deserialize, Serialize)]
pub struct Embed {
    title: Option<String>,
    #[serde(rename = "type")]
    kind: Option<String>,
    description: Option<String>,
    url: Option<String>,
    timestamp: Option<String>,
    color: Option<u64>, // TODO: COLOR ENUM
    footer: Option<EmbedFooter>,
    image: Option<EmbedImage>,
    thumbnail: Option<EmbedThumbnail>,
    video: Option<EmbedVideo>,
    provider: Option<EmbedProvider>,
    author: Option<EmbedAuthor>,
    fields: Vec<EmbedField>,
}

#[derive(Default, Debug, Deserialize, Serialize)]
pub struct EmbedFooter {
    text: String,
    icon_url: Option<String>,
    proxy_icon_url: Option<String>,
}

#[derive(Default, Debug, Deserialize, Serialize)]
pub struct EmbedImage {
    url: Option<String>,
    proxy_url: Option<String>,
    height: Option<u64>,
    width: Option<u64>,
}

#[derive(Default, Debug, Deserialize, Serialize)]
pub struct EmbedThumbnail {
    url: Option<String>,
    proxy_url: Option<String>,
    height: Option<u64>,
    width: Option<u64>,
}

#[derive(Default, Debug, Deserialize, Serialize)]
pub struct EmbedVideo {
    url: Option<String>,
    height: Option<u64>,
    width: Option<u64>,
}

#[derive(Default, Debug, Deserialize, Serialize)]
pub struct EmbedProvider {
    name: Option<String>,
    url: Option<String>,
}

#[derive(Default, Debug, Deserialize, Serialize)]
pub struct EmbedAuthor {
    name: Option<String>,
    url: Option<String>,
    icon_url: Option<String>,
    proxy_icon_url: Option<String>,
}

#[derive(Default, Debug, Deserialize, Serialize)]
pub struct EmbedField {
    name: String,
    value: String,
    inline: bool,
}

impl Embed {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_title(&mut self, title: impl Into<String>) -> &mut Self {
        self.title = Some(title.into());

        self
    }

    pub fn set_description(&mut self, description: impl Into<String>) -> &mut Self {
        self.description = Some(description.into());

        self
    }

    pub fn set_url(&mut self, url: impl Into<String>) -> &mut Self {
        self.url = Some(url.into());

        self
    }

    pub fn set_colot(&mut self, _color: u64) -> &mut Self {
        unimplemented!();
    }

    pub fn set_footer(&mut self, footer: EmbedFooter) -> &mut Self {
        self.footer = Some(footer);

        self
    }

    pub fn set_image(&mut self, image: EmbedImage) -> &mut Self {
        self.image = Some(image);

        self
    }
}
