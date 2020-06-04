use std::time::{Instant, Duration};
use rand::{thread_rng, RngCore, Rng};
use std::fs::OpenOptions;
use std::io::prelude::*;
use rand::distributions::Alphanumeric;
use eytzinger_sort::traversal_sort::eytzinger_traversal_sort;
use eytzinger_sort::find::first_index_for_eytzinger;
use eytzinger_sort::march_sort::eytzinger_march_sort;
use eytzinger_sort::clone::clone_as_eytzinger;
use eytzinger_sort::waltz_sort::eytzinger_waltz_sort;
use eytzinger::SliceExt;

fn main() {
    let mut nr_of_items = 2;
    let mut file_avg = OpenOptions::new()
        .truncate(true)
        .create(true)
        .write(true)
        .open("sort_bench_avg.csv")
        .unwrap();
    writeln!(file_avg, "items,stable,unstable,march,traverse,waltz,unstable+inplace,unstable+clone,eytz_search,eytz_search2,binary_search").expect("OK");
    for _ in 0..20 {
        nr_of_items <<= 1;
        let mut file = OpenOptions::new()
            .truncate(true)
            .create(true)
            .write(true)
            .open(format!("sort_bench_{}.csv", nr_of_items))
            .unwrap();
        writeln!(file, "stable,unstable,march,traverse,waltz,unstable+inplace,unstable+clone,eytz_search,eytz_search2,binary_search").expect("OK");
        let mut data = vec![0;10];
        let nr_of_samples = 20;
        for _ in 0..nr_of_samples {
            let mut vec_rnd = vec![];
            // push_random_strings(&mut vec_rnd, nr_of_items);
            push_random_u64(&mut vec_rnd, nr_of_items);
            let mut vec_stable = vec_rnd.clone();
            let mut vec_unstable = vec_rnd.clone();
            let mut vec_march = vec_rnd.clone();
            let mut vec_trav = vec_rnd.clone();
            let mut vec_waltz = vec_rnd.clone();
            let mut vec_inplace = vec_rnd.clone();

            let now = Instant::now();
            vec_stable.sort();
            let stable_sort_time = now.elapsed();

            let now = Instant::now();
            vec_unstable.sort_unstable();
            let unstable_sort_time = now.elapsed();

            let mut eytz_march_time = Duration::from_nanos(0);
            let mut eytz_traverse_time = Duration::from_nanos(0);
            let mut eytz_waltz_time = Duration::from_nanos(0);
            let mut eytz_inplace_time = Duration::from_nanos(0);

            // Stop measuring because the waiting time becomes way too long
            if nr_of_items < 10_000 {
                let now = Instant::now();
                eytzinger_march_sort(&mut vec_march);
                eytz_march_time = now.elapsed();

                let now = Instant::now();
                eytzinger_traversal_sort(&mut vec_trav);
                eytz_traverse_time = now.elapsed();

                let now = Instant::now();
                eytzinger_waltz_sort(&mut vec_waltz);
                eytz_waltz_time = now.elapsed();
            }

            if nr_of_items < 100_000 {
                let now = Instant::now();
                vec_inplace.sort_unstable();
                vec_inplace.eytzingerize(&mut eytzinger::permutation::InplacePermutator);
                eytz_inplace_time = now.elapsed();
            }

            let mut vec_tmp = vec_rnd.clone();
            let now = Instant::now();
            vec_tmp.sort_unstable();
            let vec_clone = clone_as_eytzinger(&vec_tmp);
            let eytzinger_clone_time = now.elapsed();

            let now = Instant::now();
            for (index, e) in vec_clone.iter().enumerate() {
                assert_eq!(first_index_for_eytzinger(&vec_clone, e).expect("What"), index)
            }
            let eytz_search = now.elapsed();

            let now = Instant::now();
            for (index, e) in vec_clone.iter().enumerate() {
                assert_eq!(vec_clone.eytzinger_search(e).expect("What"), index)
            }
            let eytz_search2 = now.elapsed();

            let now = Instant::now();
            for (index, e) in vec_unstable.iter().enumerate() {
                assert_eq!(vec_unstable.binary_search(e).expect("???"), index)
            }
            let binary_search = now.elapsed();

            writeln!(file, "{},{},{},{},{},{},{},{},{},{}",
                     stable_sort_time.as_nanos(),
                     unstable_sort_time.as_nanos(),
                     eytz_march_time.as_nanos(),
                     eytz_traverse_time.as_nanos(),
                     eytz_waltz_time.as_nanos(),
                     eytz_inplace_time.as_nanos(),
                     eytzinger_clone_time.as_nanos(),
                     eytz_search.as_nanos(),
                     eytz_search2.as_nanos(),
                     binary_search.as_nanos()).expect("");
            data[0] += stable_sort_time.as_nanos();
            data[1] += unstable_sort_time.as_nanos();
            data[2] += eytz_march_time.as_nanos();
            data[3] += eytz_traverse_time.as_nanos();
            data[4] += eytz_waltz_time.as_nanos();
            data[5] += eytz_inplace_time.as_nanos();
            data[6] += eytzinger_clone_time.as_nanos();
            data[7] += eytz_search.as_nanos();
            data[8] += eytz_search2.as_nanos();
            data[9] += binary_search.as_nanos();
        }
        writeln!(file_avg, "{},{},{},{},{},{},{},{},{},{},{}",
                 nr_of_items,
                 data[0] / nr_of_samples,
                 data[1] / nr_of_samples,
                 data[2] / nr_of_samples,
                 data[3] / nr_of_samples,
                 data[4] / nr_of_samples,
                 data[5] / nr_of_samples,
                 data[6] / nr_of_samples,
                 data[7] / nr_of_samples,
                 data[8] / nr_of_samples,
                 data[9] / nr_of_samples).expect("");
    }
}

fn push_random_strings(vec: &mut Vec<String>, nr_of_items: usize) {
    let mut rng = thread_rng();
    for _ in 0..nr_of_items {
        let rand_string: String = thread_rng()
            .sample_iter(&Alphanumeric)
            .take(((rng.next_u64() % 10) + 5) as usize)
            .collect();
        vec.push(rand_string)
    }
}

fn push_random_u64(vec: &mut Vec<u64>, nr_of_items: usize) {
    let mut rng = thread_rng();
    for _ in 0..nr_of_items {
        vec.push(rng.next_u64())
    }
}