use bit_vec::BitVec;

/*
 * Extended methods for Vec<bool> as a vector of bits
 */
pub trait MyBitVec {
    fn copy_append(&self, val: bool) -> Self;
    fn to_binary(&self) -> String;
}

impl MyBitVec for BitVec<u32> {
    fn copy_append(&self, val: bool) -> BitVec<u32> {
        let mut n = self.clone();
        n.push(val);
        return n;
    }

    fn to_binary(&self) -> String {
        self.iter()
            .map(|i| if i { "1" } else { "0" })
            .collect()
    }
}