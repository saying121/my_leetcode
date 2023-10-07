# 二叉树的最小深度                                                       

* ID: 111     | Passing rate: 52.7% | PaidOnly: false  | Difficulty: Easy 
* Url: https://leetcode.cn/problems/minimum-depth-of-binary-tree/ 
* Topic: 树, 深度优先搜索, 广度优先搜索, 二叉树 

## Test Case:

```
[3,9,20,null,null,15,7]
[2,null,3,null,4,null,5,null,6]
```

---

给定一个二叉树，找出其最小深度。

最小深度是从根节点到最近叶子节点的最短路径上的节点数量。

**说明：**叶子节点是指没有子节点的节点。


**示例 1：**

[\"\"]
**输入：**root = [3,9,20,null,null,15,7]
**输出：**2

**示例 2：**

**输入：**root = [2,null,3,null,4,null,5,null,6]
**输出：**5


**提示：**

* 树中节点数的范围在 `[0, 10⁵]` 内
* `-1000 <= Node.val <= 1000`

---