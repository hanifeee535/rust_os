.syntax unified
.cpu cortex-m4
.thumb
.fpu fpv4-sp-d16

.global PendSV_Handler
.type PendSV_Handler, %function
PendSV_Handler:

    // Save current task context
    //1. get current running task's psp value
    mrs     r0, psp  

    //2. Using that psp value, store SF2 (R4 to R11)         
    stmdb   r0!, {r4-r11}     // Save R4–R11 to PSP
    push    {lr}

    //3. Save the current value of PSP
    bl      save_psp_value   

    // Retrieve the context of next task 
	//1. Decide next task to run
    bl      update_to_next_task

    //2. get its past psp value
    bl      get_psp_value

    //3. Using that PSP value retrieve SF2 (R4 to R11)
    ldmia   r0!, {r4-r11}   

    //3. Update PSP and exit  
    msr     psp, r0           // Update PSP
    pop     {lr}
    bx      lr                // Exception return → restores R0–R3,R12,LR,PC,xPSR

//------------------------------------------------------

.global switch_sp_to_psp
.type switch_sp_to_psp, %function
switch_sp_to_psp:
    //initialize the psp with task1 stack start 
	//because we are first going to lunch task 1

    push    {lr}               // to get back to the previous function from where it called, we need to preserve LR. because in the next line it will call another function named get_psp_value. After this, we will pop it back
    bl      get_psp_value      //get the value of psp of current task
    msr     psp, r0            //initialize psp
    pop     {lr}               //pop back LR

    //Change stack pointer to psp using control register
    mov     r0, #0x02         // Use PSP in Thread mode, privileged
    msr     control, r0
    bx      lr

//------------------------------------------------------
// Set MSP (Main Stack Pointer) for the scheduler
.global init_scheduler_stack
.type init_scheduler_stack, %function
init_scheduler_stack:
    msr     msp, r0         // Load R0 value (top_of_stack variable) into MSP
    bx      lr              // Return from function

