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
