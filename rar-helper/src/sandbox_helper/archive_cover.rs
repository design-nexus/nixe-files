// SPDX-License-Identifier: MIT
pub(super) const MAX_ENTRIES: usize = 4096;
pub(super) const MAX_IMAGE_BYTES: u64 = 8 * 1024 * 1024;

pub(super) fn image_name(name: &str) -> bool {
    matches!(
        name.rsplit('.')
            .next()
            .unwrap_or_default()
            .to_ascii_lowercase()
            .as_str(),
        "jpg" | "jpeg" | "png" | "webp" | "gif"
    )
}

pub(super) fn compare_names(left: &str, right: &str) -> std::cmp::Ordering {
    let (left, right) = (left.as_bytes(), right.as_bytes());
    let (mut l, mut r) = (0, 0);
    while l < left.len() && r < right.len() {
        if left[l].is_ascii_digit() && right[r].is_ascii_digit() {
            let start_l = l;
            let start_r = r;
            while l < left.len() && left[l].is_ascii_digit() {
                l += 1;
            }
            while r < right.len() && right[r].is_ascii_digit() {
                r += 1;
            }
            let a = left[start_l..l]
                .iter()
                .position(|c| *c != b'0')
                .map_or(l, |n| start_l + n);
            let b = right[start_r..r]
                .iter()
                .position(|c| *c != b'0')
                .map_or(r, |n| start_r + n);
            let order = (l - a)
                .cmp(&(r - b))
                .then_with(|| left[a..l].cmp(&right[b..r]));
            if !order.is_eq() {
                return order;
            }
        } else {
            let order = left[l]
                .to_ascii_lowercase()
                .cmp(&right[r].to_ascii_lowercase());
            if !order.is_eq() {
                return order;
            }
            l += 1;
            r += 1;
        }
    }
    left.len().cmp(&right.len()).then_with(|| left.cmp(right))
}

