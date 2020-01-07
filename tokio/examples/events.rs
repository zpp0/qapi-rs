#[cfg(feature = "qmp")]
mod main {
    use std::env::args;
    use std::io;
    use futures::compat::Future01CompatExt;
    use futures::future::{FutureExt, TryFutureExt};
    use futures::stream::StreamExt;

    pub fn main() {
        ::env_logger::init();

        let socket_addr = args().nth(1).expect("argument: QMP socket path");

        tokio::run(async {
            let socket = tokio_uds::UnixStream::connect(socket_addr).compat().await?;
            let (caps, _stream, events) = tokio_qapi::QapiStream::open_tokio(socket).await?;
            println!("{:#?}", caps);

            let mut events = events.into_stream().boxed();
            while let Some(event) = events.next().await {
                println!("Got event {:#?}", event?);
            }

            Ok(())
        }.map_err(|err: io::Error| panic!("Failed with {:?}", err)).boxed().compat());
    }
}

#[cfg(not(feature = "qmp"))]
mod main {
    pub fn main() { panic!("requires feature qmp") }
}

fn main() { main::main() }
