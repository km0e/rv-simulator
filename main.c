void main()
{
         int A[100];
         int i;
         for(i=0;i<100;i++)
             A[i]=i;
         for(i=1;i<100;i++)
             A[i]=A[i-1]+1000;
         if (i>5){
             A[50]=10;
         }
}
IF/ID : addi x2 x2 -432
NPC             : 4        PC           : 0        INST         : E5010113
ID/EX : 
REG_WRITE       : 0        WB_SEL       : 1        MEM_WRITE    : 0        JAL_         : 0        BRANCH_EN    : 0       
PC_SEL          : 0        IMM_SEL      : 0        ALU_CTRL     : 0        BRANCH_TYPE  : 0        NPC          : 0       
PC              : 0        RS1_DATA     : 0        RS2_DATA     : 0        IMM          : 0        RS1          : 0       
RD              : 0        RS2          : 0        OPCODE       : 0        LOAD_SIGNAL  : 0
EX/MEM : 
REG_WRITE       : 0        WB_SEL       : 0        MEM_WRITE    : 0        NPC          : 0        ALU_RES      : 0       
RS2_DATA        : 0        RD           : 0
MEM/WB : 
REG_WRITE       : 0        WB_SEL       : 0        NPC          : 0        ALU_RES      : 0        MEM_DATA     : 0       
RD              : 0
Hazard
PC_EN           : 1       IF_IF_EN      : 1       ID_EX_CLR     : 0 