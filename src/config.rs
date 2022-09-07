use std::net::IpAddr;
use std::net::Ipv4Addr;
use std::net::Ipv6Addr;
use std::num::NonZeroU16;
use std::num::NonZeroU32;
use std::num::NonZeroUsize;
use std::time::Duration;

use tracing::event;
use tracing::Level;

pub(crate) const DEFAULT_PORT: NonZeroU16 = unsafe { NonZeroU16::new_unchecked(2223) };
pub(crate) const DEFAULT_DELAY_MS: NonZeroU32 = unsafe { NonZeroU32::new_unchecked(10000) };
pub(crate) const DEFAULT_MAX_LINE_LENGTH: NonZeroUsize = unsafe { NonZeroUsize::new_unchecked(32) };
pub(crate) const DEFAULT_MAX_CLIENTS: NonZeroUsize = unsafe { NonZeroUsize::new_unchecked(64) };

pub(crate) struct Config {
    pub(crate) port: NonZeroU16,
    pub(crate) delay: Duration,
    pub(crate) max_line_length: NonZeroUsize,
    pub(crate) max_clients: NonZeroUsize,
    pub(crate) bind_family: IpAddr,
}

impl Config {
    pub(crate) fn new() -> Self {
        Self {
            port: DEFAULT_PORT,
            delay: Duration::from_millis(DEFAULT_DELAY_MS.get().into()),
            max_line_length: DEFAULT_MAX_LINE_LENGTH,
            max_clients: DEFAULT_MAX_CLIENTS,
            bind_family: IpAddr::V6(Ipv6Addr::UNSPECIFIED),
        }
    }

    pub(crate) fn set_port(&mut self, port: NonZeroU16) {
        self.port = port;
    }

    pub(crate) fn set_delay(&mut self, delay: NonZeroU32) {
        self.delay = Duration::from_millis(u64::from(delay.get()));
    }

    pub(crate) fn set_max_clients(&mut self, max_clients: NonZeroUsize) {
        self.max_clients = max_clients;
    }

    pub(crate) fn set_max_line_length(&mut self, l: NonZeroUsize) {
        self.max_line_length = l;
    }

    pub(crate) fn set_bind_family_ipv4(&mut self) {
        self.bind_family = IpAddr::V4(Ipv4Addr::UNSPECIFIED);
    }

    pub(crate) fn set_bind_family_ipv6(&mut self) {
        self.bind_family = IpAddr::V6(Ipv6Addr::UNSPECIFIED);
    }

    pub(crate) fn log(&self) {
        event!(Level::INFO, "Port: {}", self.port);
        event!(Level::INFO, "Delay: {}ms", self.delay.as_millis());
        event!(Level::INFO, "MaxLineLength: {}", self.max_line_length);
        event!(Level::INFO, "MaxClients: {}", self.max_clients);
        let bind_family_description = match self.bind_family {
            IpAddr::V6(_) => "Ipv4 + 6",
            IpAddr::V4(_) => "Ipv4 only",
        };
        event!(Level::INFO, "BindFamily: {}", bind_family_description);
    }
}
