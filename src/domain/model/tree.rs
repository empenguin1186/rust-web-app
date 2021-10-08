use crate::models::CommentPE;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Item {
    comment: String,
    author: i32,
}

#[derive(Deserialize, Serialize, Debug)]
pub enum Tree {
    Leaf { item: Item },
    Branch { item: Item, children: Vec<Tree> },
}

impl Tree {
    pub fn new(comments: CommentPE) -> Tree {
        Tree::Leaf {
            item: Item {
                comment: String::from(""),
                author: 0,
            },
        }
    }

    fn create_tree(depth: &mut usize, index: &mut usize, comments: Vec<CommentPE>) -> Tree {
        // レコードが1つしかないもしくは最後の要素の場合、すぐに Leaf を返す
        if comments.len() == 1 || *index == comments.len() - 1 {
            let author = comments.get(0).unwrap().author;
            let comment = comments.get(0).unwrap().comment;
            return Tree::Leaf {
                item: Item {
                    comment: comment,
                    author: author,
                },
            };
        }

        let author = comments.get(*index).unwrap().author;
        let comment = comments.get(*index).unwrap().comment;
        if *depth < comments.get(*index + 1).unwrap().path.len() {
            *index = *index + 1;
            *depth = comments.get(*index).unwrap().path.len();
            let result = Tree::Branch {
                item: Item {
                    comment: comment,
                    author: author,
                },
                children: vec![Tree::create_tree(depth, index, comments)],
            };
            return result;
        } else {
            let result = Tree::Leaf {
                item: Item {
                    comment: comment,
                    author: author,
                },
            };
            *index = *index + 1;
            *depth = comments.get(*index).unwrap().path.len();
            return result;
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
                author: 1,
            },
        };
        println!("tree: {:?}", tree);
    }
}
