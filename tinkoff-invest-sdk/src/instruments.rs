use std::ops::RangeInclusive;

use chrono::NaiveDate;
use tinkoff_invest_grpc::api::instruments_service_client::InstrumentsServiceClient;
use tinkoff_invest_grpc::api::{self};
use tinkoff_invest_grpc::Inner;

use crate::error::{ErrorType, TinkoffInvestError};
use crate::types::InstrumentsList;
use crate::{
    service,
    types::{self},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InstrumentRequest {
    Figi(String),
    Ticker { id: String, class_code: String },
    Uid(String),
}

impl From<InstrumentRequest> for api::InstrumentRequest {
    fn from(req: InstrumentRequest) -> api::InstrumentRequest {
        let mut request = api::InstrumentRequest::default();
        match req {
            InstrumentRequest::Figi(figi) => {
                request.set_id_type(api::InstrumentIdType::Figi);
                request.id = figi;
            }
            InstrumentRequest::Ticker { id, class_code } => {
                request.set_id_type(api::InstrumentIdType::Ticker);
                request.id = id;
                request.class_code = class_code;
            }
            InstrumentRequest::Uid(uid) => {
                request.set_id_type(api::InstrumentIdType::Uid);
                request.id = uid;
            }
        };
        request
    }
}

service!(InstrumentsClient, InstrumentsServiceClient<Inner>);
impl InstrumentsClient {
    pub async fn bond_by(
        &mut self,
        request: InstrumentRequest,
    ) -> crate::Result<Option<types::Bond>> {
        let req: api::InstrumentRequest = request.into();
        match self.internal.bond_by(req).await.map_err(Into::into) {
            Ok(response) => {
                let inner = response.into_inner();
                let bond = inner.instrument;
                Ok(bond.map(Into::into))
            }
            // Не смотря на то, что поле bond помечено как Option, если мы укажем не существующий id - нам вернётся 50002 код
            // По тому мы её обрабатываем и делаем Ok(None), а Err оставляем для всех остальных ошибок
            Err(error) => {
                let error: TinkoffInvestError = error;
                match error.error_type() {
                    ErrorType::NotFound => Ok(None),
                    _ => Err(error),
                }
            }
        }
    }

    pub async fn trading_schedules_all(
        &mut self,
        range: RangeInclusive<NaiveDate>,
    ) -> crate::Result<Vec<types::TradingSchedule>> {
        let (start, end) = range.into_inner();
        let req = api::TradingSchedulesRequest {
            exchange: "".to_owned(),
            from: Some(types::chrono_timestamp_to_grpc_timestamp(
                start
                    .and_hms_opt(0, 0, 0)
                    .expect("Invalid hour/minute/second"),
            )),
            to: Some(types::chrono_timestamp_to_grpc_timestamp(
                end.and_hms_opt(23, 59, 59)
                    .expect("Invalid hour/minute/second"),
            )),
        };
        self.trading_schedules_internal(req).await
    }

    async fn trading_schedules_internal(
        &mut self,
        req: api::TradingSchedulesRequest,
    ) -> crate::Result<Vec<types::TradingSchedule>> {
        let res = self.internal.trading_schedules(req).await?;
        let data = res.into_inner();
        let schedules = data.exchanges;
        Ok(unsafe { std::mem::transmute(schedules) })
    }

    pub async fn trading_schedules(
        &mut self,
        exchange: String,
        range: RangeInclusive<NaiveDate>,
    ) -> crate::Result<Vec<types::TradingSchedule>> {
        let (start, end) = range.into_inner();
        let req = api::TradingSchedulesRequest {
            exchange: exchange,
            from: Some(types::chrono_timestamp_to_grpc_timestamp(
                start
                    .and_hms_opt(0, 0, 0)
                    .expect("Invalid hour/minute/second"),
            )),
            to: Some(types::chrono_timestamp_to_grpc_timestamp(
                end.and_hms_opt(23, 59, 59)
                    .expect("Invalid hour/minute/second"),
            )),
        };
        self.trading_schedules_internal(req).await
    }

    pub async fn bonds(&mut self, list: InstrumentsList) -> crate::Result<Vec<types::Bond>> {
        let numeric_status =  match list {
            InstrumentsList::Base => api::InstrumentStatus::Base,
            InstrumentsList::All => api::InstrumentStatus::All,
        } as i32;

        let response = self
            .internal
            .bonds(api::InstrumentsRequest {
                instrument_status: numeric_status,
            })
            .await?;
        let data = response.into_inner();
        let bonds = data.instruments;
        return Ok(unsafe{ std::mem::transmute(bonds)})
    }
}
