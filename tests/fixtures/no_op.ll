; ModuleID = 'no_op.c'
source_filename = "no_op.c"
target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"
target triple = "x86_64-pc-linux-gnu"

define i32 @main() {
entry:
  %a = alloca i32, align 4
  store i32 10, i32* %a, align 4
  %0 = load i32, i32* %a, align 4 ; 中間有 Load，上面的 Store 不能刪
  store i32 20, i32* %a, align 4
  %1 = load i32, i32* %a, align 4
  %add = add nsw i32 %0, %1
  ret i32 %add
}
