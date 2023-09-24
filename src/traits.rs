use std::ffi::OsStr;
use std::sync::Arc;

use libc::{gid_t, uid_t};

use base::{Group, User};

/// Trait for producers of users.
pub trait Users {
    /// Returns a `User` if one exists for the given user ID; otherwise, returns `None`.
    fn get_user_by_uid(&self, uid: uid_t) -> Option<Arc<User>>;

    /// Returns a `User` if one exists for the given username; otherwise, returns `None`.
    fn get_user_by_name<S: AsRef<OsStr> + ?Sized>(&self, username: &S) -> Option<Arc<User>>;

    /// Returns the user ID for the user running the process.
    fn get_current_uid(&self) -> uid_t;

    /// Returns the username of the user running the process.
    fn get_current_username(&self) -> Option<Arc<OsStr>>;

    /// Returns the effective user id.
    fn get_effective_uid(&self) -> uid_t;

    /// Returns the effective username.
    fn get_effective_username(&self) -> Option<Arc<OsStr>>;
}

/// Trait for producers of groups.
pub trait Groups {
    /// Returns a `Group` if one exists for the given group ID; otherwise, returns `None`.
    fn get_group_by_gid(&self, gid: gid_t) -> Option<Arc<Group>>;

    /// Returns a `Group` if one exists for the given groupname; otherwise, returns `None`.
    fn get_group_by_name<S: AsRef<OsStr> + ?Sized>(&self, group_name: &S) -> Option<Arc<Group>>;

    /// Returns the group ID for the user running the process.
    fn get_current_gid(&self) -> gid_t;

    /// Returns the group name of the user running the process.
    fn get_current_groupname(&self) -> Option<Arc<OsStr>>;

    /// Returns the effective group id.
    fn get_effective_gid(&self) -> gid_t;

    /// Returns the effective group name.
    fn get_effective_groupname(&self) -> Option<Arc<OsStr>>;
}

/// Trait for containers of users.
pub trait AllUsers {
    /// [`User`] iterator returned by [`all_users`][Self::all_users].
    type UserIter<'a>: Iterator<Item = &'a User>
    where
        Self: 'a;

    /// Creates a new iterator over every user present on the system.
    fn all_users(&self) -> Self::UserIter<'_>;

    /// Returns a `User` if one exists for the given user ID; otherwise, returns `None`.
    fn user_by_uid(&self, uid: uid_t) -> Option<&User>;

    /// Returns a `User` if one exists for the given username; otherwise, returns `None`.
    fn user_by_name<S: AsRef<OsStr> + ?Sized>(&self, username: &S) -> Option<&User>;

    /// Returns the user ID for the user running the process.
    fn current_uid(&self) -> uid_t;

    /// Returns the username of the user running the process.
    fn current_username(&self) -> Option<&OsStr>;

    /// Returns the effective user id.
    fn effective_uid(&self) -> uid_t;

    /// Returns the effective username.
    fn effective_username(&self) -> Option<&OsStr>;
}
