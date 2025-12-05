#include <windows.h>
#include <mmsystem.h>
#include <stdio.h>

int main() {
    printf("Calling timeGetTime: %u\n", timeGetTime());
    timeBeginPeriod(1);
    timeEndPeriod(1);
    return 0;
}
