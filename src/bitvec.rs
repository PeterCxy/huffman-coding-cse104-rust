/*
 * Extended methods for Vec<bool> as a vector of bits
 */
pub trait BitVec {
    fn copy_append(&self, val: bool) -> Self;
    fn to_binary(&self) -> String;
}

impl BitVec for Vec<bool> {
    fn copy_append(&self, val: bool) -> Vec<bool> {
        let mut n = self.clone();
        n.push(val);
        return n;
    }

    fn to_binary(&self) -> String {
        self.iter()
            .map(|i| if *i { "0" } else { "1" })
            .collect()
    }
}