
MEMORY
{
    FLASH : ORIGIN = 0x8020000, LENGTH = 256K
    RAM : ORIGIN = 0x24000000, LENGTH = 512K
}

REGION_ALIAS("REGION_TEXT", FLASH);
REGION_ALIAS("REGION_RODATA", FLASH);
REGION_ALIAS("REGION_DATA", RAM);
REGION_ALIAS("REGION_BSS", RAM);
REGION_ALIAS("REGION_HEAP", RAM);
REGION_ALIAS("REGION_STACK", RAM);

INCLUDE link.x