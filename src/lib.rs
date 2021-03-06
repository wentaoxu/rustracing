//! [OpenTracing][opentracing] API for Rust
//!
//! # References
//!
//! - [The OpenTracing Semantic Specification (v1.1)][specification]
//!
//! [opentracing]: http://opentracing.io/
//! [specification]: https://github.com/opentracing/specification/blob/master/specification.md
#![warn(missing_docs)]
#![cfg_attr(feature = "cargo-clippy", allow(doc_markdown))]
extern crate backtrace;
extern crate rand;
#[macro_use]
extern crate trackable;

pub use error::{Error, ErrorKind};
pub use tracer::Tracer;

pub mod carrier;
pub mod convert;
pub mod log;
pub mod sampler;
pub mod span;
pub mod tag;

mod error;
mod tracer;

/// This crate specific `Result` type.
pub type Result<T> = std::result::Result<T, Error>;

#[cfg(test)]
mod test {
    use sampler::AllSampler;
    use tag::StdTag;
    use super::*;

    #[test]
    fn it_works() {
        let (tracer, span_rx) = Tracer::new(AllSampler);
        {
            let span = tracer.span("it_works").start_with_state(());
            let mut child = span.child("child", |options| options.start_with_state(()));
            child.set_tags(|| StdTag::peer_addr("127.0.0.1:80".parse().unwrap()));
        }

        let span = span_rx.try_recv().unwrap();
        assert_eq!(span.operation_name(), "child");

        let span = span_rx.try_recv().unwrap();
        assert_eq!(span.operation_name(), "it_works");
    }
}
