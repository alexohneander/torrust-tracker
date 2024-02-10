use clap::Parser;
use torrust_torrent_repository_benchmarks::args::Args;
use torrust_torrent_repository_benchmarks::benches::{asyn, sync, sync_asyn};
use torrust_tracker::core::torrent::entry::{Entry, MutexStd, MutexTokio};

#[allow(clippy::too_many_lines)]
#[allow(clippy::print_literal)]
fn main() {
    let args = Args::parse();

    // Add 1 to worker_threads since we need a thread that awaits the benchmark
    let rt = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(args.threads + 1)
        .enable_time()
        .build()
        .unwrap();

    println!("tokio::sync::RwLock<std::collections::BTreeMap<InfoHash, Entry>>");
    println!(
        "{}: Avg/AdjAvg: {:?}",
        "add_one_torrent",
        rt.block_on(asyn::add_one_torrent::<Entry>(1_000_000))
    );
    println!(
        "{}: Avg/AdjAvg: {:?}",
        "update_one_torrent_in_parallel",
        rt.block_on(asyn::update_one_torrent_in_parallel::<Entry>(&rt, 10))
    );
    println!(
        "{}: Avg/AdjAvg: {:?}",
        "add_multiple_torrents_in_parallel",
        rt.block_on(asyn::add_multiple_torrents_in_parallel::<Entry>(&rt, 10))
    );
    println!(
        "{}: Avg/AdjAvg: {:?}",
        "update_multiple_torrents_in_parallel",
        rt.block_on(asyn::update_multiple_torrents_in_parallel::<Entry>(&rt, 10))
    );

    if let Some(true) = args.compare {
        println!();

        println!("std::sync::RwLock<std::collections::BTreeMap<InfoHash, Entry>>");
        println!(
            "{}: Avg/AdjAvg: {:?}",
            "add_one_torrent",
            sync::add_one_torrent::<Entry>(1_000_000)
        );
        println!(
            "{}: Avg/AdjAvg: {:?}",
            "update_one_torrent_in_parallel",
            rt.block_on(sync::update_one_torrent_in_parallel::<Entry>(&rt, 10))
        );
        println!(
            "{}: Avg/AdjAvg: {:?}",
            "add_multiple_torrents_in_parallel",
            rt.block_on(sync::add_multiple_torrents_in_parallel::<Entry>(&rt, 10))
        );
        println!(
            "{}: Avg/AdjAvg: {:?}",
            "update_multiple_torrents_in_parallel",
            rt.block_on(sync::update_multiple_torrents_in_parallel::<Entry>(&rt, 10))
        );

        println!();

        println!("std::sync::RwLock<std::collections::BTreeMap<InfoHash, Arc<std::sync::Mutex<Entry>>>>");
        println!(
            "{}: Avg/AdjAvg: {:?}",
            "add_one_torrent",
            sync::add_one_torrent::<MutexStd>(1_000_000)
        );
        println!(
            "{}: Avg/AdjAvg: {:?}",
            "update_one_torrent_in_parallel",
            rt.block_on(sync::update_one_torrent_in_parallel::<MutexStd>(&rt, 10))
        );
        println!(
            "{}: Avg/AdjAvg: {:?}",
            "add_multiple_torrents_in_parallel",
            rt.block_on(sync::add_multiple_torrents_in_parallel::<MutexStd>(&rt, 10))
        );
        println!(
            "{}: Avg/AdjAvg: {:?}",
            "update_multiple_torrents_in_parallel",
            rt.block_on(sync::update_multiple_torrents_in_parallel::<MutexStd>(&rt, 10))
        );

        println!();

        println!("std::sync::RwLock<std::collections::BTreeMap<InfoHash, Arc<tokio::sync::Mutex<Entry>>>>");
        println!(
            "{}: Avg/AdjAvg: {:?}",
            "add_one_torrent",
            rt.block_on(sync_asyn::add_one_torrent::<MutexTokio>(1_000_000))
        );
        println!(
            "{}: Avg/AdjAvg: {:?}",
            "update_one_torrent_in_parallel",
            rt.block_on(sync_asyn::update_one_torrent_in_parallel::<MutexTokio>(&rt, 10))
        );
        println!(
            "{}: Avg/AdjAvg: {:?}",
            "add_multiple_torrents_in_parallel",
            rt.block_on(sync_asyn::add_multiple_torrents_in_parallel::<MutexTokio>(&rt, 10))
        );
        println!(
            "{}: Avg/AdjAvg: {:?}",
            "update_multiple_torrents_in_parallel",
            rt.block_on(sync_asyn::update_multiple_torrents_in_parallel::<MutexTokio>(&rt, 10))
        );

        println!();

        println!("tokio::sync::RwLock<std::collections::BTreeMap<InfoHash, Arc<std::sync::Mutex<Entry>>>>");
        println!(
            "{}: Avg/AdjAvg: {:?}",
            "add_one_torrent",
            rt.block_on(asyn::add_one_torrent::<MutexStd>(1_000_000))
        );
        println!(
            "{}: Avg/AdjAvg: {:?}",
            "update_one_torrent_in_parallel",
            rt.block_on(asyn::update_one_torrent_in_parallel::<MutexStd>(&rt, 10))
        );
        println!(
            "{}: Avg/AdjAvg: {:?}",
            "add_multiple_torrents_in_parallel",
            rt.block_on(asyn::add_multiple_torrents_in_parallel::<MutexStd>(&rt, 10))
        );
        println!(
            "{}: Avg/AdjAvg: {:?}",
            "update_multiple_torrents_in_parallel",
            rt.block_on(asyn::update_multiple_torrents_in_parallel::<MutexStd>(&rt, 10))
        );

        println!();

        println!("tokio::sync::RwLock<std::collections::BTreeMap<InfoHash, Arc<tokio::sync::Mutex<Entry>>>>");
        println!(
            "{}: Avg/AdjAvg: {:?}",
            "add_one_torrent",
            rt.block_on(asyn::add_one_torrent::<MutexTokio>(1_000_000))
        );
        println!(
            "{}: Avg/AdjAvg: {:?}",
            "update_one_torrent_in_parallel",
            rt.block_on(asyn::update_one_torrent_in_parallel::<MutexTokio>(&rt, 10))
        );
        println!(
            "{}: Avg/AdjAvg: {:?}",
            "add_multiple_torrents_in_parallel",
            rt.block_on(asyn::add_multiple_torrents_in_parallel::<MutexTokio>(&rt, 10))
        );
        println!(
            "{}: Avg/AdjAvg: {:?}",
            "update_multiple_torrents_in_parallel",
            rt.block_on(asyn::update_multiple_torrents_in_parallel::<MutexTokio>(&rt, 10))
        );
    }
}
