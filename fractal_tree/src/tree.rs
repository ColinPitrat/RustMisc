use branch::Branch;
use dc::DrawingContext;
use leaf::Leaf;
use point::Point;

#[derive(Debug)]
pub struct TreeParams {
    pub ratio: f64,
    pub angle: f64,
    pub size: f64,
    pub randomness: f64,
    pub leaves: bool,
    pub max_generations: isize,
}

pub struct Tree {
    branches: Vec<Branch>,
    leaves: Vec<Leaf>,
}

impl Tree {
    pub fn new(tree_params: &TreeParams, root: Point) -> Tree {
        //println!("Create tree with {:#?}", tree_params);
        let mut branches = vec!(
                Branch::new(
                    Point::new(root.x, root.y),
                    Point::new(root.x, root.y - tree_params.size),
                    tree_params.randomness)
                );
        loop {
            let mut to_add = vec!();
            for branch in branches.iter_mut() {
                to_add.extend(branch.children(tree_params.ratio, tree_params.angle, tree_params.max_generations));
            }
            if to_add.is_empty() {
                break;
            }
            branches.extend(to_add);
        }
        let mut leaves = vec!();
        if tree_params.leaves {
            for b in branches.iter() {
                leaves.extend(b.leaf());
            }
        }
        Tree{branches, leaves}
    }

    pub fn falling_leaves(&mut self) {
        for l in self.leaves.iter_mut() {
            l.fall();
        }
    }

    pub fn animate(&mut self) {
        for l in self.leaves.iter_mut() {
            l.animate();
        }
    }

    pub fn display(&self, dc: &mut DrawingContext) {
        for b in self.branches.iter() {
            b.display(dc)
        }
        for l in self.leaves.iter() {
            l.display(dc)
        }
    }
}
