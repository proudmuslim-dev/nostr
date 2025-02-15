// Copyright (c) 2022-2023 Yuki Kishimoto
// Copyright (c) 2023-2024 Rust Nostr Developers
// Distributed under the MIT software license

//! Nostr FFI

use std::sync::Arc;

use uniffi::Object;

mod error;
mod event;
pub mod helper;
mod key;
mod message;
mod nips;
mod types;
mod util;

pub use crate::error::NostrError;
pub use crate::event::{
    Event, EventBuilder, EventId, Tag, TagEnum, TagKind, TagKindKnown, UnsignedEvent,
};
pub use crate::key::{Keys, PublicKey, SecretKey};
pub use crate::message::{Alphabet, ClientMessage, Filter, RelayMessage};
pub use crate::nips::nip04::{nip04_decrypt, nip04_encrypt};
pub use crate::nips::nip05::{get_nip05_profile, verify_nip05};
pub use crate::nips::nip11::RelayInformationDocument;
//pub use crate::nips::nip44::{nip44_decrypt, nip44_encrypt};
pub use crate::nips::nip46::NostrConnectURI;
pub use crate::nips::nip57::ZapRequestData;
pub use crate::nips::nip94::FileMetadata;
pub use crate::types::{Contact, ImageDimensions, Metadata, Profile, Timestamp};
pub use crate::util::generate_shared_key;

#[derive(Object)]
pub struct NostrLibrary;

#[uniffi::export]
impl NostrLibrary {
    #[uniffi::constructor]
    pub fn new() -> Arc<Self> {
        Arc::new(Self)
    }

    pub fn git_hash_version(&self) -> String {
        nostr::git_hash_version().to_string()
    }
}

uniffi::setup_scaffolding!("nostr");
