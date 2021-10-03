use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Item {
    comment: String,
    author: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub enum Tree {
    Leaf { item: Item },
    Branch { item: Item, children: Vec<Tree> },
}

impl Tree {
    pub fn new() -> Tree {
        Tree::Leaf {
            item: Item {
                comment: String::from(""),
                author: String::from(""),
            },
        }
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    pub fn test() {
        let tree = Tree::Leaf {
            item: Item {
                comment: String::from("hoge"),
                author: String::from("A"),
            },
        };
        println!("tree: {:?}", tree);
    }
}
