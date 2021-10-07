#![feature(test)]

extern crate test;

#[cfg(test)]
mod tests {
    use test::Bencher;
    use rand::{thread_rng, RngCore};

    const ELEMENTS_COUNT: usize = 1000000;

    fn gen_data() -> Vec<(u64, u64)> {
        let mut data = Vec::with_capacity(ELEMENTS_COUNT);
        let mut rng = thread_rng();

        for _ in 0..ELEMENTS_COUNT {
            data.push((rng.next_u64(), rng.next_u64()))
        }

        data
    }

    #[bench]
    fn bench_std_hashmap(b: &mut Bencher) {
        let data = gen_data();
        let mut hashmap = std::collections::HashMap::with_capacity(ELEMENTS_COUNT);
        b.iter(|| {
            hashmap.clear();
            for el in data.iter() {
                *hashmap.entry(el.0).or_insert(el.1) += 1;
            }
        });
    }

    #[bench]
    fn bench_hb_hashmap(b: &mut Bencher) {
        let data = gen_data();
        let mut hashmap = hashbrown::HashMap::with_capacity(ELEMENTS_COUNT);
        b.iter(|| {
            hashmap.clear();
            for el in data.iter() {
                *hashmap.entry(el.0).or_insert(el.1) += 1;
            }
        });
    }
}