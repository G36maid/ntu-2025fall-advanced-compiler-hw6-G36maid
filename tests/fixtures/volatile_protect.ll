; Test case: Volatile store protection
; Purpose: Verify that volatile stores are NEVER eliminated, even if they appear dead
; Expected: The volatile store should be preserved after DSE optimization

define void @test_volatile() {
entry:
  %a = alloca i32, align 4
  
  ; This volatile store MUST NOT be removed, even though it's overwritten
  store volatile i32 10, ptr %a, align 4
  
  ; This regular store overwrites the previous value
  ; The volatile store should still be preserved
  store i32 20, ptr %a, align 4
  
  ret void
}

; Additional test: Multiple volatile stores
define void @test_multiple_volatile() {
entry:
  %b = alloca i32, align 4
  
  ; First volatile store - must be preserved
  store volatile i32 1, ptr %b, align 4
  
  ; Second volatile store - must also be preserved
  store volatile i32 2, ptr %b, align 4
  
  ; Third volatile store - must also be preserved
  store volatile i32 3, ptr %b, align 4
  
  ret void
}

; Test: Mixed volatile and non-volatile stores
define void @test_mixed_stores() {
entry:
  %c = alloca i32, align 4
  
  ; Dead non-volatile store (can be removed)
  store i32 100, ptr %c, align 4
  
  ; Volatile store (must be preserved)
  store volatile i32 200, ptr %c, align 4
  
  ; Another non-volatile store (should be preserved as it's the last store)
  store i32 300, ptr %c, align 4
  
  ret void
}
