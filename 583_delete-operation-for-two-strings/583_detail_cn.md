# 两个字符串的删除操作

* ID: 583     | Passing rate: 66.8% | PaidOnly: false  | Difficulty: Medium
* Url: https://leetcode.cn/problems/delete-operation-for-two-strings/
* Topic: 字符串, 动态规划

## Test Case:

```
"sea"
"eat"
"leetcode"
"etco"
```

---

给定两个单词 `word1` 和 `word2` ，返回使得 `word1` 和 `word2`
**相同** 所需的 **最小步数**。

**每步** 可以删除任意一个字符串中的一个字符。


**示例 1:**

**输入:** word1 = `"sea"`, word2 = `"eat"`
**输出:** `2`
**解释:** 第一步将 "sea" 变为 "ea" ，第二步将 "eat "变为 "ea"

**示例 2:**

输入：word1 = `"leetcode"`, word2 = `"etco"`
输出：`4`


**提示：**

* `1 <= word1.length, word2.length <= 500`
* `word1` 和 `word2` 只包含小写英文字母

---
