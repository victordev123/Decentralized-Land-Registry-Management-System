; ModuleID = 'probe6.de0e773f43368b55-cgu.0'
source_filename = "probe6.de0e773f43368b55-cgu.0"
target datalayout = "e-m:e-p:32:32-p10:8:8-p20:8:8-i64:64-n32:64-S128-ni:1:10:20"
target triple = "wasm32-unknown-unknown"

; core::f64::<impl f64>::is_subnormal
; Function Attrs: inlinehint nounwind
define internal zeroext i1 @"_ZN4core3f6421_$LT$impl$u20$f64$GT$12is_subnormal17hc50d2f22713c5a62E"(double %self) unnamed_addr #0 {
start:
  %_2 = alloca [1 x i8], align 1
  %_4 = fcmp une double %self, %self
  br i1 %_4, label %bb1, label %bb2

bb2:                                              ; preds = %start
  %b = bitcast double %self to i64
  %_6 = and i64 %b, 4503599627370495
  %_7 = and i64 %b, 9218868437227405312
  %0 = icmp eq i64 %_6, 0
  br i1 %0, label %bb4, label %bb3

bb1:                                              ; preds = %start
  store i8 0, ptr %_2, align 1
  br label %bb9

bb4:                                              ; preds = %bb2
  switch i64 %_7, label %bb3 [
    i64 9218868437227405312, label %bb8
    i64 0, label %bb7
  ]

bb3:                                              ; preds = %bb4, %bb2
  %1 = icmp eq i64 %_7, 0
  br i1 %1, label %bb6, label %bb5

bb8:                                              ; preds = %bb4
  store i8 1, ptr %_2, align 1
  br label %bb9

bb7:                                              ; preds = %bb4
  store i8 2, ptr %_2, align 1
  br label %bb9

bb9:                                              ; preds = %bb1, %bb5, %bb6, %bb7, %bb8
  %2 = load i8, ptr %_2, align 1
  %_3 = zext i8 %2 to i32
  %_0 = icmp eq i32 %_3, 3
  ret i1 %_0

bb6:                                              ; preds = %bb3
  store i8 3, ptr %_2, align 1
  br label %bb9

bb5:                                              ; preds = %bb3
  store i8 4, ptr %_2, align 1
  br label %bb9
}

; probe6::probe
; Function Attrs: nounwind
define dso_local void @_ZN6probe65probe17h94f658ad63db18c6E() unnamed_addr #1 {
start:
; call core::f64::<impl f64>::is_subnormal
  %_1 = call zeroext i1 @"_ZN4core3f6421_$LT$impl$u20$f64$GT$12is_subnormal17hc50d2f22713c5a62E"(double 1.000000e+00) #2
  ret void
}

attributes #0 = { inlinehint nounwind "target-cpu"="generic" }
attributes #1 = { nounwind "target-cpu"="generic" }
attributes #2 = { nounwind }

!llvm.ident = !{!0}

!0 = !{!"rustc version 1.82.0 (f6e511eec 2024-10-15)"}
