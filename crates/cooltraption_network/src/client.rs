use std::{net::SocketAddrV4, marker::PhantomData};
use quinn::{Endpoint, Connection, SendStream, RecvStream};

struct Client{
    endpoint: Endpoint,
    connection: Connection,
}

struct NetworkWriter{
    send: SendStream
}

struct NetworkReader{
    recv: RecvStream
}

impl Client {
    async fn connect(addr: SocketAddrV4) -> Self {
        //let client = Endpoint::client(std::net::SocketAddr::V4(addr));
        let endpoint = Endpoint::client("0.0.0.0:0".parse().unwrap()).unwrap();
        let server_addr = "127.0.0.1:5000".parse().unwrap();

        let connection = endpoint
            .connect(server_addr, "localhost")
            .unwrap()
            .await
            .unwrap();

        Self{ endpoint, connection }
    }

    async fn test(&mut self){
        let (mut send, recv) = self.connection.open_bi().await.unwrap();
    }
}
