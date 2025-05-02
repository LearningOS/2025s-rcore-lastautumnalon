# Lab 1 report

## Coding assignments

功能：
1.读取id处的无符号整数值u8
2.把data as u8 write into id address.
3.在task里面添加了一个SyscallCounter，对当前任务的每个syscall进行调用次数记录，并实现更新方法和查询方法。

## homework

1.
app_0 运行时，kernel提示PageFault,然后kill了
app_1,app_2 都是IllegalInstruction,然后也被kernel kill了

因为app0直接写0x0的内存
app1,2用了S态的指令，不符合特权级

2.

2.1 sp代表内核栈指针，__restore可用于初始化app和trap执行后的用户上下文恢复
2.2 用户t0 t1 t2 其中 t2 为用户态sp，t0为sstatus，t1为sepc。sstatus存储特权级，在sret后CPU会自动设置; sepc在sret后，CPU按照sepc指向的指令继续执行, sp为用户态的栈指针。
2.3 因为x2代表着现在的sp,如果直接ld，那么后面的指令无法正确执行，因为栈里面2*8(sp)是用户态的sp;x4一般不会用到，在保存的时候这两个都没保存 
2.4 sp跟sccratch交换，其实是sp成为了用户态栈指针，sccratch成为内核态栈指针
2.5 sret 在risc-v中唯一一种使特权级下降的方法就是trap返回系列指令 而且trap处理完成之后确实应该回到user正常运行了，所以要切换到U-Mode
2.6 sp为内核栈指针，sccratch为用户栈指针
2.7 trap触发的一瞬间就进入了。。 那应该就是csrrw sp, sccratch, sp ?
[1]


    在完成本次实验的过程（含此前学习的过程）中，我曾分别与 以下各位 就（与本次实验相关的）以下方面做过交流，还在代码中对应的位置以注释形式记录了具体的交流对象及内容：

        《ChatGpt,Google》

    此外，我也参考了 以下资料 ，还在代码中对应的位置以注释形式记录了具体的参考来源及内容：

        [1]Tutorial Guide ch2 https://learningos.cn/rCore-Tutorial-Guide-2025S/chapter2/4trap-handling.html

3. 我独立完成了本次实验除以上方面之外的所有工作，包括代码与文档。 我清楚地知道，从以上方面获得的信息在一定程度上降低了实验难度，可能会影响起评分。

4. 我从未使用过他人的代码，不管是原封不动地复制，还是经过了某些等价转换。 我未曾也不会向他人（含此后各届同学）复制或公开我的实验代码，我有义务妥善保管好它们。 我提交至本实验的评测系统的代码，均无意于破坏或妨碍任何计算机系统的正常运转。 我清楚地知道，以上情况均为本课程纪律所禁止，若违反，对应的实验成绩将按“-100”分计。
