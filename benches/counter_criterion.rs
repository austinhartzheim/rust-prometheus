#[macro_use]
extern crate criterion;
use criterion::{Criterion, BenchmarkId};

use std::collections::HashMap;
use std::sync::{atomic, Arc};
use std::thread;

use prometheus::{Counter, CounterVec, IntCounter, Opts};

fn bench_counter_with_label_values(c: &mut Criterion) {
    let counter = CounterVec::new(
        Opts::new("benchmark_counter", "A counter to benchmark it."),
        &["one", "two", "three"],
    )
    .unwrap();

    c.bench_function(
        "bench_counter_with_label_values",
        move |b| b.iter(|| counter.with_label_values(&["eins", "zwei", "drei"]).inc()),
    );
}

fn bench_counter_with_mapped_labels(c: &mut Criterion) {
    let counter = CounterVec::new(
        Opts::new("benchmark_counter", "A counter to benchmark it."),
        &["one", "two", "three"],
    )
    .unwrap();

    c.bench_function(
        "bench_counter_with_mapped_labels",
        move |b| b.iter(|| {
            let mut labels = HashMap::with_capacity(3);
            labels.insert("two", "zwei");
            labels.insert("one", "eins");
            labels.insert("three", "drei");
            counter.with(&labels).inc();
        }),
    );
}

fn bench_counter_with_prepared_mapped_labels(c: &mut Criterion) {
    let counter = CounterVec::new(
        Opts::new("benchmark_counter", "A counter to benchmark it."),
        &["one", "two", "three"],
    )
    .unwrap();
    
    let mut labels = HashMap::with_capacity(3);
    labels.insert("two", "zwei");
    labels.insert("one", "eins");
    labels.insert("three", "drei");

    c.bench_function(
        "bench_counter_with_prepared_mapped_labels",
        move |b| b.iter(|| counter.with(&labels).inc()),
    );
}

fn bench_counter_no_labels(c: &mut Criterion) {
    let counter = Counter::new("benchmark_counter", "A counter to benchmark.").unwrap();

    c.bench_function(
        "bench_counter_no_labels",
        move |b| b.iter(|| counter.inc()),
    );
}

fn bench_int_counter_no_labels(c: &mut Criterion) {
    let counter = IntCounter::new("benchmark_int_counter", "A int_counter to benchmark.").unwrap();

    c.bench_function(
        "bench_int_counter_no_labels",
        move |b| b.iter(|| counter.inc()),
    );
}

fn bench_counter_concurrent(c: &mut Criterion) {
    let mut group = c.benchmark_group("bench_counter_concurrent");
    for thread_count in &[2, 4, 8, 16, 64] {
        group.bench_with_input(BenchmarkId::new("nop", thread_count), thread_count,
            |b, thread_count| {
                let signal_exit = Arc::new(atomic::AtomicBool::new(false));
                let counter = Counter::new("foo", "bar").unwrap();

                let thread_handles: Vec<_> = (0..*thread_count)
                    .map(|_| {
                        let signal_exit2 = signal_exit.clone();
                        let _counter2 = counter.clone();
                        thread::spawn(move || {
                            while !signal_exit2.load(atomic::Ordering::Relaxed) {
                                // Do nothing to simulate a lock that's not contended.
                            }
                        })
                    })
                    .collect();

                b.iter(|| counter.inc());

                // Wait for accompanying thread to exit.
                signal_exit.store(true, atomic::Ordering::Relaxed);
                for h in thread_handles {
                    h.join().unwrap();
                }
            }
        );

        group.bench_with_input(BenchmarkId::new("counter no labels", thread_count), thread_count,
            |b, thread_count| {
                let signal_exit = Arc::new(atomic::AtomicBool::new(false));
                let counter = Counter::new("foo", "bar").unwrap();
                
                let thread_handles: Vec<_> = (0..*thread_count)
                    .map(|_| {
                        let signal_exit2 = signal_exit.clone();
                        let counter2 = counter.clone();
                        thread::spawn(move || {
                            while !signal_exit2.load(atomic::Ordering::Relaxed) {
                                counter2.inc();
                            }
                        })
                    })
                    .collect();

                b.iter(|| counter.inc());

                // Wait for accompanying thread to exit.
                signal_exit.store(true, atomic::Ordering::Relaxed);
                for h in thread_handles {
                    h.join().unwrap();
                }
            }
        );

        group.bench_with_input(BenchmarkId::new("int counter no labels", thread_count), thread_count,
            |b, thread_count| {
                let signal_exit = Arc::new(atomic::AtomicBool::new(false));
                let counter = IntCounter::new("foo", "bar").unwrap();
                
                let thread_handles: Vec<_> = (0..*thread_count)
                    .map(|_| {
                        let signal_exit2 = signal_exit.clone();
                        let counter2 = counter.clone();
                        thread::spawn(move || {
                            while !signal_exit2.load(atomic::Ordering::Relaxed) {
                                counter2.inc();
                            }
                        })
                    })
                    .collect();

                b.iter(|| counter.inc());

                // Wait for accompanying thread to exit.
                signal_exit.store(true, atomic::Ordering::Relaxed);
                for h in thread_handles {
                    h.join().unwrap();
                }
            }
        );

        group.bench_with_input(BenchmarkId::new("int counter with labels", thread_count), thread_count,
            |b, thread_count| {
                let signal_exit = Arc::new(atomic::AtomicBool::new(false));
                let counter = CounterVec::new(Opts::new("foo", "bar"), &["one", "two", "three"]).unwrap();
                
                let thread_handles: Vec<_> = (0..*thread_count)
                    .map(|_| {
                        let signal_exit2 = signal_exit.clone();
                        let counter2 = counter.clone();
                        thread::spawn(move || {
                            while !signal_exit2.load(atomic::Ordering::Relaxed) {
                                counter2.with_label_values(&["eins", "zwei", "drei"]).inc();
                            }
                        })
                    })
                    .collect();

                b.iter(|| counter.with_label_values(&["eins", "zwei", "drei"]).inc());

                // Wait for accompanying thread to exit.
                signal_exit.store(true, atomic::Ordering::Relaxed);
                for h in thread_handles {
                    h.join().unwrap();
                }
            }
        );
    }

    group.finish();
}


criterion_group!(
    counter,
    bench_counter_with_label_values,
    bench_counter_with_mapped_labels,
    bench_counter_with_prepared_mapped_labels,
    bench_counter_no_labels,
    bench_int_counter_no_labels,
);
criterion_group!(
    counter_concurrent,
    bench_counter_concurrent,
);
criterion_main!(counter, counter_concurrent);