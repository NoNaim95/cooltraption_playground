use message_io::node;
use message_io::node::NodeListener;
use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::network_interface::bootstrapper::MessageIoInterfaceBootstrapper;
use crate::network_interface::messageio_adapter::MessageIoAdapter;
use crate::network_interface::NetworkInterfaceEventHandler;
use crate::network_interface::NetworkInterfaceWrapper;
use crate::network_interface::Signal;

pub struct MessageIoInterfaceBootstrapperBuilder<P>
where
    P: Serialize + DeserializeOwned,
{
    pub message_io_adapter_wrapper: NetworkInterfaceWrapper<MessageIoAdapter<P>, P>,
    pub network_state_publisher: Vec<NetworkInterfaceEventHandler<P>>,
    pub node_listener: NodeListener<Signal>,
}

impl<P> Default for MessageIoInterfaceBootstrapperBuilder<P>
where
    P: Serialize + DeserializeOwned,
{
    fn default() -> Self {
        let (node_handler, node_listener) = node::split::<Signal>();
        Self {
            message_io_adapter_wrapper: NetworkInterfaceWrapper::new(MessageIoAdapter::new(
                node_handler,
            )),
            network_state_publisher: vec![],
            node_listener,
        }
    }
}

impl<P> MessageIoInterfaceBootstrapperBuilder<P>
where
    P: Serialize + DeserializeOwned + 'static,
{
    pub fn add_network_interface_event_handler(
        &mut self,
        handler: NetworkInterfaceEventHandler<P>,
    ) {
        self.network_state_publisher.push(handler);
    }

    pub fn build(self) -> MessageIoInterfaceBootstrapper<P> {
        MessageIoInterfaceBootstrapper::new(
            self.message_io_adapter_wrapper,
            self.network_state_publisher,
            self.node_listener,
        )
    }
}
