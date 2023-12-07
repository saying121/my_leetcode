//start/
class Solution {
public:
    bool doesValidArrayExist(vector<int>& derived) {
        int n = 0;
        for (int i :derived) {
            n ^= i;
        }
        return n == 0;
    }
};
//end/
