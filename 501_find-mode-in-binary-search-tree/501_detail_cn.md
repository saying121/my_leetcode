# 二叉搜索树中的众数                                                      

* ID: 501     | Passing rate: 54.8% | PaidOnly: false  | Difficulty: Easy 
* Url: https://leetcode.cn/problems/find-mode-in-binary-search-tree/ 
* Topic: 树, 深度优先搜索, 二叉搜索树, 二叉树 

## Test Case:

```
[1,null,2,2]
[0]
```

---

给你一个含重复值的二叉搜索树（BST）的根节点 `root` ，找出并返回 BST 中的所有
[众数][1]（即，出现频率最高的元素）。

如果树中有不止一个众数，可以按 **任意顺序** 返回。

假定 BST 满足如下定义：

* 结点左子树中所含节点的值 **小于等于** 当前节点的值
* 结点右子树中所含节点的值 **大于等于** 当前节点的值
* 左子树和右子树都是二叉搜索树


**示例 1：**

[\"\"]
**输入：**root = [1,null,2,2]
**输出：**[2]

**示例 2：**

**输入：**root = [0]
**输出：**[0]


**提示：**

* 树中节点的数目在范围 `[1, 10⁴]` 内
* `-10⁵ <= Node.val <= 10⁵`


**进阶：**你可以不使用额外的空间吗？（假设由递归产生的隐式调用栈的开销不被计算在
内）

[1]: \"https://baike.baidu.com/item/%E4%BC%97%E6%95%B0/44796\"

---