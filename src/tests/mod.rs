pub fn do_rsgx_tests() -> usize {
    crate::dgemm_kernel::tests::do_rsgx_tests() + crate::sgemm_kernel::tests::do_rsgx_tests()
}
