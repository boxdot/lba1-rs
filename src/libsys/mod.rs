pub fn decompress_lzs(src: &[u8], dst: &mut [u8]) {
    let mut decompressed_len = dst.len();
    let mut bits = 1; // bit checked in the mask
    let mut mask = 0; // mask for 8 compressed/non-compressed blocks

    let mut src_idx = 0;
    let mut dst_idx = 0;

    while decompressed_len > 0 {
        if bits == 1 {
            // load mask byte
            mask = src[src_idx];
            src_idx += 1;
        }

        if mask & bits == 0 {
            // compressed
            let offset = u16::from_le_bytes([src[src_idx], src[src_idx + 1]]) as usize;
            src_idx += 2;

            let len = (offset & 0xF) + 2;
            let displacement = offset >> 4;

            let data_idx = dst_idx - displacement - 1;

            if offset == 0 {
                let data = dst[data_idx];
                dst[dst_idx..dst_idx + len].fill(data);
            } else {
                if dst_idx <= data_idx + len {
                    // overlapping
                    for idx in 0..len {
                        dst[dst_idx + idx] = dst[data_idx + idx];
                    }
                } else {
                    // non-overlapping
                    let (lhs, rhs) = dst.split_at_mut(dst_idx);
                    rhs[0..len].copy_from_slice(&lhs[data_idx..data_idx + len]);
                }
            }
            dst_idx += len;
            decompressed_len -= len;
        } else {
            // non-compressed
            dst[dst_idx] = src[src_idx];
            src_idx += 1;
            dst_idx += 1;
            decompressed_len -= 1;
        }

        bits = bits.rotate_left(1);
    }
}
