//! Quarks are a very small thing.
//!
//! Quark is a diy native web UI library designed to help you create small, fast and responsive
//! desktop apps.
//! It provides multiple advantages over other similar projects, like [Electron]:
//! - Miniscule (178128 bytes/178 kb)
//! - Extremely fast rapid edit-debug-run cycles due to low dependency count
//! - No npm required to build 🎉!
//!
//! [Electron]: https://www.electronjs.org/

#![warn(missing_docs)]

pub mod cli;
pub mod config;
pub mod error;
pub mod prelude;
pub mod setup;
pub mod webview;

use config::QuarkConfig;
use error::QuarkError;
use webview::{Webview, WebviewBuilder};

#[allow(dead_code)]
pub struct Quark {
    webview: Webview,
    config: QuarkConfig,
}

impl Quark {
    /// Creates a `Quark` type from a `QuarkConfig`
    ///
    /// # Example
    ///
    /// ```rust
    /// # use libquark::prelude::*;
    /// # fn main() -> Result<(), QuarkError> {
    /// let config = QuarkConfig::new().title("Quark");
    /// let quark = Quark::new(config)?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// Most applications utilizing Quark will often call the `.run` method right after:
    ///
    /// ```rust,ignore
    /// quark.run();
    /// ```
    pub fn new(config: QuarkConfig) -> Result<Self, QuarkError> {
        let args = cli::parse_args();

        let webview = WebviewBuilder::new()
            .title(&config.title)
            .width(config.width)
            .height(config.height)
            .resize(config.resizable)
            .debug(cfg!(debug_assertions))
            .build();

        let mut quark = Quark { webview, config };

        if args.live {
            quark.setup_http()?;
        } else {
            quark.setup_static()?;
        }
        Ok(quark)
    }

    fn setup_static(&mut self) -> Result<(), QuarkError> {
        crate::setup::setup_static(self)
    }

    fn setup_http(&mut self) -> Result<(), QuarkError> {
        crate::setup::setup_http(self)
    }

    pub fn bind<F>(&mut self, name: &str, handler: F)
    where
        F: FnMut(&str, &str) + 'static,
    {
        self.webview.bind(name, handler);
    }

    /// Evaluates JavaScript code on the frontend web application
    ///
    /// This can be used to execute code on the frontend web application at any time in the
    /// backend.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// quark.eval("alert('hello!')");
    /// ```
    pub fn eval(&self, js: &str) {
        self.webview.eval(js);
    }

    /// Starts the quark application
    ///
    /// This will initialize the webview and show a window. This blocks the thread until Quark is
    /// closed.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use libquark::prelude::*;
    /// # fn main() -> Result<(), QuarkError> {
    /// let config = QuarkConfig::new().title("Quark");
    /// let quark = Quark::new(config)?;
    /// quark.run();
    /// # Ok(())
    /// # }
    /// ```
    pub fn run(&mut self) {
        self.webview.run();
    }
}
