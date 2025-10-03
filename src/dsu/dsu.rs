/**

new:
 0  1  2  3  4
[0, 0, 0, 0, 0]

Self reference
 0> 1> 2> 3> 4>
[0, 1, 2, 3, 4] union ops
[0, 0, 2, 3, 4]
[0, 0, 2, 3, 3]


*/

pub struct Dsu {
    pub parent: Vec<i32>,
}

impl Dsu {
    pub fn new(size: usize) -> Self {
        Self {
            parent: vec![0; size],
        }
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
        print!("{:?}", dsu_set.parent);
        dsu_set.make_set(0);
        assert_eq!(dsu_set.find_set(0), *dsu_set.parent.iter().next().unwrap());
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
