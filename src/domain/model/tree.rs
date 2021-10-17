use serde::{Deserialize, Serialize};

use crate::models::CommentPE;

/// コメント内容とそのコメントの投稿者の情報を含む構造体
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Item {
    comment: String,
    author: u64,
}

/// 階層構造のデータを定義する列挙型
#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(untagged)]
pub enum Tree {
    /// 階層構造のデータにおける末端の要素
    Leaf { item: Item },
    /// 階層構造のデータにおける子要素を持つ要素
    Branch { item: Item, children: Vec<Tree> },
}

impl Tree {
    /// 与えられたデータに対応した Tree を生成する
    /// # Arguments
    /// * `comments` - `CommentsPE` テーブルから取得したレコード群
    pub fn new(comments: &Vec<CommentPE>) -> Tree {
        let mut index = 0 as usize;
        let mut depth = comments.get(index).unwrap().path.as_ref().unwrap().len();
        Tree::create_tree(&mut depth, &mut index, comments)
    }

    /// Branch に子要素を追加する
    /// # Arguments
    /// * `tree` - Branch に追加する子要素
    fn add_child(&mut self, tree: Tree) {
        if let Tree::Branch { item: _, children } = self {
            children.push(tree);
        }
    }

    /// Tree を生成する
    /// # Arguments
    /// * `depth` - 現在注目している要素の深さ(pathの文字数で表現)
    /// * `index` - 現在注目している要素の comments における index 番号
    /// * `comments` - `CommentsPE` テーブルから取得したレコード群
    fn create_tree(depth: &mut usize, index: &mut usize, comments: &Vec<CommentPE>) -> Tree {
        // レコードが1つしかないもしくは最後の要素の場合、すぐに Leaf を返す
        if comments.len() == 1 || *index == comments.len() - 1 {
            let author = comments.get(*index).unwrap().author;
            let comment = &comments.get(*index).unwrap().comment;
            *depth = 2 as usize;
            return Tree::Leaf {
                item: Item {
                    comment: comment.to_string(),
                    author,
                },
            };
        }

        let author = comments.get(*index).unwrap().author;
        let comment = &comments.get(*index).unwrap().comment;
        let cur_depth = comments.get(*index).unwrap().path.as_ref().unwrap().len();
        return if cur_depth
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

            // この時点で子要素が存在するので Branch を作成する
            let mut branch = Tree::Branch {
                item: Item {
                    comment: comment.to_string(),
                    author,
                },
                children: vec![],
            };

            // 子要素が存在する限り探索を続ける
            if let Tree::Branch { .. } = branch {
                while *depth > cur_depth {
                    branch.add_child(Tree::create_tree(depth, index, comments));
                }
            }
            branch
        } else {
            // この時点で末端の要素だと判明しているので Leaf を返す
            let leaf = Tree::Leaf {
                item: Item {
                    comment: comment.to_string(),
                    author,
                },
            };
            *index = *index + 1;
            *depth = comments.get(*index).unwrap().path.as_ref().unwrap().len();
            leaf
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
        let mut depth = input.len();
        let mut index = 0 as usize;
        assert_eq!(Tree::create_tree(&mut depth, &mut index, &input), expected);
    }
}
