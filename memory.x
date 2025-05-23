MEMORY
{
  /* NOTE K = KiBi = 1024 bytes */
  FLASH : ORIGIN = 0x08000000, LENGTH = 512K
  RAM : ORIGIN = 0x20000000, LENGTH = 128K
}

/* Define the stack size */
_stack_size = 32K;

/* This is where the call stack will be allocated. */
/* The stack is of the full descending type. */
_stack_start = ORIGIN(RAM) + LENGTH(RAM);
_stack_end = _stack_start - _stack_size;

/* Define the minimum heap size */
_min_heap_size = 32K;

/* Define heap bounds */
_heap_start = ORIGIN(RAM);
_heap_end = _stack_end;
