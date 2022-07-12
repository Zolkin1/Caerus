;.section .text.context_switch
.global context_switch
.thumb_func
context_switch:
mrs r0, MSP
b HardFault