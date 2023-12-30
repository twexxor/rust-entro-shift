pub fn entro_shift(mut entropy: u32) -> u32 {
    entropy ^= (entropy << 14) ^ (entropy >> 13);
    entropy = (!(entropy ^ 1111111111)).wrapping_sub(!entropy << 5);
    entropy = (entropy << 31).wrapping_add(entropy >> 1);
    entropy = (entropy << 3).wrapping_add(entropy);
    entropy ^= 1111111111;
    entropy = !((entropy).wrapping_sub(entropy << 5));
    return (entropy >> 1) ^ entropy;
}
