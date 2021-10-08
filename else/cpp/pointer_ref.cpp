#include <bits/stdc++.h>
using namespace std;

void mult_by_2(int a) {
  cout << a << endl;
  a *= 2;
  cout << a << endl;
}

void mult_by_3(int *a) {
  cout << *a << endl;
  *a *= 3;
  cout << *a << endl;
}

void mult_by_4(int &a) {
  cout << a << endl;
  a *= 4;
  cout << a << endl;
}

int main() {
  int v = 16;

  // 値渡し, 関数の呼び出し元の値を変更しない
  // 関数の呼び出し元の値のコピーを渡している
  cout << v << endl;
  mult_by_2(v);
  cout << v << endl;

  cout << "##############################" << endl;

  // ポインタ渡し, 変数のメモリ上のアドレスを渡す
  // 関数の呼び出し元の値を書き換えることができる
  cout << v << endl;
  mult_by_3(&v);
  cout << v << endl;

  cout << "##############################" << endl;

  // 参照渡し, CにはなくC++で新たに追加された記法
  // 値渡しのように書くことができるが、ポインタ渡しの時と同様に
  // 関数の呼び出し元の値を書き換えることができる
  // ポインタ渡しとの違いは、参照は再割り当てができないのと、NULLを指定できない(初期化が必要)こと
  cout << v << endl;
  mult_by_4(v);
  cout << v << endl;
}
