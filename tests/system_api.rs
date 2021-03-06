// Copyright 2017 The Exonum Team
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//   http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

extern crate exonum;
extern crate exonum_testkit;

use exonum_testkit::{ApiKind, TestKitBuilder};
use exonum::api::public::HealthCheckInfo;

#[test]
fn test_healthcheck_connectivity_false() {
    let testkit = TestKitBuilder::validator().with_validators(2).create();
    let api = testkit.api();
    let info: HealthCheckInfo = api.get(ApiKind::System, "v1/healthcheck");
    let expected = HealthCheckInfo { connectivity: false };
    assert_eq!(info, expected);
}
