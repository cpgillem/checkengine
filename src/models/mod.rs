/// An account, check register, expense account, etc.
pub mod register;

/// A user.
pub mod member;

/// A "split" in something like gnucash. Defines a cash flow between two accounts.
pub mod posting;

/// A transaction which groups postings that equal to 0.
pub mod posting_group;