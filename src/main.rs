use std::sync::mpsc::channel;

use rayon::prelude::*;

fn rayon_join() {
    // This is a simple example of using rayon::join to run two closures in parallel.
    // The closures are passed to rayon::join as arguments.
    // The closures are executed in parallel and the results are returned as a tuple.

    // rayon::join takes two closures as arguments and returns a tuple of the results of the closures.
    // The closures are executed in parallel.

    let v1 = vec![1, 2, 3, 4, 5];
    let v2 = vec![6, 7, 8, 9, 10];

    let (sum1, sum2) = rayon::join(|| v1.par_iter().sum::<i32>(), || v2.par_iter().sum::<i32>());
    println!("sum1: {}, sum2: {}", sum1, sum2);
}

fn rayon_par_bridge() {
    // This is a simple example of using rayon::par_bridge to convert a parallel iterator to a sequential iterator.
    // The par_bridge method is used to convert a parallel iterator to a sequential iterator.
    // The par_bridge method is useful when you want to use a parallel iterator in a sequential context.

    // Sequential iterators are iterators that are executed sequentially.
    // Sequentially means one after the other.

    // Channels in rust are used to send data between threads.
    // A channel has two ends, a sender and a receiver.

    let rx = {
        let (tx, rx) = channel();
        (1..=3).into_iter().for_each(|i| {
            let _ = tx.send(i);
        });

        rx
    };

    let mut output: Vec<i32> = rx.into_iter().par_bridge().collect();
    output.sort_unstable();

    assert_eq!(&*output, &[1, 2, 3]);
    println!("output: {:?}", output);
}

fn main() {
    rayon_join();
    rayon_par_bridge();
}
