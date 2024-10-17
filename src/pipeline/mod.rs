use async_trait::async_trait;
use tokio::{pin, time::interval};
use tokio_stream::{wrappers::IntervalStream, StreamExt};

/// The maximum number of observations that can be recevied before we
/// recompute statistical significance.
/// If this number is too low, we'll be performing compute-intensive
/// statical tests very often. If this number is too high, we could
/// be waiting too long before computing, which could permit us to promote more eagerly.
const DEFAULT_BATCH_SIZE: usize = 512;

/// An [Observer] watches a particular external system (like AWS CloudWatch Logs)
/// and converts them into observations before emitting them as a stream.
#[async_trait]
pub trait Observer {
    /// The kind of object emitted by the Observer.
    type Item;

    /// The [query] method will query the observable external system on demand
    /// and produce a collection of observations. This collection of observations
    /// is supposed to represent the set that occurred since the last time this
    /// function was called.
    // TODO: This should return a result which we should account for in error handling.
    async fn query(&mut self) -> Vec<Self::Item>;
}

// TODO: Add a call to chunk_timeout to ensure that items are arriving after a particular
//       amount of time.
/// [repeat_query] runs the query on an interval and returns a stream of items.
/// This function runs indefinitely.
pub fn repeat_query<T: Observer>(
    mut observer: T,
    duration: tokio::time::Duration,
) -> impl tokio_stream::Stream<Item = T::Item> {
    // • Everything happens in this stream closure, which desugars
    //   into a background thread and a channel write at yield points.
    async_stream::stream! {
        // • Initialize a timer that fires every interval.
        let timer = IntervalStream::new(interval(duration));
        // • The timer must be pinned to use in an iterator
        //   because we must promise that its address must not
        //   be moved between iterations.
        pin!(timer);
        // Each iteration of the loop represents one unit of tiem.
        while let Some(_) = timer.next().await {
            // • We perform the query then dump the results into the stream.
            let items = observer.query().await;
            for item in items {
                yield item;
            }
        }
    }
}

// TODO: Honestly, this function can be inlined where used.
/// Batch observations together into maximally sized chunks, and dump
/// them to a stream every so often.
pub fn batch_observations<T: Observer>(
    obs: impl tokio_stream::Stream<Item = T::Item>,
    duration: tokio::time::Duration,
) -> impl tokio_stream::Stream<Item = Vec<T::Item>> {
    obs.chunks_timeout(DEFAULT_BATCH_SIZE, duration)
}

#[cfg(test)]
mod tests {
    use static_assertions::assert_obj_safe;

    use super::Observer;

    assert_obj_safe!(Observer<Item = ()>);
}
