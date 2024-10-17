use aws_sdk_cloudwatchlogs as cloudwatchlogs;
use futures_core::stream::Stream;
use tokio::sync::mpsc::Sender;

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

/// An [Observation] represents a measured outcome that
/// belongs to either a control group or an experimental
/// group (i.e. canary).
pub struct Observation {
    /// The experimental group or the control group.
    group: Group,
    /// The outcome of the observation, by status code.
    outcome: StatusCategory,
}

/// The [Group] indicates from whence a given observation
/// was generated: either by a control group deployment or by
/// a canary deployment.
pub enum Group {
    /// The control group is the current running deployment.
    Control,
    /// The experimental group represents the canary deployment.
    Experimental,
}

/// [StatusCategory] groups HTTP response status codes according
/// to five general categories. This type is used as the dependent
/// variable in statical observations.
pub enum StatusCategory {
    // Information responses
    _1XX,
    // Successful responses
    _2XX,
    // Redirection messages
    _3XX,
    // Client error responses
    _4XX,
    // Server error responses
    _5XX,
}

#[cfg(test)]
mod tests {
    use crate::adapter::{Group, Observation, StatusCategory};

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
