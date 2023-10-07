class Solution {
    public boolean isPalindrome(int x) {
        String string =String.valueOf(x);
        int len = string.length()-1;
        int i=0,j=len;
        while(i<j){
            if(string.charAt(i) != string.charAt(j)){
                return false;
            }
            i++;
            j--;
        }
        return true;
    }
}
