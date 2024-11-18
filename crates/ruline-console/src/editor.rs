use tokio::sync::broadcast;
use tracing::instrument;

use crate::{error::Error, Result};

#[derive(Clone)]
pub struct VersionEditor {
    pub members: Vec<VersionEditorMember>,
    pub tx: broadcast::Sender<VersionEditorEvent>,
}

impl VersionEditor {
    pub fn new() -> Self {
        Self {
            members: Vec::new(),
            tx: broadcast::channel(16).0,
        }
    }

    pub fn add_member(&mut self, member_id: String, name: String, avatar: String) -> Result<()> {
        if self.members.iter().any(|member| member.id == member_id) {
            return Ok(());
        }

        self.members.push(VersionEditorMember {
            id: member_id.to_owned(),
            name: name.to_owned(),
            avatar: avatar.to_owned(),
        });
        let _ = self
            .tx
            .send(VersionEditorEvent::MemberJoined {
                member_id,
                avatar,
                name,
            })
            .map_err(Error::ChannelSendError)?;
        Ok(())
    }

    pub fn remove_member(&mut self, member_id: &str) -> Result<()> {
        if let Some(index) = self
            .members
            .iter()
            .position(|member| member.id == member_id)
        {
            self.members.remove(index);
        }
        let _ = self
            .tx
            .send(VersionEditorEvent::MemberLeft {
                member_id: member_id.to_owned(),
            })
            .map_err(Error::ChannelSendError)?;
        Ok(())
    }

    #[instrument(skip(self), name = "Send Event", fields(otel.kind = "producer", event = event.name()))]
    pub fn send_event(&self, event: VersionEditorEvent) -> Result<()> {
        let _ = self.tx.send(event).map_err(Error::ChannelSendError)?;
        Ok(())
    }

    pub fn subscribe(&self) -> broadcast::Receiver<VersionEditorEvent> {
        self.tx.subscribe()
    }
}

#[derive(Debug, Clone)]
pub enum VersionEditorEvent {
    MemberJoined {
        member_id: String,
        avatar: String,
        name: String,
    },
    MemberLeft {
        member_id: String,
    },
}

impl VersionEditorEvent {
    pub fn name(&self) -> &'static str {
        match self {
            Self::MemberJoined { .. } => "member_joined",
            Self::MemberLeft { .. } => "member_left",
        }
    }
}

#[derive(Clone)]
pub struct VersionEditorMember {
    pub id: String,
    pub avatar: String,
    pub name: String,
}
