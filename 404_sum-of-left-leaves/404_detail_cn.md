# 左叶子之和                                                          

* ID: 404     | Passing rate: 62.5% | PaidOnly: false  | Difficulty: Easy 
* Url: https://leetcode.cn/problems/sum-of-left-leaves/ 
* Topic: 树, 深度优先搜索, 广度优先搜索, 二叉树 

## Test Case:

```
[3,9,20,null,null,15,7]
[1]
```

---

给定二叉树的根节点 `root` ，返回所有左叶子之和。


**示例 1：**


**输入:** root = [3,9,20,null,null,15,7] **输出:** 24 **解释:** 在这个二叉树中，
有两个左叶子，分别是 9 和 15，所以返回 24

**示例 2:**

**输入:** root = [1]**输出:** 0


**提示:**

* 节点数在 `[1, 1000]` 范围内
* `-1000 <= Node.val <= 1000`


---