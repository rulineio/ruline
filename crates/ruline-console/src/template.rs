use crate::Result;
use dashmap::DashMap;
use serde::Serialize;
use thiserror::Error;
use tinytemplate::TinyTemplate;

pub struct TemplateClient {
    email_templates: DashMap<String, String>,
}

impl TemplateClient {
    pub fn new() -> Result<Self> {
        let templates = DashMap::new();

        for entry in std::fs::read_dir("emails").map_err(TemplateError::Io)? {
            let entry = entry.map_err(TemplateError::Io)?;
            let path = entry.path();
            let name = path.file_stem().unwrap().to_str().unwrap().to_owned();
            let template = std::fs::read_to_string(path).map_err(TemplateError::Io)?;

            templates.insert(name, template);
        }

        Ok(Self {
            email_templates: templates,
        })
    }

    pub fn render<T: Serialize>(&self, name: &str, data: T) -> Result<String> {
        let template = self
            .email_templates
            .get(name)
            .ok_or_else(|| TemplateError::NotFound(name.to_owned()))?;

        let mut tt = TinyTemplate::new();
        tt.add_template(name, &template)
            .map_err(TemplateError::TinyTemplate)?;

        Ok(tt
            .render(name, &data)
            .map_err(TemplateError::TinyTemplate)?)
    }
}

pub enum Template {
    Login(LoginTemplate),
    Invitation(InvitationTemplate),
}

impl Template {
    pub fn render_email(&self, client: &TemplateClient) -> Result<String> {
        match self {
            Template::Login(data) => client.render("login", data),
            Template::Invitation(data) => client.render("invitation", data),
        }
    }

    pub fn render_text(&self) -> String {
        match self {
            Template::Login(data) => format!("Your link for Ruline: {}. The link is valid for 5 minutes. If you didn't request this link, you can safely ignore this email.", data.url),
            Template::Invitation(data) => format!("You have been invited to join {} on Ruline. To review the invitation, click on the following link: {}. If you are not aware of this invitation, please ignore this email.", data.organization, data.url),
        }
    }
}

#[derive(Serialize)]
pub struct LoginTemplate {
    pub url: String,
    pub label: String,
}

#[derive(Serialize)]
pub struct InvitationTemplate {
    pub url: String,
    pub organization: String,
}

#[derive(Debug, Error)]
pub enum TemplateError {
    #[error("template not found: {0}")]
    NotFound(String),

    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    TinyTemplate(#[from] tinytemplate::error::Error),
}
