use num_cpus;

pub struct Util {}

impl Util {
    pub fn guess_parallelism() -> u64 {
        match num_cpus::get() {
            0 | 1 => 2,
            2 => 3,
            val => (val + 2).try_into().unwrap(),
        }
    }
}
