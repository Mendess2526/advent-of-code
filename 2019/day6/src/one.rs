#[derive(Debug)]
struct Star<'a> {
    name: &'a str,
    orbiters: Vec<Star<'a>>,
}

impl<'a> Star<'a> {
    fn new(name: &'a str) -> Self {
        Star {
            name,
            orbiters: vec![],
        }
    }

    fn with_child(name: &'a str, child: Star<'a>) -> Self {
        Star {
            name,
            orbiters: vec![child],
        }
    }

    fn total_orbits(&self) -> usize {
        self.total_orbits_internal(0).iter().sum()
    }

    fn total_orbits_internal(&self, h: usize) -> Vec<usize> {
        let mut a = vec![h];
        a.extend(
            self.orbiters
                .iter()
                .map(|t| t.total_orbits_internal(h + 1))
                .flatten(),
        );
        a
    }

    fn try_add(&mut self, parent: &'a str, child: &'a str) -> bool {
        if self.name == parent {
            self.orbiters.push(Star::new(child));
            true
        } else {
            self.orbiters
                .iter_mut()
                .map(|o| o.try_add(parent, child))
                .find(|b| *b)
                .unwrap_or(false)
        }
    }

    fn find(&mut self, name: &'a str) -> Option<&mut Star<'a>> {
        if self.name == name {
            Some(self)
        } else {
            self.orbiters.iter_mut().find_map(|t| t.find(name))
        }
    }
}

fn main() {
    let mut tree_space: Vec<Star> = Vec::new();
    include_str!("../input")
        .lines()
        .map(|l| l.split(')'))
        .map(|mut l| (l.next().unwrap(), l.next().unwrap()))
        .for_each(|(p, c)| {
            if let Some(i) = tree_space
                .iter()
                .enumerate()
                .find(|(_, x)| x.name == c)
                .map(|(i, _)| i)
            {
                // This p's child already exists
                let child = tree_space.swap_remove(i);
                if let Some(parent) = tree_space.iter_mut().find_map(|t| t.find(p)) {
                    // This p already exists, just join them toguether
                    parent.orbiters.push(child);
                } else {
                    // Push the exiting child down below a new parent
                    tree_space.push(Star::with_child(p, child));
                }
            } else {
                // This child doesn't exit at the top level
                if tree_space
                    .iter_mut()
                    .map(|t| t.try_add(p, c))
                    .all(|success| !success)
                {
                    // Nor does it exist at any level
                    tree_space.push(Star {
                        name: p,
                        orbiters: vec![Star::new(c)],
                    });
                }
            }
        });
    assert_eq!(tree_space.len(), 1);
    let orbits = tree_space[0].total_orbits();
    println!("total orb: {}", orbits);
}
