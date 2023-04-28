use derive_more::Display;
use uuid::Uuid;
use warp::{
    crypto::DID,
    error::Error,
    raygun::{self, MessageEventKind},
};

use super::Message;
use crate::{
    state,
    warp_runner::ui_adapter::{convert_raygun_message, did_to_identity},
};

#[derive(Display)]
pub enum MessageEvent {
    #[display(fmt = "Received")]
    Received {
        conversation_id: Uuid,
        message: Message,
    },
    #[display(fmt = "Sent")]
    Sent {
        conversation_id: Uuid,
        message: Message,
    },
    #[display(fmt = "Edited")]
    Edited {
        conversation_id: Uuid,
        message: Message,
    },
    #[display(fmt = "Deleted")]
    Deleted {
        conversation_id: Uuid,
        message_id: Uuid,
    },
    #[display(fmt = "MessageReactionAdded")]
    MessageReactionAdded { message: warp::raygun::Message },
    #[display(fmt = "MessageReactionRemoved")]
    MessageReactionRemoved { message: warp::raygun::Message },
    #[display(fmt = "TypingIndicator")]
    TypingIndicator {
        conversation_id: Uuid,
        participant: DID,
    },
    #[display(fmt = "RecipientAdded")]
    RecipientAdded {
        conversation: raygun::Conversation,
        identity: state::Identity,
    },
    #[display(fmt = "RecipientRemoved")]
    RecipientRemoved { conversation: raygun::Conversation },
    #[display(fmt = "ConversationNameUpdated")]
    ConversationNameUpdated { conversation: raygun::Conversation },
}

pub async fn convert_message_event(
    event: warp::raygun::MessageEventKind,
    account: &mut super::super::Account,
    messaging: &mut super::super::Messaging,
) -> Result<MessageEvent, Error> {
    let evt = match event {
        MessageEventKind::MessageReceived {
            conversation_id,
            message_id,
        } => {
            let message = messaging.get_message(conversation_id, message_id).await?;

            // Return the event.
            MessageEvent::Received {
                conversation_id,
                message: convert_raygun_message(messaging, &message).await,
            }
        }
        MessageEventKind::MessageSent {
            conversation_id,
            message_id,
        } => {
            let message = messaging.get_message(conversation_id, message_id).await?;
            MessageEvent::Sent {
                conversation_id,
                message: convert_raygun_message(messaging, &message).await,
            }
        }
        MessageEventKind::MessageDeleted {
            conversation_id,
            message_id,
        } => MessageEvent::Deleted {
            conversation_id,
            message_id,
        },
        MessageEventKind::MessageReactionAdded {
            conversation_id,
            message_id,
            ..
        } => MessageEvent::MessageReactionAdded {
            message: messaging.get_message(conversation_id, message_id).await?,
        },
        MessageEventKind::MessageReactionRemoved {
            conversation_id,
            message_id,
            ..
        } => MessageEvent::MessageReactionRemoved {
            message: messaging.get_message(conversation_id, message_id).await?,
        },
        MessageEventKind::EventReceived {
            conversation_id,
            did_key,
            event,
        } => match event {
            raygun::MessageEvent::Typing => MessageEvent::TypingIndicator {
                conversation_id,
                participant: did_key,
            },
        },
        MessageEventKind::MessageEdited {
            conversation_id,
            message_id,
        } => {
            let message = messaging.get_message(conversation_id, message_id).await?;
            MessageEvent::Edited {
                conversation_id,
                message: convert_raygun_message(messaging, &message).await,
            }
        }
        MessageEventKind::RecipientAdded {
            conversation_id,
            recipient,
        } => MessageEvent::RecipientAdded {
            identity: did_to_identity(&recipient, account).await?,
            conversation: messaging.get_conversation(conversation_id).await?,
        },
        MessageEventKind::RecipientRemoved {
            conversation_id, ..
        } => MessageEvent::RecipientRemoved {
            conversation: messaging.get_conversation(conversation_id).await?,
        },
        MessageEventKind::ConversationNameUpdated {
            conversation_id, ..
        } => MessageEvent::ConversationNameUpdated {
            conversation: messaging.get_conversation(conversation_id).await?,
        },
        _ => {
            todo!();
        }
    };

    Ok(evt)
}
