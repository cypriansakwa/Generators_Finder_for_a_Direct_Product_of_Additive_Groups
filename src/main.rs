struct ZnZm {
    n: u32,
    m: u32,
}

impl ZnZm {
    fn new(n: u32, m: u32) -> Self {
        ZnZm { n, m }
    }

    fn find_generators(&self) -> Vec<(u32, u32)> {
        let mut generators = Vec::new();
        
        for i in 0..self.n {
            for j in 0..self.m {
                if self.is_generator(i, j) {
                    generators.push((i, j));
                }
            }
        }

        generators
    }

    fn is_generator(&self, a: u32, b: u32) -> bool {
        let mut visited = vec![vec![false; self.m as usize]; self.n as usize];
        let mut count = 0;

        let mut x = 0;
        let mut y = 0;

        loop {
            if visited[x as usize][y as usize] {
                break;
            }

            visited[x as usize][y as usize] = true;
            count += 1;

            x = (x + a) % self.n;
            y = (y + b) % self.m;
        }

        count == (self.n * self.m)
    }
}

fn main() {
    let group = ZnZm::new(3, 8);
    let generators = group.find_generators();

    println!("Generators of Z4 x Z6: {:?}", generators);
}

