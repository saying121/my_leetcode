# 四数之和                                                           

* ID: 18      | Passing rate: 36.8% | PaidOnly: false  | Difficulty: Medium 
* Url: https://leetcode.cn/problems/4sum/ 
* Topic: 数组, 双指针, 排序 

## Test Case: 
```
[1,0,-1,0,-2,2]
0
[2,2,2,2,2]
8
```



---
给你一个由 `n` 个整数组成的数组 `nums` ，和一个目标值 `target`
。请你找出并返回满足下述全部条件且**不重复**的四元组 `[nums[a], nums[b],
nums[c], nums[d]]` （若两个四元组元素一一对应，则认为两个四元组重复）：

* `0 <= a, b, c, d < n`
* `a`、`b`、`c` 和 `d` **互不相同**
* `nums[a] + nums[b] + nums[c] + nums[d] == target`

你可以按 **任意顺序** 返回答案 。


**示例 1：**

**输入：**nums = [1,0,-1,0,-2,2], target = 0**输出：**[[-2,-1,1,2],[-2,0,0,2],[-
1,0,0,1]]

**示例 2：**

**输入：**nums = [2,2,2,2,2], target = 8**输出：**[[2,2,2,2]]


**提示：**

* `1 <= nums.length <= 200`
* `-10⁹ <= nums[i] <= 10⁹`
* `-10⁹ <= target <= 10⁹`

---