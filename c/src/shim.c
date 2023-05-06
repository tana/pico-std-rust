#include <unistd.h>
#include <stdlib.h>
#include <errno.h>
#include "unwind.h"
#include "pico/time.h"
#include "hardware/timer.h"
#include "FreeRTOS.h"
#include "task.h"

int usleep(useconds_t us)
{
    absolute_time_t start_time = get_absolute_time();

    TickType_t ticks = us / (1000 * portTICK_PERIOD_MS);
    absolute_time_t end_time = delayed_by_us(start_time, us - 1000 * portTICK_PERIOD_MS * ticks);

    vTaskDelay(ticks);
    
    busy_wait_until(end_time);

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