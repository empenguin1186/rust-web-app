




let record = ${MySQLから取得したデータ}
let depth = 1
let index = 0
let result: Tree{0番目のレコードから}

func create_tree(depth: &int, index: &int, record: Vec<CommentsPath>) -> Tree {
  
  // レコードが1つしかない場合、すぐに Leaf を返す
  if (record.size() == 1) {
    return Leaf{id: record[*index].id, comment: record[*index].comment};
  }

  if (*index == record.size()-1) {
    return Leaf{id: record[*index].id, comment: record[*index].comment};
  }
  
  if (*depth <= record[*index+1].path.size()) {
    // レコードが複数ある場合、Branch オブジェクト生成
    let result = Branch{id: record[*index].id, comment: record[*index].comment, vec![]};
    *depth = record[*index+1].path.size();
    *index = *index + 1;
    return result.add_children(create_tree(&depth, &index, record));
  } else {
    *depth = *depth - (1*2);
    *index = *index + 1;
    return Leaf{id: record[*index].id, comment: record[*index].comment};
  }
}
