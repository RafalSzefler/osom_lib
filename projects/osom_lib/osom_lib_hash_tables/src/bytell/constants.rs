const RAW_JUMP_LEN: usize = 126;

/// Jump distances chosen like this:
/// 1. Pick the first 16 integers to promote staying in the same block.
/// 2. Add the next 66 triangular numbers to get even jumps when
///    the hash table is a power of two.
/// 3. Add 44 more triangular numbers at a much steeper growth rate
///    to get a sequence that allows large jumps so that a table
///    with 10000 sequential numbers doesn't endlessly re-allocate.
#[rustfmt::skip]
const RAW_JUMP_DISTANCES: [u64; RAW_JUMP_LEN] = [
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15,

    21, 28, 36, 45, 55, 66, 78, 91, 105, 120, 136, 153, 171, 190, 210, 231,
    253, 276, 300, 325, 351, 378, 406, 435, 465, 496, 528, 561, 595, 630,
    666, 703, 741, 780, 820, 861, 903, 946, 990, 1035, 1081, 1128, 1176,
    1225, 1275, 1326, 1378, 1431, 1485, 1540, 1596, 1653, 1711, 1770, 1830,
    1891, 1953, 2016, 2080, 2145, 2211, 2278, 2346, 2415, 2485, 2556,

    3741, 8385, 18915, 42486, 95703, 215496, 485605, 1091503, 2456436,
    5529475, 12437578, 27986421, 62972253, 141700195, 318819126, 717314626,
    1614000520, 3631437253, 8170829695, 18384318876, 41364501751,
    93070021080, 209407709220, 471167588430, 1060127437995, 2385287281530,
    5366895564381, 12075513791265, 27169907873235, 61132301007778,
    137547673121001, 309482258302503, 696335090510256, 1566753939653640,
    3525196427195653, 7931691866727775, 17846306747368716,
    40154190394120111, 90346928493040500, 203280588949935750,
    457381324898247375, 1029107980662394500, 2315492957028380766,
    5209859150892887590,
];

pub const JUMP_DISTANCES: [usize; RAW_JUMP_LEN] = const {
    assert!(RAW_JUMP_LEN < 65536);
    let mut result = [0usize; RAW_JUMP_LEN];
    let mut index = 0;

    while index < RAW_JUMP_LEN {
        #[allow(clippy::cast_possible_truncation)]
        {
            result[index] = RAW_JUMP_DISTANCES[index] as usize;
        }
        index += 1;
    }

    result
};
