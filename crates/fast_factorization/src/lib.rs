#[derive(Debug, Clone)]
pub struct FastFactrzation {
    sieve: Vec<Option<usize>>,
}

impl FastFactrzation {
    pub fn new(len: usize) -> Self {
        let mut sieve = vec![None; len];
        for p in (2..).take_while(|&p| p * p < len) {
            if sieve[p].is_some() {
                continue;
            }
            for i in seq::step(2 * p, p).take_while(|&i| i < len) {
                sieve[i] = Some(p);
            }
        }
        Self { sieve }
    }

    /// 素因数の列を降順で返します。
    /// 重複度をもっているものは、その回数だけ含みます。
    pub fn factorize(&self, x: u32) -> Vec<u32> {
        assert_ne!(x, 0, "0 を素因数分解をするのをやめましょう！");
        let mut x = x as usize;
        if x == 1 {
            Vec::new()
        } else {
            let mut res = Vec::new();
            loop {
                if let Some(p) = self.sieve[x] {
                    res.push(p as u32);
                    x /= p;
                } else {
                    res.push(x as u32);
                    return res;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::FastFactrzation;

    #[test]
    fn test_hand() {
        let a = FastFactrzation::new(300);
        assert_eq!(a.factorize(1), Vec::new());
        assert_eq!(a.factorize(2), vec![2]);
        assert_eq!(a.factorize(3), vec![3]);
        assert_eq!(a.factorize(4), vec![2, 2]);
        assert_eq!(a.factorize(5), vec![5]);
        assert_eq!(a.factorize(6), vec![3, 2]);
        assert_eq!(a.factorize(7), vec![7]);
        assert_eq!(a.factorize(8), vec![2, 2, 2]);
        assert_eq!(a.factorize(9), vec![3, 3]);
        assert_eq!(a.factorize(10), vec![5, 2]);
        assert_eq!(a.factorize(11), vec![11]);
        assert_eq!(a.factorize(12), vec![3, 2, 2]);
    }
}
