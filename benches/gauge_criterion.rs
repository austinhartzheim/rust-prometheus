// Copyright 2016 PingCAP, Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// See the License for the specific language governing permissions and
// limitations under the License.

#![feature(test)]

#[macro_use]
extern crate criterion;
use criterion::Criterion;

use prometheus::{Gauge, GaugeVec, IntGauge, Opts};

fn bench_gauge(c: &mut Criterion) {
    let mut group = c.benchmark_group("bench_gauge");

    group.bench_function(
        "with labels",
        |b| {
            let gauge = GaugeVec::new(
                Opts::new("benchmark_gauge", "A gauge to benchmark it."),
                &["one", "two", "three"],
            )
            .unwrap();
            b.iter(|| gauge.with_label_values(&["eins", "zwei", "drei"]).inc())
        }
    );

    group.bench_function(
        "no labels",
        |b| {
            let gauge = Gauge::new("benchmark_gauge", "A gauge to benchmark.").unwrap();
            b.iter(|| gauge.inc());
        }
    );

    group.bench_function(
        "int gauge no labels",
        |b| {
            let gauge = IntGauge::new("benchmark_int_gauge", "A int_gauge to benchmark.").unwrap();
            b.iter(|| gauge.inc());
        }
    );
}

criterion_group!(
    gauge,
    bench_gauge,
);
criterion_main!(gauge);