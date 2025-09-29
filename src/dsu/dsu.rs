pub struct Dsu {
    pub parent: [i32; 10],
}

impl Dsu {
    pub fn new(_size: usize) -> Self {
        Self { parent: [0; 10] }
    }

    pub fn make_set(&mut self, v: i32) {
        self.parent[v as usize] = v;
    }

    pub fn find_set(&self, v: i32) -> i32 {
        if v == self.parent[v as usize] {
            v
        } else {
            self.find_set(self.parent[v as usize])
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_element() {
        let mut dsu_set: Dsu = Dsu::new(20);
        dsu_set.make_set(20);
        assert_eq!(dsu_set.find_set(20), *dsu_set.parent.iter().next().unwrap());
    }

    #[test]
    fn merge_two_sets() {
        assert!(todo!());
    }

    #[test]
    fn find_representative() {
        assert!(todo!());
    }
}
