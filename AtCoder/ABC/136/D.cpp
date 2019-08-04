#include<iostream>
#include<string>
#include<vector>
using namespace std;

// 移動が10^100回なので、普通の手段では無理
// 状態遷移でループ的なもの見つければ繰り返し省略できる？
// どう見つければいいのかわからないが

// 10^100回の繰り返しだと、高々10^5の長さの文字列では、LRの変わり目に数字が集中することになる。

/**
 * LR: 最終的に必ず00
 * RL: 常に値を交換
 * RR: 0x
 * LL: x0
 * 
 * 交換が曲者。左右から運ばれてくる値が合計にならず交換位置で左右にばらける
 */


int main(){
    cin.tie(0);
    ios::sync_with_stdio(false);

    string S;   // len = 10^5
    cin >> S;
    int N = (int)S.size();

    vector<int> src(N, 1), dst(N, 0), prv(N, 0), pprv(N, 0);

    int count = 0;
    while(true){
        count++;
        pprv = prv;
        prv = src;

        for(int j=0; j<N; j++){
            if(src[j] == 0)
                continue;
            if(S[j] == 'R')
                dst[j+1]+=src[j];
            else
                dst[j-1]+=src[j];
        }
        src = dst;
        for(int j=0; j<N; j++){
            dst[j] = 0;
        }

        if(src == pprv)
            break;
    }

    vector<int> &res = src;
    if(count % 2 != 0)
        res = prv;

    for(int i=0; i<N; i++){
        if(i != 0)
            cout << " ";
        cout << res[i];
    }
    cout << endl;

    return 0;
}