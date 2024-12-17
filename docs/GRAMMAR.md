# DSL 语法规则

## 1. 语法概述

DSL 语法基于 yaml 语法，分析文件时会首先通过反序列化函数进行反序列化，请首先确保文件符合 yaml 的写法与缩进。

并非所有字段都需要声明，在不需要和必须的字段详细说明中会有说明。

## 2. DSL 表层模块

DSL 第一层有四个模块，分别是 `bot_name`、`start_step`、`variables` 和 `steps`。

### 2.1 `bot_name` 模块（必须）

`bot_name` 模块用于指定机器人的名字，是一个字符串。

示例

```yaml
bot_name: "Sakiko"
```

### 2.2 `start_step` 模块（必须）

`start_step` 模块用于指定对话的起始步骤，是一个字符串，该字符串必须在 `steps` 模块中存在定义。

示例

```yaml
start_step: "step1"
```

### 2.3 `variables` 模块（可选）

`variables` 模块用于定义变量，是一个哈希表，键为变量名，值为变量的类型。

变量类型有 9 种，分别是 `Int`、`Float`、`Str`以及它们衍生的数组`IntVec`、`FloatVec`、`StrVec`和字典`IntDic`、`FloatDic`、`StrDic`。

不声明该模块时，将会只会有默认的变量，目前只有一个保留变量。

#### `Int` 类型

`Int` 类型表示整数，为一个 32 位有符号整数。

示例（消除了多余缩进，下同）

```yaml
int1: !Int 1
```

其中 `int1` 是变量名，`1` 是变量的值，`!Int` 表示该变量的类型为 `Int`。

#### `Float` 类型

`Float` 类型表示浮点数，为了一个 64 位浮点数。

示例

```yaml
float1: !Float 1.0
```

解释同上。

#### `Str` 类型

`Str` 类型表示字符串，为一个字符串。

示例

```yaml
str: !Str test
```

解释同上。

#### `IntVec` 类型

`IntVec` 类型表示整数数组，为一个 32 位有符号整数数组。

示例

```yaml
int_vec: !IntVec
- 1
- 2
- 3
```

其中 `int_vec` 是变量名，`1`、`2`、`3` 是数组中的值，`!IntVec` 表示该变量的类型为 `IntVec`。

#### `FloatVec` 类型

`FloatVec` 类型表示浮点数数组，为一个 64 位浮点数数组。

示例

```yaml
float_vec: !FloatVec
- 1.0
- 2.0
- 3.0
```

解释同上。

#### `StrVec` 类型

`StrVec` 类型表示字符串数组，为一个字符串数组。

示例

```yaml
str_vec: !StrVec
- test1
- test2
- test3
```

解释同上。

#### `IntDic` 类型

`IntDic` 类型表示整数字典，键为字符串，值为 32 位有符号整数的字典。

示例

```yaml
int_dic: !IntDic
  key1: 1
  key2: 2
  key3: 3
```

其中 `int_dic` 是变量名，`key1`、`key2`、`key3` 是字典的键，`1`、`2`、`3` 是字典的值，`!IntDic` 表示该变量的类型为 `IntDic`。

#### `FloatDic` 类型

`FloatDic` 类型表示浮点数字典，键为字符串，值为 64 位浮点数的字典。

示例

```yaml
float_dic: !FloatDic
  key1: 1.0
  key2: 2.0
  key3: 3.0
```

解释同上。

#### `StrDic` 类型

`StrDic` 类型表示字符串字典，键为字符串，值为字符串的字典。

示例

```yaml
str_dic: !StrDic
  key1: test1
  key2: test2
  key3: test3
```

解释同上。

#### ***注意*** 保留字段

虽然对变量名无限制，但是有一个保留字段 `input`，该字段用于存储用户本次输入，为字符串类型。

`checker` 程序会对变量名进行检查，如果变量名为 `input`，则会报错。

但在实际运行中，为了使程序尽量能够运行，会直接覆盖该变量。

### 2.4 `steps` 模块（必须）

`steps` 模块用于定义对话的步骤，是一个哈希表，键为步骤名，值为每个步骤的实现。

示例

```yaml
steps:
  step1:
    ...
  step2:
    ...
```

其中 `step1`、`step2` 是步骤名，`...` 是步骤具体的实现。

## 3. DSL 步骤模块

每个步骤包含两个个字段，分别是 `description`、`transitions`。

示例

```yaml
step1:
  description:
    ...
  transitions:
    ...
```

### 3.1 `description` 字段（可选）

`description` 字段用于定义步骤的描述，即机器人应该说的话，支持格式化，是一个字符串和一个字符串数组，为格式化字符串和要输出变量。

示例

无变量
```yaml
description:
  - Start of the conversation
  - []
```

有变量
```yaml
description:
  - End of the conversation {} {}
  - - int1
    - float1
```

其中 `Start of the conversation` 是机器人应该说的话，`[]` 是变量数组为空，`End of the conversation {} {}` 是机器人应该说的话，`int1`、`float1` 是变量名。

变量会按照顺序替换字符串中的 `{}`。

***注意*** 浮点数格式化时保留 3 位小数。

#### `description` 字段不声明时

如果不声明 `description` 字段，客服机器人将不会输出任何内容，同时该步骤也不会要求用户输入，仅会通过条件判断进行跳转和操作变量。

***注意*** 此时该步骤类似自动机中的空转移，可能会导致无限循环，对此 `checker` 程序不会进行检查。

### 3.2 `transitions` 字段（必须）

`transitions` 字段用于定义步骤的转移，即根据用户输入的内容或变量比较条件，机器人应该如何进行下一步的操作。

`transitions` 字段是一个类的数组。

示例

```yaml
transitions:
  - ...
  - ...
```

## 4. DSL 转移模块

每个转移包含多个字段，分别是 `pattern`、`compares`、`step` 和 `operation`。

示例

```yaml
- pattern: ...
  compares:
    ...
  step: ...
  operation:
    ...
```

### 4.1 `pattern` 字段（可选）

`pattern` 字段用于定义用户输入的模式，即用户输入的内容应该符合的正则表达式或直接匹配的字符串。

示例

直接匹配的字符串
```yaml
pattern: "test"
```

正则表达式
```yaml
pattern: "^(\d+)$"
```

***注意*** 匹配正则表达式时，默认是部分匹配，即只要用户输入的内容中包含该正则表达式即可，若需要完全匹配，需要在正则表达式前后加上 `^` 和 `$`。

#### `pattern` 字段不声明时

如果不声明 `pattern` 字段，该转移将会不需要用户输入，直接进行条件判断和操作变量。

***注意*** 一个转移模块中可以同时存在未声明和声明的 `pattern` 字段，但是未声明的 `pattern` 字段必须在声明的 `pattern` 字段之前，客服机器人会按顺序处理所有的转移模块，遇到第一个符合条件的转移模块后会直接跳转，遇到第一个需要用户输入的转移模块后会忽略其后的所有不需要用户输入的转移模块。

***注意*** 此时该步骤类似自动机中的空转移，可能会导致无限循环，对此 `checker` 程序不会进行检查。

### 4.2 `compares` 字段（可选）

`compares` 字段用于定义用户输入的内容与变量的比较条件，即用户输入的内容应该与变量的值符合的比较条件。

`compares` 字段是一个数组，数组中每个元素是一个比较模块。

示例

```yaml
compares:
  - ...
  - ...
```

#### `compares` 不声明时

默认为真

### 4.3 `step` 字段（必须）

`step` 字段用于定义转移的目标步骤，即用户输入的内容符合条件后应该跳转到的步骤。

示例

```yaml
step: "step2"
```

该字段是一个字符串，字符串的值必须在 `steps` 模块中存在定义或为 `end`。

#### ***注意*** 保留字段

`end` 为保留字段，表示结束对话。

`checker` 程序会对步骤名进行检查，如果步骤名为 `end`，则会报错。

但在实际运行中，为了使程序尽量能够运行，会忽略掉 `end` 字段中的所有内容。

### 4.4 `operation` 字段（可选）

`operation` 字段用于定义转移的操作，即用户输入的内容符合条件后应该进行的操作。

`operation` 字段是一个数组，数组中每个元素是一个操作模块。

示例

```yaml
operation:
  - ...
  - ...
```

## 5. DSL 比较模块

每个比较模块包含两个字段，分别是 `compare` 和 `compare_type`。

示例

```yaml
- compare: ...
  compare_type: ...
    ...
```

### 5.1 `compare` 字段（必须）

`compare` 字段用于定义该次比较的结果是与还是或，与需要所有同类模块为真，或需要所有模块至少一个为真。

示例

```yaml
compare: And
```

该字段是一个字符串，字符串的值为 `And` 或 `Or`。

***注意*** 当只有一个比较模块时，该字段是多余的，但是为了保持统一性，还是需要声明。

### 5.2 `compare_type` 字段（必须）

`compare_type` 字段用于定义该次比较的类型，即用户输入的内容与变量的比较条件。

有 6 种比较类型，分别是 `Eq`、`Ne`、`Gt`、`Ge`、`Lt` 和 `Le`。

#### `Eq` 类型

判断两个变量是否相等。

示例

```yaml
compare_type: !Eq
  - int1
  - int2
```

其中 `int1`、`int2` 是变量名，`!Eq` 表示该比较的类型为 `Eq`。

***注意*** 所有变量的类型必须相同且为 `Int`、`Float` 或 `Str`，变量必须在 `variables` 模块中声明过。

#### `Ne` 类型

判断两个变量是否不相等。

示例

```yaml
compare_type: !Ne
  - int1
  - int2
```

解释同上。

#### `Gt` 类型

判断第一个变量是否大于第二个变量。

示例

```yaml
compare_type: !Gt
  - int1
  - int2
```

解释同上。

#### `Ge` 类型

判断第一个变量是否大于等于第二个变量。

示例

```yaml
compare_type: !Ge
  - int1
  - int2
```

解释同上。

#### `Lt` 类型

判断第一个变量是否小于第二个变量。

示例

```yaml
compare_type: !Lt
  - int1
  - int2
```

解释同上。

#### `Le` 类型

判断第一个变量是否小于等于第二个变量。

示例

```yaml
compare_type: !Le
  - int1
  - int2
```

解释同上。

## 6. DSL 操作模块

每个操作模块包含一个字段，包含操作名和操作参数。

示例

```yaml
- !Add
  - ans
  - a
  - b
```

有 13 种操作，分别是 `Add`、`Sub`、`Mul`、`Div`、`Get`、`Set`、`Let`、`Cpy`、`Rnd`、`Shu`、`Qry`、`Ins` 和 `Inp`。

### 6.1 `Add` 操作

`Add` 操作用于将两个变量相加，结果存入第一个变量。

示例

```yaml
- !Add
  - ans
  - a
  - b
```

为 `ans = a + b`。

变量类型必须相同且为 `Int`或`Float` 。

### 6.2 `Sub` 操作

`Sub` 操作用于将第三个变量从第二个变量中减去，结果存入第一个变量。

示例

```yaml
- !Sub
  - ans
  - a
  - b
```

为 `ans = a - b`。

变量类型必须相同且为 `Int`或`Float` 。

### 6.3 `Mul` 操作

`Mul` 操作用于将两个变量相乘，结果存入第一个变量。

示例

```yaml
- !Mul
  - ans
  - a
  - b
```

为 `ans = a * b`。

变量类型必须相同且为 `Int`或`Float` 。

### 6.4 `Div` 操作

`Div` 操作用于将第二个变量除以第三个变量，结果存入第一个变量。

示例

```yaml
- !Div
  - ans
  - a
  - b
```

为 `ans = a / b`。

变量类型必须相同且为 `Int`或`Float` 。

### 6.5 `Get` 操作

`Get` 操作用于获取数组中的元素，结果存入第一个变量。

示例

```yaml
- !Get
  - ans
  - int_vec
  - iter
```

为 `ans = int_vec[iter]`。

`iter` 为迭代变量，必须为 `Int` 类型。

数组储存的变量类型与结果必须相同。

***注意*** 数组越界操作会导致运行时错误，`checker` 程序不会进行检查。

### 6.6 `Set` 操作

`Set` 操作用于设置数组中的元素。

示例

```yaml
- !Set
  - a
  - int_vec
  - iter
```

为 `int_vec[iter] = a`。

`iter` 为迭代变量，必须为 `Int` 类型。

数组储存的变量类型与结果必须相同。

***注意*** 数组越界操作会导致运行时错误，`checker` 程序不会进行检查。

### 6.7 `Let` 操作

`Let` 操作用于设置变量的值。

示例

```yaml
- !Let
  - a
  - !Int 1
```

为 `a = 1`。

变量类型必须与设定新值的类型相同。

### 6.8 `Cpy` 操作

`Cpy` 操作用于将第二个变量的值赋给第一个变量。

示例

```yaml
- !Cpy
  - a
  - b
```

为 `a = b`。

变量类型必须相同。

### 6.9 `Rnd` 操作

`Rnd` 操作用于生成随机数。

示例

```yaml
- !Rnd
  - a
  - l
  - r
```

为 `a = random(l, r)`。

`l` 为左边界，`r` 为右边界。

变量类型必须为 `Int` 或 `Float`，且必须相同。

### 6.10 `Shu` 操作

`Shu` 操作用于打乱数组。

示例

```yaml
- !Shu
  - int_vec
```

为 `shuffle(int_vec)`。

必须为数组类型。

### 6.11 `Qry` 操作

`Qry` 操作用于查询字典中的值，结果存入第一个变量。

示例

```yaml
- !Qry
  - ans
  - int_dic
  - key
```

为 `ans = int_dic[key]`。

`key` 为键，必须为 `Str` 类型。

字典储存的变量类型与结果必须相同。

### 6.12 `Ins` 操作

`Ins` 操作用于插入元素到字典中。

示例

```yaml
- !Ins
  - a
  - int_dic
  - key
```

为 `int_dic[key] = a`。

`key` 为键，必须为 `Str` 类型。

字典储存的变量类型与结果必须相同。

### 6.13 `Inp` 操作

`Inp` 操作用于获取用户输入。

示例

```yaml
- !Inp
  - a
```

为 `a = input`。

变量类型必须为 `Str`、`Int` 或 `Float`。

***注意*** 对输入的内容不进行检查，可能会导致运行时错误。

### 7 `checker` 程序的使用

`checker` 程序用于检查 DSL 文件的正确性，包括语法错误、变量未声明、步骤未定义等。

`checker` 程序的使用方法为

使用 `cargo` 运行
```shell
cargo run --release --bin checker -- <DSL 文件路径>
```

直接运行
```shell
./checker <DSL 文件路径>
```

`checker` 程序会输出检查结果。

***注意*** `checker` 程序只会检查语法错误，不会检查逻辑错误，如无限循环等。

***注意*** `checker` 相比实际运行在有些地方会更加严格，如变量名检查等，一个步骤的语法出现问题如果在实际运行中没有到达该步骤则不会报错，但是 `checker` 程序会报错。