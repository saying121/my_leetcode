# 第二高的薪水                                                        

* ID: 176     | Passing rate: 36.8% | PaidOnly: false  | Difficulty: Medium
* Url: https://leetcode.cn/problems/second-highest-salary/
* Topic: 数据库

## Test Case:

```
{"headers":{"Employee":["id","salary"]},"rows":{"Employee":[[1,100],[2,200],[3,300]]}}
{"headers":{"Employee":["id","salary"]},"rows":{"Employee":[[1,100]]}}
```

---

`Employee` 表：
+-------------+------+
| Column Name | Type |
+-------------+------+
| id          | int  |
| salary      | int  |
+-------------+------+
在 SQL 中，id 是这个表的主键。
表的每一行包含员工的工资信息。


查询并返回 `Employee` 表中第二高的薪水 。如果不存在第二高的薪水，查询应该返回
`null(Pandas 则返回 None)` 。

查询结果如下例所示。


**示例 1：**

**输入：**
Employee 表：
+----+--------+
| id | salary |
+----+--------+
| 1  | 100    |
| 2  | 200    |
| 3  | 300    |
+----+--------+
**输出：**
+---------------------+
| SecondHighestSalary |
+---------------------+
| 200                 |
+---------------------+

**示例 2：**

**输入：**
Employee 表：
+----+--------+
| id | salary |
+----+--------+
| 1  | 100    |
+----+--------+
**输出：**
+---------------------+
| SecondHighestSalary |
+---------------------+
| null                |
+---------------------+

---
```sql
Create table If Not Exists Employee (id int, salary int)
Truncate table Employee
insert into Employee (id, salary) values ('1', '100')
insert into Employee (id, salary) values ('2', '200')
insert into Employee (id, salary) values ('3', '300')
```
