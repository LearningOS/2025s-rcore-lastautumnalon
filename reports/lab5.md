# lab5

## lab
写了死锁检测，数据主要维护在pcb里。
然后锁的available可以直接在这些锁的实现里面维护

## homework
1.回收子线程TaskUserRes中的资源，回收进程孩子控制块，回收memset

2.第一种实现是一直会请求锁，而unlock在添加等待task之后没有重新把mutex锁上
第二种实现只请求一次锁，可能会在调度回来之后无法请求到锁了。
而unlock应该没有问题
