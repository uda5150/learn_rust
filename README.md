### learn_rust
Rust学習量ディレクトリ  
参考：[プログラミング言語 Rust, 2nd Edition](https://doc.rust-jp.rs/the-rust-programming-language-ja/1.6/book/) 

### 学習メモ
#### 2章
- 変数
    - 変数は標準で不変(immutable)
    - 可変にしたい場合はmutを変数名前につける
    - 3章で出てくる
- 参照
    - &は参照
        - 参照とは・・・？
        - 4章で出てくる
    - これ、Rustの肝っぽいが、難しそう
- Result型
    - 列挙型
    - enum
        - 6章で出てくる
- クレート？？？
    - ライブラリのこと？
- トレイト？？？
    - 10章に出てくるらしい
- match
    - 6章と18章に出てくる
- シャドーイング
    - これって便利なのだろうか？
    - 混乱しそうなイメージ
    - 3章に出てくるとのこと
- 数値型
    - 3章
#### 3章（所有権）
- 変数
    - 基本は不変
    - mutをつけて可変に
    - 定数
        - letではなくconst
        - 値の型を必ず注釈する
        - 命名規則は全て大文字とアンダースコアで単語を区切る
- シャドーイング
    - 値に変更は行えるが、再代入ができるわけではない
    - letを使わないといけない
    - メリットが良くわからない・・・
        - 
- データ型
    - スカラー型
        - 整数型
            - 符号なし：u
            - 符号付き：i
            - 大きさ
                - 8bit：i8 or u8
                - arch：isize or usize
            - どの整数型を使うべきか
                - rustの基準はi32
        - 浮動小数点型
            - f32とf64がある
            - 基準はf64
        - 数値演算
            - 足し算、引き算、掛け算、割り算、余り
        - 論理値型
            - true, false
            - bool
        - 文字型
            - char型
            - 文字列は””だが、char型は’'
    - 複合型
        - タプル型
            - 複数の型を複合型としてまとめあげることができる
            - let tup: (i32, f64, u8) = (500, 6.4, 1)
            - 分配：パターンマッチングによる分解
                - let (x, y, z) = tup;
                - println!(“The value of y is: {}”, y)
            - 添字アクセス
                - let five_hundred = tup.0;
        - 配列型
            - 全要素、同じ型
            - 固定長
            - ヒープよりもスタックにデータのメモリを確保したいとき or 常に固定長の要素があることを確認したいときに有効
            - 配列とベクタ型をどちらを使っていいかわからないときはベクタ型を使う
                - monthのように変わらない要素ならば配列でおｋ
            - 要素へのアクセス
                - let a = [1, 2, 3];
                - let first = a[0];
            - 配列への無効なアクセス
                - パニックを起こす
- 関数
    - 命名規則はスネークケース(some_variable)
    - 関数は名前＋()で呼び出せる
    - どこかで定義すれば呼び出せる
    - 関数の引数
        - 仮引数は型を宣言しないといけない
        - 複数の仮引数を持たせたいときは仮引数の定義をカンマで区切る
    - 文と式
        - 文は何も返さない
        - 式は必ず値を返す
    - if式
        - 返す値の型は一致していること
        - 複数条件が扱いやすくなるmatchは6章
    - ループでの繰り返し=> これって全部mapとかで書き換えられしまうのでは？
        - loop
            - 明示的にやめさせるまで実行する
            - breakでやめさせられる
        - while
            - 条件が真の間、ループが走る
            - if, else, breakを使わなくて良いので、分かりやすい
        - for
            - コレクションを覗き見る
            - コレクションの長さに変更に強い
            - Rustでの使用頻度が一番高いループ
#### 4章
- 所有権
    - メモリはコンパイラがコンパイル時にチェックする一定の規則とともに所有権システムを通じて管理されている
    - 所有権の理解＝Rustの理解の礎
    - スタックとヒープ
        - Rustにおいては値がスタックに載るか、ヒープに載るかは重要
            - ？？？
        - スタック：last in first out
            - お皿の山のイメージ
            - スタックにPush：データの追加
            - スタックからPop ：データを取り除く
            - データへのアクセス方法のおかげでスタックは高速
            - スタック上のデータは全て既知の固定サイズにならなければいけない
        - ヒープ
            - コンパイル時にサイズがわからない or サイズが可変のときはヒープに格納する
            - ヒープにデータをおくとき、OSはヒープ上に十分な大きさの空の領域を見つけ、使用中にして、ポインタを返す
                - ポインタはその確保した場所のアドレス
                - この過程をヒープに領域を確保する：allocateという
            - レストランでの座席確保
        - 所有権が解決すること
            - どの部分のコードがヒープ上のデータを使用しているか把握すること
                - お客様がどのテーブルを使っているかを把握すること
            - ヒープ上の重複するデータを最小化すること
                - レストランのたとえだとムリ？
            - メモリ不足にならないようにヒープ上の未使用のデータを掃除すること
                - 机の掃除？
                - もっと良いたとえがありそう
    - 所有権規則
        - Rustの各値は、所有者と呼ばれる変数と対応している
        - いかなるときも所有者は一つである
        - 所有者がスコープから外れたら、値は破棄される
    - 変数とスコープ
        - 他の言語とそんな変わらない
    - String型
        - 8章で深く議論する
        - 文字列リテラルは不変
        - String型はユーザー入力を受け取るとき（長さが分からないとき）に便利
        - ヒープにメモリを確保する
    - メモリと確保
        - メモリは実行時にOSに要求される
            - String::from関数を読んだら、実装が必要なメモリを要求する
                - どうやって？？？
        - String型を使用し終わったら、OSにこのメモリを変換する方法が必要
            - GCはこれを勝手にやる
            - ない場合は、自分でメモリを開放する必要がある
        - Rustはメモリを所有している変数がスコープを抜けたら、自動でメモリを変換する
        - 変数がスコープを抜けるとき、drop関数を読んでメモリを変換している
        - ムーブ
            - ヒープ上のデータはコピーされない
            - スタック上のポインタ、長さ、許容量をコピーする
            - shallow copyの最初の変数を無効化する版がRustではムーブとして知られている
            - Rustではdeep copyが自動的に行われることはない
        - 既知のサイズを持つ型は全てスタック上に保持される
        - 型にCopy注釈をつけるには、継承可能トレイトを使う
    - 参照と借用（borrowing）
        - ＆で参照をつけることができる
        - - で参照を外せる（8章、15章）
        - 可変な参照
            - 変数をmutで可変に
            - &mutで可変な参照を作成できる
            - 制約：特定のスコープで特定のデータに対しては一つしか可変な参照を持てない
                - 利点：データ競合を防ぐことができる
            - データ競合はこれら3つの振る舞いが起きるときに発生する
                - 2つ以上のポインタが同じデータに同時にアクセスする
                - 少なくとも1つのポインタがデータに書き込みを行っている
                - データへのアクセスを同期する機構が使用されていない
            - スコープが違えば、複数の可変の参照を作成可能
            - 可変と不変の参照の組み合わせは不可
    - 宙に浮いた参照
        - ダングリングポインタ：他人に渡されてしまった可能性のあるメモリを指すポインタで、その箇所へのポインタを保持している間にメモリを開放することで発生する
        - Rustはそれを防げるようになっている
            - データ参照があった場合は、参照がスコープを抜けるまでデータがスコープを抜けることがないように確認してくれる
        - ライフタイムは10章
    - スライス
        - 良く分かってない・・・が大変なのは分かった
    - 文字列スライス
        - 
#### 5章（構造体）
- struct(構造体)とは、意味のあるグループを形成する複数の関連した値をまとめ、名前づけできる独自のデータ型
  - オブジェクトのデータ属性のようなもの
  - Haskellの型に命名できるやつと同じ？
  - 何が嬉しいのか？
- 構造体を利用するには、各フィールドに具体的な値を入れてインスタンスを生成する
  - mutで可変インスタンスを生成すれば、フィールドの値を後から変更できる
  - フィールドと変数名が同名の場合は、フィールド初期化省略記法が使える
- 構造体更新記法
  - user1で定義したものをuser2でも..user1という記法を使うと明示的にセットしなくても残りのフィールドがuser1と一緒になる
- タプル構造体
  - フィールドに紐付けられた名前がなく、型だけのものをタプル構造体
  - struct Color(i32, i32, i32);
- ユニット様構造体
  - 一切フィールドのない構造体
  - ある型にトレイトを実装するが、型自体に保持させるデータがないときに使う（トレイトは10章）
- メソッド記法
  - 構造体の文脈で定義される
  - 関数の代替としてメソッドを使う利点は２つ
    - 全メソッドのシグニチャでselfの型を繰り返す必要が無くなる
    - 体系化できるようになる
  - '->演算子'はないが自動参照、参照外しがある
    - CとC++の場合、オブジェクトのポインタに対してメソッドを呼び出し、先にポインタを参照外しする必要があるなら、->を使用する
    - Rustのコンパイラはobjectがメソッドのシグニチャと合致するように自動で&, &mut, *を付与する

#### 6章（Enumとパターンマッチング）
##### enum(列挙型)
- 取りうる値を列挙することで型を定義させてくれる
- nullは無く、Option enumがある
  - 値がnullである場合を明示的に処理する必要がある

##### match
- Rustのmatchは包括的で全ての可能性を網羅しないとコードが有効にならない
- 全ての可能性を列挙したくないときには、'_'が利用できる
- if letを使うと、_ => ()を毎回書かなくて良くなる
  - ただし、包括性チェックを失うので注意

##### メモ
- 構造体よりも拡張性があることはわかったが、じゃあ、構造体いらないのでは？とはならないのか？
- cargo でPJT作ってみようがなかったので、コードを書かないで終わってしまった・・・

#### 7章（モジュール）
- モジュールとは関数や型定義を含む名前空間のことで、それらの定義がモジュール外からも見えるようにするか否かを選択することができる
- モジュール規則
  - モジュールにサブモジュールがなければ、fooの定義はfoo.rsに書く
  - fooというモジュールにサブモジュールがある場合はfoo/mod.rsというファイルに書く
- プライバシー規則
  - 要素が公開であれば、どの親モジュールを通してもアクセス可
  - 要素が非公開なら、直接の親モジュールとその親の子モジュールのみアクセス可

##### メモ