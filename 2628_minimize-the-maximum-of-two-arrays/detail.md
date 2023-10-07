# 最小化两个数组中的最大值                                                   

* ID: 2628    | Passing rate: 34.8% | PaidOnly: false  | Difficulty: Medium 
* Url: https://leetcode.cn/problems/minimize-the-maximum-of-two-arrays/ 
* Topic: 数学, 二分查找, 数论 

## Test Case: 
```
2
7
1
3
3
5
2
1
2
4
8
2
```



---
给你两个数组 `arr1` 和 `arr2`
，它们一开始都是空的。你需要往它们中添加正整数，使它们满足以下条件：

* `arr1` 包含 `uniqueCnt1` 个** 互不相同** 的正整数，每个整数都 **不能 **被
  `divisor1` **整除** 。
* `arr2` 包含 `uniqueCnt2` 个** 互不相同** 的正整数，每个整数都 **不能** 被
  `divisor2` **整除** 。
* `arr1` 和 `arr2` 中的元素 **互不相同** 。

给你 `divisor1` ，`divisor2` ，`uniqueCnt1` 和 `uniqueCnt2` ，请你返回两个数组中
**最大元素** 的 **最小值** 。


**示例 1：**

输入：divisor1 = 2, divisor2 = 7, uniqueCnt1 = 1, uniqueCnt2 = 3输出：4解释：我
们可以把前 4 个自然数划分到 arr1 和 arr2 中。arr1 = [1] 和 arr2 = [2,3,4] 。可以
看出两个数组都满足条件。最大值是 4 ，所以返回 4 。

**示例 2：**

输入：divisor1 = 3, divisor2 = 5, uniqueCnt1 = 2, uniqueCnt2 = 1输出：3解释：arr
1 = [1,2] 和 arr2 = [3] 满足所有条件。最大值是 3 ，所以返回 3 。

**示例 3：**

输入：divisor1 = 2, divisor2 = 4, uniqueCnt1 = 8, uniqueCnt2 = 2输出：15解释：最
终数组为 arr1 = [1,3,5,7,9,11,13,15] 和 arr2 = [2,6] 。上述方案是满足所有条件的
最优解。


**提示：**

* `2 <= divisor1, divisor2 <= 10⁵`
* `1 <= uniqueCnt1, uniqueCnt2 < 10⁹`
* `2 <= uniqueCnt1 + uniqueCnt2 <= 10⁹`

---