use futures::{Async, Future, IntoFuture, Poll, Stream};
use std::collections::VecDeque;

/// Converts a list of futures into a Stream of results from the futures
///
/// The stream will yield items as they become available on the futures internally,
/// in the order that their originating futures were submitted to the queue
///
/// The difference with `futures::futures_ordered` is that futures CAN NOT be complete out of order
pub fn futures_ordered<I>(futures: I) -> FuturesOrdered<<I::Item as IntoFuture>::Future>
where
    I: IntoIterator,
    I::Item: IntoFuture,
{
    let mut queue = FuturesOrdered {
        items: VecDeque::new(),
        current: None,
    };

    for future in futures {
        queue.items.push_back(future.into_future());
    }

    return queue;
}

#[must_use = "streams do nothing unless polled"]
pub struct FuturesOrdered<T>
where
    T: Future,
{
    items: VecDeque<T>,
    current: Option<T>,
}

impl<T> Stream for FuturesOrdered<T>
where
    T: Future,
{
    type Item = T::Item;
    type Error = T::Error;

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        loop {
            if let Some(ref mut f) = self.current {
                match f.poll()? {
                    Async::Ready(result) => {
                        self.current = self.items.pop_front();
                        return Ok(Async::Ready(Some(result)));
                    }
                    Async::NotReady => return Ok(Async::NotReady),
                }
            } else {
                if self.items.is_empty() {
                    return Ok(Async::Ready(None));
                }
                self.current = self.items.pop_front();
            }
        }
    }
}
