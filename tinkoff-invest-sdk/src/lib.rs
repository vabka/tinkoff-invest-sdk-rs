mod generated;
pub mod instruments;
pub mod types;
pub mod users;

use users::*;
pub use generated::errors as error;

use error::TinkoffInvestError;
use instruments::InstrumentsClient;
pub use chrono;
pub use tinkoff_invest_grpc::decimal;
pub struct TinkoffInvestClient {
    internal: tinkoff_invest_grpc::TinkoffInvestClient,
}

pub type Result<T> = core::result::Result<T, TinkoffInvestError>;
macro_rules! service {
    ($name:ident, $internal:ty $(, {$($impl: item)*$(;)*})?) => {
        pub struct $name {
            internal: $internal
        }

        impl $name {
            pub(crate) fn new(internal: $internal) -> Self {
                Self {internal}
            }

            $(
                $(
                $impl
            )*
        )?
        }
    }
}
pub(crate) use service;

macro_rules! service_getter {
    ($name: ident, $service: ident) => {
        pub fn $name(&self) -> $service {
            $service::new(self.internal.$name())
        }
    };
}

impl TinkoffInvestClient {
    pub async fn connect(token: &str) -> core::result::Result<Self, Box<dyn std::error::Error>> {
        let internal = tinkoff_invest_grpc::TinkoffInvestClient::connect(token).await?;
        Ok(Self { internal })
    }
    service_getter!(users, UsersClient);
    service_getter!(instruments, InstrumentsClient);
    // market_data
    // market_data_stream
    // operations
    // operations_stream
    // orders
    // orders_stream
    // sandbox
    // stop_orders
}
