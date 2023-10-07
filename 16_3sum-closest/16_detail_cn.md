# 最接近的三数之和                                                       

* ID: 16      | Passing rate: 45.0% | PaidOnly: false  | Difficulty: Medium 
* Url: https://leetcode.cn/problems/3sum-closest/ 
* Topic: 数组, 双指针, 排序 

## Test Case:

```
[-1,2,1,-4]
1
[0,0,0]
1
```

---

给你一个长度为 `n` 的整数数组 `nums`* *和 一个目标值 `target`。请你从 `nums`*
*中选出三个整数，使它们的和与 `target` 最接近。

返回这三个数的和。

假定每组输入只存在恰好一个解。


**示例 1：**

**输入：**nums = [-1,2,1,-4], target = 1
**输出：**2
**解释：**与 target 最接近的和是 2 (-1 + 2 + 1 = 2) 。

**示例 2：**

**输入：**nums = [0,0,0], target = 1
**输出：**0


**提示：**

* `3 <= nums.length <= 1000`
* `-1000 <= nums[i] <= 1000`
* `-10⁴ <= target <= 10⁴`

---