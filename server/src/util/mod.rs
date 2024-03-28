// Evil extenions

use std::future::Future;

pub trait AsyncMap<In, Out, E, Fun, Fut>
where
    Fun: Fn(In) -> Fut,
    Fut: Future<Output = Result<Out, E>>,
{
    async fn transpose_flatten(self, fun: Fun) -> Result<Out, E>;
}

impl<In, Out, E, Fun, Fut> AsyncMap<In, Out, E, Fun, Fut> for Result<In, E>
where
    Fun: Fn(In) -> Fut,
    Fut: Future<Output = Result<Out, E>>,
{
    async fn transpose_flatten(self, fun: Fun) -> Result<Out, E> {
        fun(self?).await
    }
}
