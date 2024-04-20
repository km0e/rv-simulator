void main()
{
         int A[5];
         int i;
         for(i=0;i<5;i++)
             A[i]=i;
         for(i=1;i<5;i++)
             A[i]=A[i-1]+1000;
         if (i>3){
             A[50]=10;
         }
}