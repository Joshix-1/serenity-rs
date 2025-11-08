use std::any::Any;
use std::sync::Arc;

const INVALID_TYPE_EROR_MESSAGE: &str =
    "Type provided to Context should be the same as ClientBuilder::data.";

pub(super) trait PrivateDataContainerTrait {
    #[must_use]
    fn data_arc(&self) -> &Arc<dyn Any + Send + Sync + 'static>;
}

#[expect(private_bounds, reason = "The doc strings only make sense for our types.")]
pub trait DataContainer: PrivateDataContainerTrait {
    /// A container for a data type that can be used across contexts.
    ///
    /// The purpose of the data field is to be accessible and persistent across contexts; that is,
    /// data can be modified by one context, and will persist through the future and be accessible
    /// through other contexts. This is useful for anything that should "live" through the program:
    /// counters, database connections, custom user caches, etc.
    ///
    /// # Panics
    /// Panics if the generic provided is not equal to the type provided in [`ClientBuilder::data`].
    ///
    /// [`ClientBuilder::data`]: super::ClientBuilder::data
    #[must_use]
    fn data<Data: Send + Sync + 'static>(&self) -> Arc<Data> {
        self.try_data().expect(INVALID_TYPE_EROR_MESSAGE)
    }

    /// Tries to fetch the data type provided to [`ClientBuilder::data`].
    ///
    /// This returns None if no data was provided or Data is the wrong type and
    /// is mostly for Framework usage, normal bots should just use [`Self::data`].
    #[must_use]
    fn try_data<Data: Send + Sync + 'static>(&self) -> Option<Arc<Data>> {
        Arc::clone(self.data_arc()).downcast().ok()
    }

    /// A version of [`Self::data`] which returns a reference to the Data.
    ///
    /// This is useful if you need to borrow `Data` with the lifetime of `Self`, but otherwise
    /// [`Self::data`] should be used.
    ///
    /// # Panics
    /// Panics if the generic provided is not equal to the type provided in [`ClientBuilder::data`].
    ///
    /// [`ClientBuilder::data`]: super::ClientBuilder::data
    #[must_use]
    #[expect(clippy::needless_lifetimes, reason = "Easier to understand when explicitly written")]
    fn data_ref<'a, Data: Send + Sync + 'static>(&'a self) -> &'a Data {
        self.try_data_ref().expect(INVALID_TYPE_EROR_MESSAGE)
    }

    /// Tries to fetch the data type provided to [`ClientBuilder::data`].
    ///
    /// This returns None if no data was provided or Data is the wrong type and
    /// is mostly for Framework usage, normal bots should just use [`Self::data_ref`].
    #[must_use]
    #[expect(clippy::needless_lifetimes, reason = "Easier to understand when explicitly written")]
    fn try_data_ref<'a, Data: Send + Sync + 'static>(&'a self) -> Option<&'a Data> {
        self.data_arc().downcast_ref()
    }
}
