const H: [u32; 8] = [
    0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a, 0x510e527f, 0x9b05688c, 0x1f83d9ab, 0x5be0cd19,
];

const K: [u32; 64] = [
    0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4, 0xab1c5ed5,
    0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3, 0x72be5d74, 0x80deb1fe, 0x9bdc06a7, 0xc19bf174,
    0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc, 0x2de92c6f, 0x4a7484aa, 0x5cb0a9dc, 0x76f988da,
    0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7, 0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967,
    0x27b70a85, 0x2e1b2138, 0x4d2c6dfc, 0x53380d13, 0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85,
    0xa2bfe8a1, 0xa81a664b, 0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070,
    0x19a4c116, 0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3,
    0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208, 0x90befffa, 0xa4506ceb, 0xbef9a3f7, 0xc67178f2,
];

pub const fn sha256(input: [u8; 32]) -> [u8; 32] {
    let mut chunk = [0; 64];

    let mut i = 0;
    while i < 32 {
        chunk[i] = input[i];
        i += 1;
    }

    chunk[32] = 128;
    chunk[62] = 1;

    let mut w = [0_u32; 64];

    let mut i = 0;
    while i < 16 {
        w[i] = u32::from_be_bytes([
            chunk[4 * i],
            chunk[4 * i + 1],
            chunk[4 * i + 2],
            chunk[4 * i + 3],
        ]);
        i += 1;
    }

    let mut i = 0;
    while i < 48 {
        w[i + 16] = w[i]
            .wrapping_add(w[i + 1].rotate_right(7) ^ w[i + 1].rotate_right(18) ^ (w[i + 1] >> 3))
            .wrapping_add(w[i + 9])
            .wrapping_add(
                w[i + 14].rotate_right(17) ^ w[i + 14].rotate_right(19) ^ (w[i + 14] >> 10),
            );
        i += 1;
    }

    let mut h = H;

    let mut i = 0;
    while i < 64 {
        let t = h[7]
            .wrapping_add(h[4].rotate_right(6) ^ h[4].rotate_right(11) ^ h[4].rotate_right(25))
            .wrapping_add((h[4] & h[5]) ^ ((!h[4]) & h[6]))
            .wrapping_add(K[i])
            .wrapping_add(w[i]);
        let s0 = h[0].rotate_right(2) ^ h[0].rotate_right(13) ^ h[0].rotate_right(22);
        let maj = (h[0] & h[1]) ^ (h[0] & h[2]) ^ (h[1] & h[2]);

        h[7] = h[6];
        h[6] = h[5];
        h[5] = h[4];
        h[4] = h[3].wrapping_add(t);
        h[3] = h[2];
        h[2] = h[1];
        h[1] = h[0];
        h[0] = t.wrapping_add(s0.wrapping_add(maj));

        i += 1;
    }

    let mut i = 0;
    while i < 8 {
        h[i] = h[i].wrapping_add(H[i]);
        i += 1;
    }

    let h = [
        h[0].to_be_bytes(),
        h[1].to_be_bytes(),
        h[2].to_be_bytes(),
        h[3].to_be_bytes(),
        h[4].to_be_bytes(),
        h[5].to_be_bytes(),
        h[6].to_be_bytes(),
        h[7].to_be_bytes(),
    ];
    [
        h[0][0], h[0][1], h[0][2], h[0][3], h[1][0], h[1][1], h[1][2], h[1][3], h[2][0], h[2][1],
        h[2][2], h[2][3], h[3][0], h[3][1], h[3][2], h[3][3], h[4][0], h[4][1], h[4][2], h[4][3],
        h[5][0], h[5][1], h[5][2], h[5][3], h[6][0], h[6][1], h[6][2], h[6][3], h[7][0], h[7][1],
        h[7][2], h[7][3],
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    use ring::digest::{digest, SHA256};

    #[test]
    fn basic_hash() {
        let input: [u8; 32] = [
            119, 97, 107, 101, 32, 117, 112, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0,
        ];

        let result_a = sha256(input);
        let result_b = digest(&SHA256, &input);

        assert_eq!(result_a, result_b.as_ref());
    }
}
