#include <errno.h>
#include <sys/stat.h>
#include <sys/types.h>
#include <string.h>

extern char _heap_start, _heap_end;
static char *heap_ptr = &_heap_start;

void *_sbrk(int incr) {
    if (heap_ptr + incr > &_heap_end) {
        errno = ENOMEM;
        return (void*)-1;
    }
    void *ptr = heap_ptr;
    heap_ptr += incr;
    return ptr;
}

int _write(int fd, char *buf, int len) {
    // Implement UART output here if needed
    return len;
}

void *malloc(size_t size) { return _sbrk(size); }
void free(void *ptr) {}
void *calloc(size_t nmemb, size_t size) {
    void *ptr = _sbrk(nmemb * size);
    if (ptr) memset(ptr, 0, nmemb * size);
    return ptr;
}

// Additional syscall implementations
int _isatty(int fd) {
    if (fd <= 2) return 1;
    errno = EBADF;
    return 0;
}

int _close(int fd) {
    if (fd <= 2) return 0;
    errno = EBADF;
    return -1;
}

int _lseek(int fd, int ptr, int dir) {
    (void)fd;
    (void)ptr;
    (void)dir;
    errno = EBADF;
    return -1;
}

int _read(int fd, char *ptr, int len) {
    (void)fd;
    (void)ptr;
    (void)len;
    errno = EBADF;
    return -1;
}

int _fstat(int fd, struct stat *st) {
    if (fd <= 2) {
        st->st_mode = S_IFCHR;
        return 0;
    }
    errno = EBADF;
    return -1;
}

// Required by malloc when using -nostartfiles
void *_malloc_r(struct _reent *r, size_t size) { return malloc(size); }
void _free_r(struct _reent *r, void *ptr) { free(ptr); }
void *_calloc_r(struct _reent *r, size_t n, size_t size) { return calloc(n, size); }