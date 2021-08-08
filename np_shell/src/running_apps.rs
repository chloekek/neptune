use crate::AppId;
use crate::RunningApp;

use std::collections::BTreeMap;
use std::collections::HashMap;
use std::ops::Deref;
use std::os::unix::io::RawFd;
use std::rc::Rc;
use std::time::Instant;

/// Information about all running apps.
///
/// The information is indexed by various properties of the running apps.
/// This provides efficient lookup and iteration.
/// Access the indexes through the methods on this type.
pub struct RunningApps
{
    by_app_id: HashMap<AppId, Rc<RunningApp>>,
    by_process: HashMap<u32, Rc<RunningApp>>,
    by_socket: HashMap<RawFd, Rc<RunningApp>>,
    by_last_opened: BTreeMap<Instant, Rc<RunningApp>>,
}

impl RunningApps
{
    /// Create an empty set of running apps.
    pub fn new() -> Self
    {
        Self{
            by_app_id: HashMap::new(),
            by_process: HashMap::new(),
            by_socket: HashMap::new(),
            by_last_opened: BTreeMap::new(),
        }
    }

    /// Retrieve a running app by its app identifier.
    ///
    /// If there is no running app with
    /// the given app identifier,
    /// this function returns [`None`].
    pub fn by_app_id(&self, app_id: AppId) -> Option<&RunningApp>
    {
        self.by_app_id.get(&app_id).map(Rc::deref)
    }

    /// Retrieve a running app by its process identifier.
    ///
    /// If there is no running app with
    /// the given process identifier,
    /// this function returns [`None`].
    pub fn by_process(&self, process: u32) -> Option<&RunningApp>
    {
        self.by_process.get(&process).map(Rc::deref)
    }

    /// Retrieve a running app by its socket file descriptor.
    ///
    /// If there is no running app with
    /// the given socket file descriptor,
    /// this function returns [`None`].
    pub fn by_socket(&self, socket: RawFd) -> Option<&RunningApp>
    {
        self.by_socket.get(&socket).map(Rc::deref)
    }

    /// Iterator over the socket file descriptors of all running apps.
    ///
    /// The socket file descriptors are yielded in arbitrary order.
    pub fn sockets<'a>(&'a self) -> impl 'a + Iterator<Item=RawFd>
    {
        self.by_socket.keys().copied()
    }

    /// Iterator over all running apps,
    /// ordered by the time at which
    /// they were last opened.
    ///
    /// The apps are ordered such that the app
    /// which was last opened earliest comes first.
    /// For example, if there are two running apps \\( A \\) and \\( B \\),
    /// and app \\( A \\) was last opened ten minutes ago,
    /// and app \\( B \\) was last opened five minutes ago,
    /// the iterator yields the apps in this order:
    /// first app \\( A \\), then app \\( B \\).
    /// The iterator is double-ended,
    /// so you can iterate over the apps in opposite order
    /// using the [`Iterator::rev`] function.
    pub fn by_last_opened(&self) -> impl DoubleEndedIterator<Item=&RunningApp>
    {
        self.by_last_opened.values().map(Rc::deref)
    }
}
