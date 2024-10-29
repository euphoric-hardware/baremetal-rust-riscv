/* Also see: https://github.com/riscv-software-src/riscv-tests/blob/51de00886cd28a3cf9b85ee306fb2b5ee5ab550e/benchmarks/common/test.ld */

OUTPUT_ARCH( "riscv" )
ENTRY(_start)

SECTIONS
{
  /* start */
  . = 0x80000000;
  /* end */
  _end = .;
}
