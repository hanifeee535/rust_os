.syntax unified
.cpu cortex-m4
.thumb
.fpu fpv4-sp-d16

.global PendSV_Handler
.type PendSV_Handler, %function
PendSV_Handler:
    // Save current task context
    mrs     r0, psp
    stmdb   r0!, {r4-r11}     // Save R4–R11 to PSP
    bl      save_psp_value    // Store updated PSP in TCB

    // Select next task
    bl      update_to_next_task

    // Restore next task context
    bl      get_psp_value
    ldmia   r0!, {r4-r11}     // Restore R4–R11 from PSP
    msr     psp, r0           // Update PSP

    dsb                       // Ensure all memory ops complete
    isb                       // Flush pipeline, ensure PSP is visible
    bx      lr                // Exception return → restores R0–R3,R12,LR,PC,xPSR

//------------------------------------------------------

.global switch_sp_to_psp
.type switch_sp_to_psp, %function
switch_sp_to_psp:
    push    {lr}
    bl      get_psp_value
    msr     psp, r0
    pop     {lr}
    mov     r0, #0x02         // Use PSP in Thread mode, privileged
    msr     control, r0
    isb
    bx      lr

//------------------------------------------------------

.global init_scheduler_stack
.type init_scheduler_stack, %function
init_scheduler_stack:
    msr     msp, r0
    bx      lr
