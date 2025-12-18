## 型・基本文法編
### HashSet
- forループでの存在確認にはこれ
- 集合型。
- 存在確認が$O(1)$で終わる。
- 重複不可。
- 長めのループで、データの存在確認に使える
#### 基本機能
- `.insert()`
  - HashSetに挿入する(順序なし)
  - 挿入できたかどうかを`bool`で返す
- `.contains()`
  - Vecと同じように参照を渡す。
  - ただし$O(1)$。
### HashMap
- 連想配列。
- Pythonでいう`dict`。
#### 基本機能
- `.insert()`
  - HashMapに挿入する。
  - 引数が2つ。
    - 1つ目はキー
    - 2つ目は値
  - キーが同じだと上書きする
- `.get()`
  - HashMapをキーから検索して、`Option<T>`で返す(Tは値の型)。
```rust
use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    map.insert("apple", 3);
    map.insert("banana", 5);

  if let Some(count) = map.get("apple") {
    println!("{}", count);
  }
}
```
## Algorithm
### DFS
- 別名: 深さ優先探索
- 別名2: Depth First Search
- 再帰で処理可能。
- 深さを優先して探索する手法。
- 順列全探索を再現できる。
- 「1つだけ答えよ」のような問題ではboolの返り値を設定してtrueを返したら速攻処理を中断するとよい。