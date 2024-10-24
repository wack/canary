use futures_core::stream::Stream;
use tokio::sync::mpsc::Sender;

use crate::stats::Observation;

pub use engines::*;
pub use ingresses::*;
pub use monitors::*;

pub struct CloudwatchLogs {
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

impl CloudwatchLogs {
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

/// Contains the trait definition and decision engine implementations.
/// DecisionEngines are responsible for determining
/// how much traffic is sent to deployments and when deployments should be yanked or promoted.
mod engines;
/// Contains the trait definition and ingress implementations. Ingresses are responsible
/// for actuating changes to traffic.
mod ingresses;
mod monitors;

#[cfg(test)]
mod tests {
    use crate::adapters::Observation;
    use crate::metrics::ResponseStatusCode;
    use crate::stats::Group;

    use super::{CloudwatchLogs, ObservationEmitter};

    use futures_util::pin_mut;
    use futures_util::StreamExt;

    struct FakeObservationEmitter;
    impl ObservationEmitter for FakeObservationEmitter {
        fn emit_next(&mut self) -> Vec<super::Observation> {
            vec![Observation {
                group: Group::Control,
                outcome: ResponseStatusCode::_2XX,
            }]
        }
    }

    #[tokio::test]
    async fn smoke_adapter_works() {
        let event_stream = CloudwatchLogs::new(FakeObservationEmitter);
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
