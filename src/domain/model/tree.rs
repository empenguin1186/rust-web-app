use serde::{Deserialize, Serialize};

use crate::models::CommentPE;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Item {
    comment: String,
    author: u64,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(untagged)]
pub enum Tree {
    Leaf { item: Item },
    Branch { item: Item, children: Vec<Tree> },
}

impl Tree {
    pub fn new(comments: &Vec<CommentPE>) -> Tree {
        let mut index = 0 as usize;
        let mut depth = comments.get(index).unwrap().path.as_ref().unwrap().len();
        Tree::create_tree(&mut depth, &mut index, comments)
    }

    fn add_child(&mut self, tree: Tree) {
        if let Tree::Branch { item: _, children } = self {
            children.push(tree);
        }
    }

    fn create_tree(depth: &mut usize, index: &mut usize, comments: &Vec<CommentPE>) -> Tree {
        // レコードが1つしかないもしくは最後の要素の場合、すぐに Leaf を返す
        if comments.len() == 1 || *index == comments.len() - 1 {
            let author = comments.get(*index).unwrap().author;
            let comment = &comments.get(*index).unwrap().comment;
            *depth = 2 as usize;
            return Tree::Leaf {
                item: Item {
                    comment: comment.to_string(),
                    author: author,
                },
            };
        }

        let author = comments.get(*index).unwrap().author;
        let comment = &comments.get(*index).unwrap().comment;
        let cur_depth = comments.get(*index).unwrap().path.as_ref().unwrap().len();
        if cur_depth
            < comments
                .get(*index + 1)
                .unwrap()
                .path
                .as_ref()
                .unwrap()
                .len()
        {
            *index = *index + 1;
            *depth = comments.get(*index).unwrap().path.as_ref().unwrap().len();

            let mut branch = Tree::Branch {
                item: Item {
                    comment: comment.to_string(),
                    author: author,
                },
                children: vec![],
            };

            if let Tree::Branch { .. } = branch {
                while *depth > cur_depth {
                    branch.add_child(Tree::create_tree(depth, index, comments));
                }
            }
            return branch;
        } else {
            let leaf = Tree::Leaf {
                item: Item {
                    comment: comment.to_string(),
                    author: author,
                },
            };
            *index = *index + 1;
            *depth = comments.get(*index).unwrap().path.as_ref().unwrap().len();
            return leaf;
        }
    }
}

#[cfg(test)]
mod test {
    use parameterized::parameterized;

    use super::*;

    #[parameterized(
        input = {
            vec![
                CommentPE {
                    id: 1,
                    path: Some(String::from("1/")),
                    author: 1,
                    comment: String::from("hoge"),
                },
                CommentPE {
                    id: 2,
                    path: Some(String::from("1/2/")),
                    author: 2,
                    comment: String::from("fuga"),
                },
                CommentPE {
                    id: 3,
                    path: Some(String::from("1/3/")),
                    author: 3,
                    comment: String::from("piyo"),
                }
            ],
            vec![
                CommentPE {
                    id: 1,
                    path: Some(String::from("1/")),
                    author: 1,
                    comment: String::from("hoge"),
                },
                CommentPE {
                    id: 2,
                    path: Some(String::from("1/2/")),
                    author: 2,
                    comment: String::from("fuga"),
                },
                CommentPE {
                    id: 3,
                    path: Some(String::from("1/2/3/")),
                    author: 3,
                    comment: String::from("piyo"),
                },
                CommentPE {
                    id: 4,
                    path: Some(String::from("1/4/")),
                    author: 4,
                    comment: String::from("hogehoge"),
                },
                CommentPE {
                    id: 5,
                    path: Some(String::from("1/4/5/")),
                    author: 5,
                    comment: String::from("fugafuga"),
                },
                CommentPE {
                    id: 6,
                    path: Some(String::from("1/4/5/6/")),
                    author: 6,
                    comment: String::from("piyopiyo"),
                },
                CommentPE {
                    id: 7,
                    path: Some(String::from("1/4/7/")),
                    author: 7,
                    comment: String::from("hogehogehoge"),
                },
            ]
        },
        expected = {
            Tree::Branch {
                item: Item {
                    comment: String::from("hoge"),
                    author: 1,
                },
                children: vec![
                    Tree::Leaf {
                        item: Item{comment: String::from("fuga"), author: 2}
                    },
                    Tree::Leaf {
                        item: Item{comment: String::from("piyo"), author: 3},
                    }
                ]
            },
            Tree::Branch {
                item: Item {
                    comment: String::from("hoge"),
                    author: 1,
                },
                children: vec![
                    Tree::Branch {
                        item: Item {
                            comment: String::from("fuga"),
                            author: 2,
                        },
                        children: vec![
                            Tree::Leaf{item: Item{comment: String::from("piyo"), author: 3}}
                        ]
                    },
                    Tree::Branch {
                        item: Item {
                            comment: String::from("hogehoge"),
                            author: 4,
                        },
                        children: vec![
                            Tree::Branch {
                                item: Item{comment: String::from("fugafuga"), author: 5},
                                children: vec![
                                    Tree::Leaf{item: Item{comment: String::from("piyopiyo"), author: 6}}
                                ]
                            },
                            Tree::Leaf {
                                item: Item{comment: String::from("hogehogehoge"), author: 7}
                            },
                        ]
                    },
                ]
            }
        })]
    pub fn test(input: Vec<CommentPE>, expected: Tree) {
        let mut depth = 2 as usize;
        let mut index = 0 as usize;
        assert_eq!(Tree::create_tree(&mut depth, &mut index, &input), expected);
    }
}
