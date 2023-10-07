# 最长公共前缀                                                         

* ID: 14      | Passing rate: 43.5% | PaidOnly: false  | Difficulty: Easy 
* Url: https://leetcode.cn/problems/longest-common-prefix/ 
* Topic: 字典树, 字符串 

## Test Case: 
```
["flower","flow","flight"]
["dog","racecar","car"]
```



---
编写一个函数来查找字符串数组中的最长公共前缀。

如果不存在公共前缀，返回空字符串 `\"\"`。


**示例 1：**

**输入：**strs = [\"flower\",\"flow\",\"flight\"]**输出：**\"fl\"

**示例 2：**

**输入：**strs = [\"dog\",\"racecar\",\"car\"]**输出：**\"\"**解释：**输入不存在
公共前缀。


**提示：**

* `1 <= strs.length <= 200`
* `0 <= strs[i].length <= 200`
* `strs[i]` 仅由小写英文字母组成

---