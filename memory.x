/* Important: don't over write bootloader from 0x0800_0000 to 0x0800_3fff */
MEMORY
{
  FLASH : ORIGIN = 0x08004000, LENGTH = 48K
  RAM : ORIGIN = 0x20000000, LENGTH = 16K
}
