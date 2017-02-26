#![feature(test)]

#[macro_use]
extern crate serde_derive;
extern crate test;
extern crate bincode;
extern crate serde_bench;
extern crate flame;
extern crate latin;
extern crate serde_json;
extern crate serde;

use std::io::{Write, Read};
use serde::{Serialize, Deserialize};

mod model;
mod bincode_bencher;
mod serde_bencher;

#[derive(Debug, Serialize, Copy, Clone)]
struct RunInfo {
    strategy: &'static str,
    version: &'static str,
    output: &'static str,
    debug: bool,
    iterations: u64,
}


#[derive(Debug, Serialize)]
struct SerStats {
    info: RunInfo,
    size: u64,

    runs: Vec<u64>,
    min_ns: u64,
    max_ns: u64,
    avg_ns: u64,
}

#[derive(Debug, Serialize)]
struct DeStats {
    info: RunInfo,

    runs: Vec<u64>,
    min_ns: u64,
    max_ns: u64,
    avg_ns: u64,
}

fn get_json_file(count: usize) -> Vec<model::Element> {
    let vec_of_elements: Vec<model::Element> = serde_json::from_str(include_str!("../file.json")).unwrap();
    let iter = vec_of_elements.into_iter().cycle();
    iter.take(count).collect()
}

trait SerDeBench<T: Serialize + Deserialize> {
    fn ser<W: Write>(i: &T, w: &mut W);
    fn de<R: Read>(r: &mut R) -> T;
}

trait SliceSerDeBench<T: Serialize + Deserialize> {
    fn ser_vec(i: &T) -> Vec<u8>;
    fn de_slice(r: &[u8]) -> T;
}

fn duration_ns<R, F: FnOnce() -> R>(f: F) -> (u64, R) {
    use std::time::Instant;

    let before = Instant::now();
    let r = test::black_box(f());
    let after = Instant::now();

    let dur = after - before;
    let nanos = dur.as_secs() * 1_000_000_000 + dur.subsec_nanos() as u64;
    (nanos, r)
}

fn bench_vec_backing<B, T>(t: &T, info: RunInfo) -> (SerStats, DeStats)
    where T: Serialize + Deserialize, B: SerDeBench<T> {
    use std::cmp::{min, max};

    let buffer;

    // Serialization
    let ser_stats = {
        let mut min_ns = std::u64::MAX;
        let mut max_ns = 0;
        let mut total_ns = 0;

        let mut out = vec![];
        let mut runs = Vec::with_capacity(info.iterations as usize);

        for _ in 0 .. info.iterations {
            out.clear();

            let (nanos_elapsed, _) = duration_ns(|| B::ser(t, &mut out));

            runs.push(nanos_elapsed);
            min_ns = min(min_ns, nanos_elapsed);
            max_ns = max(max_ns, nanos_elapsed);
            total_ns += nanos_elapsed;
        }

        let len = out.len() as u64;
        buffer = out;

        SerStats {
            info: info,
            size: len,

            runs: runs,
            min_ns: min_ns,
            max_ns: max_ns,
            avg_ns: total_ns / info.iterations,
        }
    };

    // Deserialization
    let de_stats = {
        let mut min_ns = std::u64::MAX;
        let mut max_ns = 0;
        let mut total_ns = 0;

        let mut runs = Vec::with_capacity(info.iterations as usize);

        for _ in 0 .. info.iterations {
            let (nanos_elapsed, _) = duration_ns(|| B::de(&mut &buffer[..]));

            runs.push(nanos_elapsed);
            min_ns = min(min_ns, nanos_elapsed);
            max_ns = max(max_ns, nanos_elapsed);
            total_ns += nanos_elapsed;
        }

        DeStats {
            info: info,

            runs: runs,
            min_ns: min_ns,
            max_ns: max_ns,
            avg_ns: total_ns / info.iterations,
        }
    };

    (ser_stats, de_stats)
}

fn bench_slice<B, T>(t: &T, info: RunInfo) -> (SerStats, DeStats)
where T: Serialize + Deserialize, B: SliceSerDeBench<T>
{
    use std::cmp::{min, max};

    let buffer;

    // Serialization
    let ser_stats = {
        let mut min_ns = std::u64::MAX;
        let mut max_ns = 0;
        let mut total_ns = 0;

        let mut out = vec![];
        let mut runs = Vec::with_capacity(info.iterations as usize);

        for _ in 0 .. info.iterations {
            let (nanos_elapsed, r) = duration_ns(|| B::ser_vec(t));
            out = r;

            runs.push(nanos_elapsed);
            min_ns = min(min_ns, nanos_elapsed);
            max_ns = max(max_ns, nanos_elapsed);
            total_ns += nanos_elapsed;
        }

        let len = out.len() as u64;
        buffer = out;

        SerStats {
            info: info,
            size: len,

            runs: runs,
            min_ns: min_ns,
            max_ns: max_ns,
            avg_ns: total_ns / info.iterations,
        }
    };

    // Deserialization
    let de_stats = {
        let mut min_ns = std::u64::MAX;
        let mut max_ns = 0;
        let mut total_ns = 0;

        let mut runs = Vec::with_capacity(info.iterations as usize);

        for _ in 0 .. info.iterations {
            let (nanos_elapsed, _) = duration_ns(|| B::de_slice(&mut &buffer[..]));

            runs.push(nanos_elapsed);
            min_ns = min(min_ns, nanos_elapsed);
            max_ns = max(max_ns, nanos_elapsed);
            total_ns += nanos_elapsed;
        }

        DeStats {
            info: info,

            runs: runs,
            min_ns: min_ns,
            max_ns: max_ns,
            avg_ns: total_ns / info.iterations,
        }
    };

    (ser_stats, de_stats)
}

fn main() {
    let model = get_json_file(1000);

    // Write into an already-sized vec
    let bincode_vec_writer = RunInfo {
        strategy: "bincode",
        output: "vec (prealloc)",
        version: "eager prealloc",
        debug: cfg!(debug_assertions),
        iterations: 100,
    };

    // Make a new vec every time
    let bincode_vec = RunInfo {
        strategy: "bincode",
        output: "vec",
        version: "eager prealloc",
        debug: cfg!(debug_assertions),
        iterations: 100,
    };

    // Make a new vec every time
    let serde_bench = RunInfo {
        strategy: "serde-bench",
        output: "vec",
        version: "0.0.5",
        debug: cfg!(debug_assertions),
        iterations: 100,
    };

    let (ser_stats, de_stats) = bench_vec_backing::<bincode_bencher::BincodeBench, _>(&model, bincode_vec_writer);
    latin::file::append_line("./ser_stats.json", serde_json::to_string(&ser_stats).unwrap()).unwrap();
    latin::file::append_line("./de_stats.json", serde_json::to_string(&de_stats).unwrap()).unwrap();

    /*
    let (ser_stats, de_stats) = bench_slice::<serde_bencher::SerdeBench, _>(&model, serde_bench);
    latin::file::append_line("./ser_stats.json", serde_json::to_string(&ser_stats).unwrap()).unwrap();
    latin::file::append_line("./de_stats.json", serde_json::to_string(&de_stats).unwrap()).unwrap();
    */

    let (ser_stats, de_stats) = bench_slice::<serde_bencher::SerdeBench, _>(&model, bincode_vec);
    latin::file::append_line("./ser_stats.json", serde_json::to_string(&ser_stats).unwrap()).unwrap();
    latin::file::append_line("./de_stats.json", serde_json::to_string(&de_stats).unwrap()).unwrap();
}
