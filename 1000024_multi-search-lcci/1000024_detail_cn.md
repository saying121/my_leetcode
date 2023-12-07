# 多次搜索                                                          

* ID: 1000024 | Passing rate: 44.6% | PaidOnly: false  | Difficulty: Medium
* Url: https://leetcode.cn/problems/multi-search-lcci/
* Topic: 字典树, 数组, 哈希表, 字符串, 字符串匹配, 滑动窗口

## Test Case:

```
""
["a","b","c"]
```

---

给定一个较长字符串`big`和一个包含较短字符串的数组`smalls`，设计一个方法，根据`sm
alls`中的每一个较短字符串，对`big`进行搜索。输出`smalls`中的字符串在`big`里出现
的所有位置`positions`，其中`positions[i]`为`smalls[i]`出现的所有位置。

**示例：**

**输入：**
big = "mississippi"
smalls = ["is","ppi","hi","sis","i","ssippi"]
**输出：** [[1,4],[8],[],[3],[1,4,7,10],[5]]

**提示：**

* `0 <= len(big) <= 1000`
* `0 <= len(smalls[i]) <= 1000`
* `smalls`的总字符数不会超过 100000。
* 你可以认为`smalls`中没有重复字符串。
* 所有出现的字符均为英文小写字母。

---