use serde::{Deserialize, Serialize};
use std::default::Default;

#[derive(Default, Debug, Deserialize, Serialize)]
pub struct Embed {
    /// Title of the embed
    pub title: Option<String>,
    #[serde(rename = "type")]
    pub kind: Option<String>,
    pub description: Option<String>,
    pub url: Option<String>,
    pub timestamp: Option<String>,
    pub color: Option<u64>, // TODO: COLOR ENUM
    pub footer: Option<EmbedFooter>,
    pub image: Option<EmbedImage>,
    pub thumbnail: Option<EmbedThumbnail>,
    pub video: Option<EmbedVideo>,
    pub provider: Option<EmbedProvider>,
    pub author: Option<EmbedAuthor>,
    pub fields: Vec<EmbedField>,
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

    pub fn set_color(&mut self, _color: u64) -> &mut Self {
        unimplemented!();
    }

    pub fn add_footer(&mut self, footer: EmbedFooter) -> &mut Self {
        self.footer = Some(footer);

        self
    }

    pub fn add_image(&mut self, image: EmbedImage) -> &mut Self {
        self.image = Some(image);

        self
    }

    pub fn add_thumbnail(&mut self, thumbnail: EmbedThumbnail) -> &mut Self {
        self.thumbnail = Some(thumbnail);

        self
    }

    pub fn add_video(&mut self, video: EmbedVideo) -> &mut Self {
        self.video = Some(video);

        self
    }

    pub fn set_provider(&mut self, provider: EmbedProvider) -> &mut Self {
        self.provider = Some(provider);

        self
    }

    pub fn set_author(&mut self, author: EmbedAuthor) -> &mut Self {
        self.author = Some(author);

        self
    }

    pub fn add_field(&mut self, name: impl Into<String>, value: impl Into<String>, inline: bool) -> &mut Self {
        let field = EmbedField {
            name: name.into(),
            value: value.into(),
            inline,
        };
        self.fields.push(field);

        self
    }
}
