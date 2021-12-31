/// These intrinsics aren't linked directly from LLVM and are mostly undocumented, however they are
/// simply lowered to the matching LLVM instructions by the compiler.  The associated instruction
/// is documented alongside each intrinsic.
extern "platform-intrinsic" {
    /// add/fadd
    pub fn simd_add<T>(x: T, y: T) -> T;

    /// sub/fsub
    pub fn simd_sub<T>(x: T, y: T) -> T;

    /// mul/fmul
    pub fn simd_mul<T>(x: T, y: T) -> T;

    /// udiv/sdiv/fdiv
    pub fn simd_div<T>(x: T, y: T) -> T;

    /// urem/srem/frem
    pub fn simd_rem<T>(x: T, y: T) -> T;

    /// shl
    pub fn simd_shl<T>(x: T, y: T) -> T;

    /// lshr/ashr
    pub fn simd_shr<T>(x: T, y: T) -> T;

    /// and
    pub fn simd_and<T>(x: T, y: T) -> T;

    /// or
    pub fn simd_or<T>(x: T, y: T) -> T;

    /// xor
    pub fn simd_xor<T>(x: T, y: T) -> T;

    /// fptoui/fptosi/uitofp/sitofp
    pub fn simd_cast<T, U>(x: T) -> U;

    /// neg/fneg
    pub fn simd_neg<T>(x: T) -> T;

    /// fabs
    pub fn simd_fabs<T>(x: T) -> T;

    pub fn simd_eq<T, U>(x: T, y: T) -> U;
    pub fn simd_ne<T, U>(x: T, y: T) -> U;
    pub fn simd_lt<T, U>(x: T, y: T) -> U;
    pub fn simd_le<T, U>(x: T, y: T) -> U;
    pub fn simd_gt<T, U>(x: T, y: T) -> U;
    pub fn simd_ge<T, U>(x: T, y: T) -> U;

    // shufflevector
    pub fn simd_shuffle<T, U, V>(x: T, y: T, idx: U) -> V;

    pub fn simd_gather<T, U, V>(val: T, ptr: U, mask: V) -> T;
    pub fn simd_scatter<T, U, V>(val: T, ptr: U, mask: V);

    // {s,u}add.sat
    pub fn simd_saturating_add<T>(x: T, y: T) -> T;

    // {s,u}sub.sat
    pub fn simd_saturating_sub<T>(x: T, y: T) -> T;

    // reductions
    pub fn simd_reduce_add_ordered<T, U>(x: T, y: U) -> U;
    pub fn simd_reduce_mul_ordered<T, U>(x: T, y: U) -> U;
    #[allow(unused)]
    pub fn simd_reduce_all<T>(x: T) -> bool;
    #[allow(unused)]
    pub fn simd_reduce_any<T>(x: T) -> bool;
    pub fn simd_reduce_max<T, U>(x: T) -> U;
    pub fn simd_reduce_min<T, U>(x: T) -> U;
    pub fn simd_reduce_and<T, U>(x: T) -> U;
    pub fn simd_reduce_or<T, U>(x: T) -> U;
    pub fn simd_reduce_xor<T, U>(x: T) -> U;

    // truncate integer vector to bitmask
    #[allow(unused)]
    pub fn simd_bitmask<T, U>(x: T) -> U;

    // select
    pub fn simd_select<M, T>(m: M, a: T, b: T) -> T;
    #[allow(unused)]
    pub fn simd_select_bitmask<M, T>(m: M, a: T, b: T) -> T;
}
