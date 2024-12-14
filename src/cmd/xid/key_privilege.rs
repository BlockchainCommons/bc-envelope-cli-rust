use bc_xid::Privilege;
use clap::ValueEnum;

#[derive(ValueEnum, Copy, Clone, Debug, PartialEq, Eq, Default)]
pub enum KeyPrivilege {
    /// Allow all applicable XID operations
    #[default]
    All,

    //
    // Operational Functions
    //

    /// Operational: Authenticate as the subject (e.g., log into services)
    Auth,

    /// Operational: Sign digital communications as the subject
    Sign,

    /// Operational: Encrypt messages from the subject
    Encrypt,

    /// Operational: Elide data under the subject's control
    Elide,

    /// Operational: Issue or revoke verifiable credentials on the subject's authority
    Issue,

    /// Operational: Access resources under the subject's control
    Access,

    //
    // Management Functions
    //

    /// Management: Delegate priviledges to third parties
    Delegate,

    /// Management: Verify (update) the XID document
    Verify,

    /// Management: Update service endpoints
    Update,

    /// Management: Remove the inception key from the XID document
    Transfer,

    /// Management: Add or remove other verifiers (rotate keys)
    Elect,

    /// Management: Transition to a new provenance mark chain
    Burn,

    /// Management: Revoke the XID entirely
    Revoke,
}

impl From<KeyPrivilege> for Privilege {
    fn from(privilege: KeyPrivilege) -> Self {
        match privilege {
            KeyPrivilege::All => Privilege::All,
            KeyPrivilege::Auth => Privilege::Auth,
            KeyPrivilege::Sign => Privilege::Sign,
            KeyPrivilege::Encrypt => Privilege::Encrypt,
            KeyPrivilege::Elide => Privilege::Elide,
            KeyPrivilege::Issue => Privilege::Issue,
            KeyPrivilege::Access => Privilege::Access,
            KeyPrivilege::Delegate => Privilege::Delegate,
            KeyPrivilege::Verify => Privilege::Verify,
            KeyPrivilege::Update => Privilege::Update,
            KeyPrivilege::Transfer => Privilege::Transfer,
            KeyPrivilege::Elect => Privilege::Elect,
            KeyPrivilege::Burn => Privilege::Burn,
            KeyPrivilege::Revoke => Privilege::Revoke,
        }
    }
}
