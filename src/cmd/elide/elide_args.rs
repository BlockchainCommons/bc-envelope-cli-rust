use std::{rc::Rc, collections::HashSet};

use bc_components::SymmetricKey;
use clap::{ValueEnum, Args};

use bc_envelope::prelude::*;

use crate::utils::parse_digests;

/// The action to take on the elements.
#[derive(ValueEnum, Copy, Clone, Debug, PartialEq, Eq)]
pub enum Action {
    /// Elide the selected elements.
    Elide,
    /// Encrypt the selected elements using the given key.
    Encrypt,
    /// Compress the selected elements.
    Compress,
}

pub trait ElideArgsLike {
    fn action(&self) -> Action;
    fn key(&self) -> Option<&str>;
    fn target(&self) -> &String;

    fn get_target_set(&self) -> anyhow::Result<HashSet<Digest>> {
        parse_digests(self.target())
    }

    fn get_action(&self) -> anyhow::Result<ObscureAction> {
        let action = match self.action() {
            Action::Elide => ObscureAction::Elide,
            Action::Encrypt => {
                let key = self.key().ok_or_else(|| anyhow::anyhow!("No key provided"))?;
                let key = SymmetricKey::from_ur_string(key)?;
                ObscureAction::Encrypt(key)
            },
            Action::Compress => ObscureAction::Compress,
        };
        Ok(action)
    }

    fn run(&self, envelope: Rc<Envelope>, revealing: bool) -> anyhow::Result<Rc<Envelope>> {
        let target = self.get_target_set()?;
        let action = self.get_action()?;
        let result = envelope.elide_set_with_action(&target, revealing, &action);
        Ok(result)
    }
}

#[derive(Debug, Args)]
#[group(skip)]
pub struct ElideArgs {
    /// The action to take. If omitted, the action is `--elide`.
    #[arg(long, default_value = "elide")]
    action: Action,

    /// The encryption key (ur:crypto-key) to use when action is `--encrypt`.
    /// Ignored otherwise.
    #[arg(long)]
    key: Option<String>,

    /// The target set of digests: zero or more `ur:digest` or `ur:envelope`
    /// separated by a single space.
    target: String,
}

impl ElideArgsLike for ElideArgs {
    fn action(&self) -> Action {
        self.action
    }

    fn key(&self) -> Option<&str> {
        self.key.as_deref()
    }

    fn target(&self) -> &String {
        &self.target
    }
}
