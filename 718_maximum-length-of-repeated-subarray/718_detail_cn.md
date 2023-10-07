# 最长重复子数组

* ID: 718     | Passing rate: 56.8% | PaidOnly: false  | Difficulty: Medium
* Url: https://leetcode.cn/problems/maximum-length-of-repeated-subarray/
* Topic: 数组, 二分查找, 动态规划, 滑动窗口, 哈希函数, 滚动哈希

## Test Case:

```
[1,2,3,2,1]
[3,2,1,4,7]
[0,0,0,0,0]
[0,0,0,0,0]
```

---

给两个整数数组 `nums1` 和 `nums2` ，返回 *两个数组中 **公共的** 、长度最长的子数组的长度 *。


**示例 1：**

**输入：**nums1 = `[1,2,3,2,1]`, nums2 = `[3,2,1,4,7]`
**输出：**`3`
**解释：**长度最长的公共子数组是 `[3,2,1]` 。

**示例 2：**

**输入：**nums1 = `[0,0,0,0,0]`, nums2 = `[0,0,0,0,0]`
**输出：**`5`


**提示：**

* `1 <= nums1.length, nums2.length <= 1000`
* `0 <= nums1[i], nums2[i] <= 100`

---
