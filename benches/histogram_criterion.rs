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

extern crate test;

#[macro_use]
extern crate criterion;
use criterion::Criterion;

use prometheus::{Histogram, HistogramOpts, HistogramVec};
use test::Bencher;

fn bench_histogram(c: &mut Criterion) {
    let mut group = c.benchmark_group("bench_histogram");

    group.bench_function(
        "with labels",
        |b| {
            let histogram = HistogramVec::new(
                HistogramOpts::new("benchmark_histogram", "A histogram to benchmark it."),
                &["one", "two", "three"],
            )
            .unwrap();
            b.iter(|| {
                histogram
                    .with_label_values(&["eins", "zwei", "drei"])
                    .observe(3.1415)
            })
        }
    );

    group.bench_function(
        "no labels",
        |b| {
            let histogram = Histogram::with_opts(HistogramOpts::new(
                "benchmark_histogram",
                "A histogram to benchmark it.",
            ))
            .unwrap();
            b.iter(|| histogram.observe(3.1415))
        }
    );

    group.bench_function(
        "timer",
        |b| {
            let histogram = Histogram::with_opts(HistogramOpts::new(
                "benchmark_histogram_timer",
                "A histogram to benchmark it.",
            ))
            .unwrap();
            b.iter(|| histogram.start_timer())
        }
    );

    #[cfg(feature = "nightly")]
    group.bench_function(
        "coarse timer",
        |b| {
            let histogram = Histogram::with_opts(HistogramOpts::new(
                "benchmark_histogram_timer",
                "A histogram to benchmark it.",
            ))
            .unwrap();
            b.iter(|| histogram.start_coarse_timer())
        }
    );

    group.bench_function(
        "local",
        |b| {
            let histogram = Histogram::with_opts(HistogramOpts::new(
                "benchmark_histogram_local",
                "A histogram to benchmark it.",
            ))
            .unwrap();
            let local = histogram.local();
            b.iter(|| local.observe(3.1415));
            local.flush();
        }
    );

    group.bench_function(
        "local timer",
        |b| {
            let histogram = Histogram::with_opts(HistogramOpts::new(
                "benchmark_histogram_local_timer",
                "A histogram to benchmark it.",
            ))
            .unwrap();
            let local = histogram.local();
            b.iter(|| local.start_timer());
            local.flush();
        }
    );

    #[cfg(feature = "nightly")]
    group.bench_function(
        "local coarse timer",
        |b| {
            let histogram = Histogram::with_opts(HistogramOpts::new(
                "benchmark_histogram_timer",
                "A histogram to benchmark it.",
            ))
            .unwrap();
            let local = histogram.local();
            b.iter(|| local.start_coarse_timer());
            local.flush();
        }
    );
}

criterion_group!(histogram, bench_histogram);
criterion_main!(histogram);