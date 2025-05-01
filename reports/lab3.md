# 实验报告

## lab3

1.将之前的代码迁移过来了
2.写了spawn函数
3.实现了stride调度，也就是在tcbInner加了优先级和stride，然后添加相应的更新函数，最主要是在manager里的fetch里面选择要调度的task并更新stride.

## homework3
1. 不是，p2调度之后溢出变成5,又成了最小，又选p2
2. 因为每次更新的步长为BigStride/2 而且每次都是更新最小stride,在最初所有stride=0的情况下，可以得出一个循环不变式，从而最大和最小之间的差距不超过这个。
3.
```rust
use core::cmp::Ordering;
struct Stride(u64);

impl PartialOrd for Stride {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.0.abs_diff(other.0) <= BigStride / 2 {
            return self.0.partial_cmp(other.0);
        } else {
            if self.0 > other.0 {
                return Some(Ordering::Less);
            } else {
                return Some(Ordering::Greater);
            }
        }
    }
}

impl PartialEq for Stride {
    fn eq(&self, other: &Self) -> bool {
        false
    }
}
``` 