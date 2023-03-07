#![allow(non_upper_case_globals)]
use common_primitives::types::Balance;

pub const WSP: Balance = 1_000_000_000_000_000_000; // 18 decimal
pub const cWSP: Balance = WSP / 100; // 16 decimal, cent-WSP
pub const mWSP: Balance = WSP / 1_000; //15 decimal, milli-WSP
pub const uWSP: Balance = WSP / 1_000_000; // 12 decimal, micro-WSP

pub const fn deposit(items: u32, bytes: u32) -> Balance {
    items as Balance * 15 * mWSP + (bytes as Balance) * 6 * mWSP // TODO: revisit the storage cost here
}
