use bc_xid::Privilege;
use clap::ValueEnum;

#[derive(ValueEnum, Copy, Clone, Debug, PartialEq, Eq, Default)]
pub enum XIDPrivilege {
    /// Allow all applicable XID operations
    #[default]
    All,

    //
    // Operational Functions
    /// Operational: Authenticate as the subject (e.g., log into services)
    Auth,

    /// Operational: Sign digital communications as the subject
    Sign,

    /// Operational: Encrypt messages from the subject
    Encrypt,

    /// Operational: Elide data under the subject's control
    Elide,

    /// Operational: Issue or revoke verifiable credentials on the subject's
    /// authority
    Issue,

    /// Operational: Access resources under the subject's control
    Access,

    //
    // Management Functions
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

impl From<XIDPrivilege> for Privilege {
    fn from(privilege: XIDPrivilege) -> Self {
        match privilege {
            XIDPrivilege::All => Privilege::All,
            XIDPrivilege::Auth => Privilege::Auth,
            XIDPrivilege::Sign => Privilege::Sign,
            XIDPrivilege::Encrypt => Privilege::Encrypt,
            XIDPrivilege::Elide => Privilege::Elide,
            XIDPrivilege::Issue => Privilege::Issue,
            XIDPrivilege::Access => Privilege::Access,
            XIDPrivilege::Delegate => Privilege::Delegate,
            XIDPrivilege::Verify => Privilege::Verify,
            XIDPrivilege::Update => Privilege::Update,
            XIDPrivilege::Transfer => Privilege::Transfer,
            XIDPrivilege::Elect => Privilege::Elect,
            XIDPrivilege::Burn => Privilege::Burn,
            XIDPrivilege::Revoke => Privilege::Revoke,
        }
    }
}
