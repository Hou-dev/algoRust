#include <iostream>
#include <array>

using namespace std;

int takeProfit(int cap,array<int,5> weight, array<int,5> priceList, int arraySize){
    //Base Case
    if (cap == 0 || arraySize == 0) {
        return 0;
    }
    //If the item is more than the knapsack can handle then it will not be includede
    else if (weight[arraySize - 1] > cap){
        return takeProfit(cap, weight, priceList, arraySize - 1 );
    }
    //return the max
    else{
        return max(
            priceList[arraySize -1] + takeProfit(cap - weight[arraySize -1], 
            weight, priceList, arraySize -1), 
            takeProfit(cap, weight, priceList, arraySize -1));
    }
}

int main(){
    int cap = 11;
    array<int,5> weight{1,2,5,6,7};
    array<int,5> priceList{1,6,18,22,28};
    int arraySize = priceList.size();
    cout << takeProfit(cap,weight, priceList, arraySize) << "\n";
    // std::cout << "hello \n" << arraySize;
    return 0;
}