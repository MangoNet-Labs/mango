// Copyright (c) MangoNet Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use axum::http::HeaderName;

pub static VERSION_HEADER: HeaderName = HeaderName::from_static("x-mgo-rpc-version");
pub static LIMITS_HEADER: HeaderName = HeaderName::from_static("x-mgo-rpc-show-usage");
