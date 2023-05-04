#include <unistd.h>
#include <stdlib.h>
#include <errno.h>
#include "unwind.h"
#include "hardware/timer.h"

int usleep(useconds_t us)
{
    busy_wait_us(us);
    return 0;
}

char *realpath(const char *path, char *resolved_path)
{
    /* There is no filesystem */
    errno = ENOENT;
    return NULL;
}

/* Interface to a macro */
/* See https://stackoverflow.com/a/1952823 */
_Unwind_Word (_Unwind_GetIP)(struct _Unwind_Context *context)
{
    return _Unwind_GetIP(context);
}