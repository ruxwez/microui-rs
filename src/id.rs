#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Id(pub u32);

const FNV_OFFSET: u32 = 2166136261;
const FNV_PRIME: u32 = 16777619;

pub fn fnv1a(seed: Id, data: &[u8]) -> Id {
    let mut h = seed.0;
    for &b in data {
        h ^= b as u32;
        h = h.wrapping_mul(FNV_PRIME);
    }
    Id(h)
}
