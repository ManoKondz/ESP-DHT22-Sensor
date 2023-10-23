
    /* reserved at the start of DRAM for e.g. the BT stack */
    RESERVE_DRAM = 0x0;
    
    /* reserved at the start of the RTC memories for use by the ULP processor */
    RESERVE_RTC_FAST = 0;
    RESERVE_RTC_SLOW = 0;
    
    /* define stack size for both cores */
    STACK_SIZE = 8k;