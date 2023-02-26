pub struct RailFence {
    n_rails: usize,
}

impl RailFence {
    pub fn new(n_rails: u32) -> RailFence {
        RailFence {
            n_rails: n_rails as usize,
        }
    }

    pub fn encode(&self, text: &str) -> String {
        if self.n_rails == 1 {
            return String::from(text);
        }
        let mut rails = Vec::with_capacity(self.n_rails);
        for _ in 0..self.n_rails {
            rails.push(String::new());
        }
        let mut rail = 0;
        let mut downwards = true;
        for c in text.chars() {
            rails[rail].push(c);
            if rail == rails.len() - 1 {
                downwards = false;
            } else if rail == 0 {
                downwards = true;
            }
            if downwards {
                rail += 1;
            } else {
                rail -= 1;
            }
        }
        rails.join("")
    }

    pub fn decode(&self, cipher: &str) -> String {
        let mut ret = Vec::new();
        let n = cipher.len();
        ret.resize(n, ' ');
        let step = 2 * self.n_rails - 2;
        let mut it = cipher.chars();
        // first rail
        for i in (0..n).step_by(step) {
            ret[i] = it.next().unwrap();
        }
        // middle rails
        for rail in 1..(self.n_rails - 1) {
            for i in (0..n).step_by(step) {
                if i + rail < n {
                    ret[i + rail] = it.next().unwrap();
                }
                if i + step - rail < n {
                    ret[i + step - rail] = it.next().unwrap();
                }
            }
        }
        // last rail
        let rail = self.n_rails - 1;
        for i in (rail..n).step_by(step) {
            ret[i] = it.next().unwrap();
        }

        ret.iter().collect()
    }
}
