use std::fmt::Display;

use tokio::runtime::{Builder, Runtime};

pub mod protocal;
pub mod world;

pub use world::World;

struct Server<W: World> {
    runtime: Runtime,
    world: W,
}

impl<W: World> Server<W> {
    /// Creates a new server with default runtime with 2 worker thread.
    fn new(world: W) -> Result<Self, Error> {
        let runtime = Builder::new_multi_thread()
            .worker_threads(2)
            .thread_name("server-worker")
            .build();

        return Ok(Self {
            runtime: match runtime {
                Ok(value) => value,
                Err(e) => return Err(Error::RuntimeCreate(e)),
            },
            world,
        });
    }

    /// Create a new server with supplied runtime.
    fn with_runtime(runtime: Runtime, world: W) -> Self {
        return Self { runtime, world };
    }
}

impl<W: World + Default> Default for Server<W> {
    /// Creates a new server with default setting.
    ///
    /// Will panic when runtime failed to build
    fn default() -> Self {
        return Self::new(W::default()).unwrap();
    }
}

#[derive(Debug)]
enum Error {
    RuntimeCreate(std::io::Error),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::RuntimeCreate(e) => write!(f, "failed to create runtime: {e}")?,
        }

        Ok(())
    }
}
