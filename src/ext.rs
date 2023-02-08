use tokio::task::LocalKey;

mod private {
    pub trait Sealed {}
}

pub trait LocalKeyExt<T: Clone + 'static>: private::Sealed {
    #[track_caller]
    fn cloned(&'static self) -> T;
}

impl<T> private::Sealed for LocalKey<T> {}

impl<T: Clone + 'static> LocalKeyExt<T> for LocalKey<T> {
    /// Returns a clone of the task-local value
    /// if the task-local value implements `Clone`.
    ///
    /// # Panics
    ///
    /// This function will panic if the task local doesn't have a value set.
    #[track_caller]
    fn cloned(&'static self) -> T {
        self.with(|v| v.clone())
    }
}
