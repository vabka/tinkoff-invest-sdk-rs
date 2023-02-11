use std::ops::RangeBounds;

use chrono::NaiveDate;
use prost_types::Timestamp;

use crate::types;


pub(crate) fn date_range_to_timestamp_pair(
    range: impl RangeBounds<NaiveDate>,
) -> (Option<Timestamp>, Option<Timestamp>) {
    let start = match range.start_bound() {
        std::ops::Bound::Included(date) => Some(types::chrono_timestamp_to_grpc_timestamp(
            date.and_hms_opt(0, 0, 0)
                .expect("Invalid hour/minute/second"),
        )),
        std::ops::Bound::Excluded(date) => Some(types::chrono_timestamp_to_grpc_timestamp(
            date.and_hms_opt(0, 0, 0)
                .expect("Invalid hour/minute/second")
                - chrono::Duration::days(1),
        )),
        std::ops::Bound::Unbounded => None,
    };

    let end = match range.end_bound() {
        std::ops::Bound::Included(date) => Some(types::chrono_timestamp_to_grpc_timestamp(
            date.and_hms_opt(0, 0, 0)
                .expect("Invalid hour/minute/second"),
        )),
        std::ops::Bound::Excluded(date) => Some(types::chrono_timestamp_to_grpc_timestamp(
            date.and_hms_opt(0, 0, 0)
                .expect("Invalid hour/minute/second")
                + chrono::Duration::days(1),
        )),
        std::ops::Bound::Unbounded => None,
    };

    (start, end)
}

pub(crate) trait EasyConvert<T> {
    fn convert(self) -> T;
}

impl<TTarget, TSelf: Into<TTarget>> EasyConvert<Vec<TTarget>> for Vec<TSelf> {
    fn convert(self) -> Vec<TTarget> {
        self.into_iter().map(Into::into).collect()
    }
}