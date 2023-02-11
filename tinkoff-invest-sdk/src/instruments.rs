use std::ops::{RangeBounds};

use chrono::{NaiveDate};

use tinkoff_invest_grpc::api::instruments_service_client::InstrumentsServiceClient;
use tinkoff_invest_grpc::api;
use tinkoff_invest_grpc::Inner;

use crate::error::{ErrorType, TinkoffInvestError};
use crate::shared::EasyConvert;
use crate::shared::date_range_to_timestamp_pair;
use crate::types::InstrumentsList;
use crate::{
    service,
    types,
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
        range: impl RangeBounds<NaiveDate>,
    ) -> crate::Result<Vec<types::TradingSchedule>> {
        let (start, end) = date_range_to_timestamp_pair(range);
        let req = api::TradingSchedulesRequest {
            exchange: "".to_owned(),
            from: start,
            to: end,
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
        Ok(schedules.convert())
    }

    pub async fn trading_schedules(
        &mut self,
        exchange: String,
        range: impl RangeBounds<NaiveDate>,
    ) -> crate::Result<Vec<types::TradingSchedule>> {
        let (start, end) = date_range_to_timestamp_pair(range);
        let req = api::TradingSchedulesRequest {
            exchange: exchange,
            from: start,
            to: end,
        };
        self.trading_schedules_internal(req).await
    }

    pub async fn bonds(&mut self, list: InstrumentsList) -> crate::Result<Vec<types::Bond>> {
        let numeric_status = match list {
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
        Ok(bonds.convert())
    }

    pub async fn get_bond_coupons(
        &mut self,
        figi: String,
        range: impl RangeBounds<NaiveDate>,
    ) -> crate::Result<Vec<types::Coupon>> {
        let (start, end) = date_range_to_timestamp_pair(range);
        let response = self
            .internal
            .get_bond_coupons(api::GetBondCouponsRequest {
                figi: figi,
                from: start,
                to: end,
            })
            .await?;
        let data = response.into_inner();
        let coupons = data.events;
        Ok(coupons.convert())
    }

    pub async fn currency_by() {}

    pub async fn currencies() {}

    pub async fn etf_by() {}
    pub async fn etfs() {}
    pub async fn future_by() {}

    pub async fn futures() {}

    pub async fn share_by() {}

    pub async fn shares() {}
    pub async fn get_accrues_interests() {}

    pub async fn get_futures_margin() {}
    pub async fn get_instrument_by() {}
    pub async fn get_dividends() {}

    pub async fn get_asset_by() {}
    pub async fn get_assets() {}
    pub async fn get_favorites() {}

    pub async fn edit_favorites() {}
    pub async fn get_countries() {}

    pub async fn find_instrument() {}
    pub async fn get_brands() {}

    pub async fn get_brand_by() {}
}
