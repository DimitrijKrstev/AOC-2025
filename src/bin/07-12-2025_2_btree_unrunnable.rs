use ::aoc2025::read_lines;

fn main() {
    let lines: Vec<String> = read_lines("inputs/07-12-2025.txt");
    let start_col: usize = lines.first().unwrap().find('S').unwrap();

    let tree: BST = build_tree(&lines, 1, start_col);

    println!("{}", tree.count_leaves());
}

struct BST {
    left: Option<Box<BST>>,
    right: Option<Box<BST>>,
}

impl BST {
    fn new() -> Self {
        BST {
            left: None,
            right: None,
        }
    }

    fn count_leaves(&self) -> usize {
        match (&self.left, &self.right) {
            (None, None) => 1,
            (Some(left), None) => left.count_leaves(),
            (None, Some(right)) => right.count_leaves(),
            (Some(left), Some(right)) => left.count_leaves() + right.count_leaves(),
        }
    }
}

fn build_tree(lines: &[String], row: usize, col: usize) -> BST {
    if row >= lines.len() || col >= lines[row].len() {
        return BST::new();
    }

    let ch = lines[row].chars().nth(col).unwrap();

    match ch {
        '^' => {
            let mut node: BST = BST::new();
            if col > 0 {
                node.left = Some(Box::new(build_tree(lines, row + 1, col - 1)));
            }

            node.right = Some(Box::new(build_tree(lines, row + 1, col + 1)));
            node
        }
        _ => build_tree(lines, row + 1, col),
    }
}
