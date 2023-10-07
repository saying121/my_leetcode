# 不同路径                                                           

* ID: 1000345 | Passing rate: 76.2% | PaidOnly: false  | Difficulty: Medium 
* Url: https://leetcode.cn/problems/2AoeFn/ 
* Topic: 数学, 动态规划, 组合数学 

## Test Case: 
```
3
7
3
2
7
3
3
3
```



---
一个机器人位于一个 `m x n`* *网格的左上角 （起始点在下图中标记为 “Start” ）。

机器人每次只能向下或者向右移动一步。机器人试图达到网格的右下角（在下图中标记为
“Finish” ）。

问总共有多少条不同的路径？


**示例 1：**


**输入：**m = 3, n = 7**输出：**28

**示例 2：**

**输入：**m = 3, n = 2**输出：**3**解释：**从左上角开始，总共有 3 条路径可以到达
右下角。1. 向右 -> 向下 -> 向下2. 向下 -> 向下 -> 向右3. 向下 -> 向右 -> 向下

**示例 3：**

**输入：**m = 7, n = 3**输出：**28

**示例 4：**

**输入：**m = 3, n = 3**输出：**6


**提示：**

* `1 <= m, n <= 100`
* 题目数据保证答案小于等于 `2 * 10⁹`


注意：本题与主站 62 题相同： [https://leetcode-cn.com/problems/unique-paths/][1]

[1]: \"https://leetcode-cn.com/problems/unique-paths/\"

---