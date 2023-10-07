# 单调递增的数字

* ID: 738     | Passing rate: 50.5% | PaidOnly: false  | Difficulty: Medium
* Url: https://leetcode.cn/problems/monotone-increasing-digits/
* Topic: 贪心, 数学

## Test Case:

```
10
1234
332
```

---

当且仅当每个相邻位数上的数字 `x` 和 `y` 满足 `x <= y`
时，我们称这个整数是**单调递增**的。

给定一个整数 `n` ，返回 *小于或等于 `n` 的最大数字，且数字呈 **单调递增*** 。


**示例 1:**

**输入:** n = `10`
**输出:** `9`

**示例 2:**

**输入:** n = `1234`
**输出:** `1234`

**示例 3:**

**输入:** n = `332`
**输出:** `299`


**提示:**

* `0 <= n <= 10⁹`

---
