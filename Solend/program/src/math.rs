pub const RAY: u128 = 1_000_000_000_000_000_000_000_000_000; // 1e27

#[inline]
pub fn ray_mul(a: u128, b: u128) -> u128 {
    (a.saturating_mul(b) + RAY / 2) / RAY
}

pub fn ray_div(a: u128, b: u128) -> u128 {
    (a.saturating_mul(RAY) + b / 2) / b
}
pub fn ray_pow(mut base: u128, mut exp: u64) -> u128 {
    let mut result = RAY;
    while exp > 0 {
        if exp & 1 == 1 {
            result = ray_mul(result, base);
        }
        base = ray_mul(base, base);
        exp >>= 1;
    }
    result
}

// converting anuall rate in bps to per-second in RAY
pub fn bps_per_year_to_sec_ray(bps_year: u16) -> u128 {
    let seconds_per_year: u128 = 31_536_000;
    let annual_ray = (bps_year as u128) * RAY / 10_000u128;
    annual_ray / seconds_per_year
}
