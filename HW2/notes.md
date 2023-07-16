# 所有权、不可变引用、可变引用
## Ownership, Reference, Borrowing

Rust 的所有权 (ownership) 机制规定：Rust 中的每个值都有一个被称为其所有者（owner）的变量，并且有且只能有唯一的所有者。

Rust 中的引用 (references) 允许使用值但不获取其所有权，这种操作也被称为所有权借用（borrowing）。通过＆T 和＆mut T 将引用分为两种：

- 不可变引用（**&T**），也被称为共享引用，所有者可以读取引用指向的数据，*但不能修改数据*。

- 可变引用（**&mut T**）也被称为独占引用，不能有别名，在同一时刻，*同一个值不可能存在别的引用*。

```
fn main() {

    let mut s = String::from("hello");
    let r1 = &mut s;
    let r2 = &mut s; // ERROR: 可变引用，不能有别名
    println!("{}, {}", r1, r2);
    
    let x = 1;
    let y = &mut x; // ERROR: 当有一个不可变值时，不能可变的借用它
}
```

Rust内存安全性基于以下规则：给定对象T，则只能符合以下一个条件：
- 对象有几个不可变的引用（&T），也称为别名（aliasing）。
- 对象有一个可变引用（&mut T），也称为可变性（mutability）。

**所以一个变量不能同时拥有可变引用和不可变引用**


a pretty tricky example:
```
struct Foo(usize);
fn main() {

    let mut a = &Foo(2);
    a = &Foo(5);  // ok
    a.0 = 7;      // [E0594]: cannot assign to `a.0` which is behind a `&` reference
    
    let b = &mut Foo(42);
    b = &Foo(36); // [E0384]: cannot assign twice to immutable variable `b`
    b.0 = 100;    // ok
    
}
```

----
更有意思的部分 - move和copy的真正区别

- Move: 如果一个类型没有实现Copy这个trait，那么把这个类型的变量赋给另一个变量或传递给一个函数时就会发生move，细节上来说，就是把右值的value赋给左值，然后invaildate右值的地址，使之不可访问；不过在TRPL中是用ownership解释的，本质是一样的。

- Copy: 


那么可变引用和不可变引用的理解就变的简单不少：
- 可变引用：可变引用不是Copy的
- 不可变应用：不可变引用实现了Copy，所以不会因为赋值而发生move

### 隐式重借用(implicitly reborrowed)


## Advanced - Cell和RefCell

#### Reference:
https://rustcc.cn/article?id=37d1cb4f-5cc9-4adc-b41a-dbe4914bf4b5
https://www.jianshu.com/p/707385771ad1
