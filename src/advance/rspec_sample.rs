use mockall::automock;
use std::{fmt::Debug, sync::Arc};

#[automock]
pub trait HmsMonitorService: Debug + Send + Sync {
    fn monitor(&self) -> bool;
}

#[derive(Clone, Debug)]
pub struct MonitorMessageConsumerListener {
    monitor_service: Arc<dyn HmsMonitorService>,
}

impl MonitorMessageConsumerListener {
    pub fn new(monitor_service: Arc<dyn HmsMonitorService>) -> Self {
        Self { monitor_service }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rspec::suite::Suite;
    use speculate::speculate;

    // #[test]
    // fn test_monitor() {
    speculate! {
        describe "monitor_listener" {
            before {
                let mut mock = MockHmsMonitorService::new();
                mock.expect_monitor().returning(|| true);
                let listener = MonitorMessageConsumerListener::new(Arc::new(mock));
            }

            it "should call monitor service" {
                assert!(listener.monitor_service.monitor());
            }

            it "should verify monitor call count" {
                let mut mock = MockHmsMonitorService::new();
                mock.expect_monitor().times(1).returning(|| false);
                println!("hello ");
                let listener = MonitorMessageConsumerListener::new(Arc::new(mock));
                assert_eq!(listener.monitor_service.monitor(), false);
            }
        }
    }
    // }

    #[derive(Clone, Default, Debug)]
    struct Environment {
        listener: Option<MonitorMessageConsumerListener>,
    }
    #[test]
    fn test_rspec_suite() {
        rspec::run(&rspec::describe(
            "monitor_listener",
            Environment::default(),
            |ctx| {
                ctx.before(|env| {
                    let mut mock = MockHmsMonitorService::new();
                    mock.expect_monitor().returning(|| true);
                    env.listener = Some(MonitorMessageConsumerListener::new(Arc::new(mock)));
                });

                ctx.it("should call monitor service", |env| {
                    assert!(env.listener.as_ref().unwrap().monitor_service.monitor());
                    true
                });
            },
        ));
    }
}
