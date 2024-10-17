use aws_sdk_cloudwatchlogs as cloudwatchlogs;
use futures_core::stream::Stream;
use tokio::sync::mpsc::Sender;

use crate::stats::Observation;

pub struct CloudwatchLogsAdapter {
    /// The AWS client for querying Cloudwatch Logs.
    client: Box<dyn ObservationEmitter>,
    outbox: Sender<Observation>,
}

// TODO: This must be a Boxed Async function since it needs
//       to perform nonblocking network IO.
/// An ObservationEmitter returns the next set of observations when queried.
/// The list of Observations may be empty if no observations occurred in the window.
pub trait ObservationEmitter: Send + Sync {
    fn emit_next(&mut self) -> Vec<Observation>;
}

impl CloudwatchLogsAdapter {
    /// Create a new [CloudwatchLogsAdapter] using a provided AWS client.
    pub fn new(client: impl ObservationEmitter + 'static) -> impl Stream<Item = Observation> {
        let (outbox, mut inbox) = tokio::sync::mpsc::channel(1024);
        let adapter = Self {
            client: Box::new(client),
            outbox,
        };
        tokio::spawn(async move {
            adapter.run().await;
        });
        async_stream::stream! {
            while let Some(item) = inbox.recv().await {
                yield item;
            }
        }
    }

    async fn run(mut self) {
        for item in self.client.emit_next() {
            self.outbox.send(item).await.unwrap();
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::adapter::Observation;
    use crate::stats::{Group, StatusCategory};

    use super::{CloudwatchLogsAdapter, ObservationEmitter};

    use futures_util::pin_mut;
    use futures_util::StreamExt;

    struct FakeObservationEmitter;
    impl ObservationEmitter for FakeObservationEmitter {
        fn emit_next(&mut self) -> Vec<super::Observation> {
            vec![Observation {
                group: Group::Control,
                outcome: StatusCategory::_2XX,
            }]
        }
    }

    #[tokio::test]
    async fn smoke_adapter_works() {
        let event_stream = CloudwatchLogsAdapter::new(FakeObservationEmitter);
        pin_mut!(event_stream);
        let mut count = 0;
        while let Some(_) = event_stream.next().await {
            println!("Yay!");
            count += 1;
            if count == 5 {
                break;
            }
        }
    }
}
