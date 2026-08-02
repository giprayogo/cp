#include <iostream>
#include <vector>

using namespace std;

class Solution
{
public:
    // vector<int> minBitwiseArray(vector<int> &nums)
    // {
    //     for (auto i = 0; i < nums.size(); ++i)
    //     {
    //         auto lsz = 0;

    //         for (auto j = 0; j < 32; j++)
    //         {
    //             if ((nums[i] & (1 << j)) == 0)
    //             {
    //                 lsz = j;
    //                 break;
    //             }
    //         }

    //         if (lsz == 0)
    //         {
    //             nums[i] = -1;
    //         }
    //         else
    //         {
    //             nums[i] = nums[i] & ~(1 << (lsz - 1));
    //         }
    //     }

    //     return nums;
    // }
    vector<int> minBitwiseArray(vector<int> &nums)
    {
        for (auto &x : nums)
        {
            if (x % 2)
            {
                // auto check = (~((x & (~x - 1) + 1) >> 1));
                // auto check = (x & (~x - 1) + 1);
                auto check = x & ~(((x & (~x - 1)) + 1) >> 1);
                cout << x << ' ' << check << '\n';
                x = x & (~(((x & (~x - 1)) + 1) >> 1));
            }
            else
            {
                x = -1;
            }
        }

        return nums;
    }
};

int main()
{
    Solution solution;

    vector<int> v = {2, 3, 5, 7, 11, 13, 31};
    v = solution.minBitwiseArray(v);
    for (auto i : v)
    {
        cout << i << ' ';
    }
}
