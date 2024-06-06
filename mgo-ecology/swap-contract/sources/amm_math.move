module mango_amm::amm_math {
    public fun safe_mul_div_u64(x: u64, y: u64, z: u64): u64 {
        ((x as u128) * (y as u128) / (z as u128) as u64)
    }

    public fun safe_compare_mul_u64(a1: u64, b1: u64, a2: u64, b2: u64): bool {
        let left = (a1 as u128) * (b1 as u128);
        let right = (a2 as u128) * (b2 as u128);
        left >= right
    }

    public fun safe_mul_u64(x: u64, y: u64): u64 {
        ((x as u128) * (y as u128) as u64)
    }
}