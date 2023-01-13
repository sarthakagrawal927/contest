#include <vector>
#include <stdio.h>
using namespace std;

void print(vector<int> &a)
{
    for (const auto &item : a)
    {
        printf("%d", item);
    }
    printf("\n");
}

int main()
{
    vector<int> a;
    vector<int> b = a; // vector is copied

    b.push_back(1);

    printf("a");
    print(a);

    printf("b");
    print(b);
}