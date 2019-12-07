#[derive(Debug)]
struct Star<'a> {
    name: &'a str,
    orbiters: Vec<Star<'a>>,
}

#[derive(Debug, Eq, PartialEq, Clone, PartialOrd, Ord)]
enum SearchResults<'a> {
    Santa(Vec<&'a str>),
    Me(Vec<&'a str>),
    TotalPath(Vec<&'a str>),
    TheEmptyVoidOfSpace,
}

impl<'a> std::ops::Add for SearchResults<'a> {
    type Output = Self;
    fn add(self, other: SearchResults<'a>) -> Self {
        use SearchResults::*;
        match self {
            Me(mut mp) => match other {
                Santa(sp) => {
                    mp.extend(sp);
                    TotalPath(mp)
                }
                TheEmptyVoidOfSpace => other,
                _ => panic!("This doesn't add up: Me + {:?}", other),
            },
            Santa(mut mp) => match other {
                Me(sp) => {
                    mp.extend(sp);
                    TotalPath(mp)
                }
                TheEmptyVoidOfSpace => other,
                _ => panic!("This doesn't add up: Santa + {:?}", other),
            },
            TheEmptyVoidOfSpace => other,
            _ => panic!("This doesn't add up: {:?} + {:?}", self, other),
        }
    }
}

impl<'a> std::ops::AddAssign<&'a str> for SearchResults<'a> {
    fn add_assign(&mut self, other: &'a str) {
        use SearchResults::*;
        match self {
            Me(p) => p.push(other),
            Santa(p) => p.push(other),
            _ => (),
        }
    }
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

    fn path_find(&'a self) -> Vec<&'a str> {
        match self._path_find() {
            SearchResults::TotalPath(p) => p,
            _ => vec![],
        }
    }

    fn _path_find(&self) -> SearchResults {
        match self.name {
            "YOU" => SearchResults::Me(vec![]),
            "SAN" => SearchResults::Santa(vec![]),
            n => {
                let mut r = SearchResults::TheEmptyVoidOfSpace;
                for s in self
                    .orbiters
                    .iter()
                    .map(Self::_path_find)
                    .filter(|x| *x != SearchResults::TheEmptyVoidOfSpace)
                {
                    r = r + s;
                }
                r += n;
                r
            }
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
    println!("{}", tree_space.pop().unwrap().path_find().len());
}
