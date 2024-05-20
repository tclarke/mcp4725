/* Linker script for the STM32F103C8T6. Only used to run the included example, not needed by the library. */
MEMORY
{
  FLASH : ORIGIN = 0x08000000, LENGTH = 64K
  RAM : ORIGIN = 0x20000000, LENGTH = 20K
}
