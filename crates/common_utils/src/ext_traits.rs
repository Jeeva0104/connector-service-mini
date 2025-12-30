// #[cfg(feature = "async_ext")]
#[cfg_attr(feature = "async_ext", async_trait::async_trait)]
impl<A: Send, E: Send + std::fmt::Debug> AsyncExt<A> for Result<A, E> {
    type WrappedSelf<T> = Result<T, E>;
    async fn async_and_then<F, B, Fut>(self, func: F) -> Self::WrappedSelf<B>
    where
        F: FnOnce(A) -> Fut + Send,
        Fut: futures::Future<Output = Self::WrappedSelf<B>> + Send,
    {
        match self {
            Ok(a) => func(a).await,
            Err(err) => Err(err),
        }
    }

    async fn async_map<F, B, Fut>(self, func: F) -> Self::WrappedSelf<B>
    where
        F: FnOnce(A) -> Fut + Send,
        Fut: futures::Future<Output = B> + Send,
    {
        match self {
            Ok(a) => Ok(func(a).await),
            Err(err) => Err(err),
        }
    }

    async fn async_unwrap_or_else<F, Fut>(self, func: F) -> A
    where
        F: FnOnce() -> Fut + Send,
        Fut: futures::Future<Output = A> + Send,
    {
        match self {
            Ok(a) => a,
            Err(_err) => func().await,
        }
    }
}

// #[cfg(feature = "async_ext")]
#[cfg_attr(feature = "async_ext", async_trait::async_trait)]
pub trait AsyncExt<A> {
    /// Output type of the map function
    type WrappedSelf<T>;

    /// Extending map by allowing functions which are async
    async fn async_map<F, B, Fut>(self, func: F) -> Self::WrappedSelf<B>
    where
        F: FnOnce(A) -> Fut + Send,
        Fut: futures::Future<Output = B> + Send;

    /// Extending the `and_then` by allowing functions which are async
    async fn async_and_then<F, B, Fut>(self, func: F) -> Self::WrappedSelf<B>
    where
        F: FnOnce(A) -> Fut + Send,
        Fut: futures::Future<Output = Self::WrappedSelf<B>> + Send;

    /// Extending `unwrap_or_else` to allow async fallback
    async fn async_unwrap_or_else<F, Fut>(self, func: F) -> A
    where
        F: FnOnce() -> Fut + Send,
        Fut: futures::Future<Output = A> + Send;
}
