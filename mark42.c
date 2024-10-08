/*mark 42*/
#include<stdio.h>
#include<string.h>
#include<conio.h>
void funall ();
void fun5 ();
void fun7 ();
void fun8 ();
void fun10 ();
void fun11 ();
void fun13 ();
void fun17 ();
void fun18 ();
void fun19 ();
void fun20 ();
void fun21 ();
void fun22 ();
void fun23 ();
void fun24 ();
void fun25 ();
void fun34 ();
void fun35 ();
void fun48 ();
void fun58 ();
void fun59 ();
void fun60 ();
void fun71 ();
void fun72 ();
void fun80 ();
void fun81 ();
main()
{
	long int l,m,i,n,k;
   char a[242],b[24],c[24];
	clrscr ();
	printf("if you want to see all programmes'show all'\n");
	gets (b);
	n=strcmp (b,"show all");
	if (n==0)
	 funall ();
	printf("\n\nenter program\n");
	gets (a);
	l=strlen (a);
	if (l==17)
	{
	  m=strcmp (a,"decimal to binary");
	  if (m==0)
	  l=17;  /* decimal to binary  */
	  if (m==9)
	  l=34; /* delecting a string  */
	  if (m==(-2))
	  l=35; /* binary to decimal*/
	}
	if (l==24)
	{
		m=strcmp (a,"sum of individual digits");
		if (m==0)
		 l=24;
		if (m==(-6))
		 l=48;
	}
	if (l==13)
  {
 m=strcmp (a,"linear search");
 if (m==0)
  l=13;
 if (m==(-59))
  l=59;
 if (m==(-58))
  l=58;
 if (m==(-10))
  l=10;
 if (m==4)
  l=60;
 }
 if (l==23)
{
 m=strcmp (a,"upper triangular matrix");
 if (m==0)
  l=23;
 if (m==(-9))
  l=22;
 if (m==(-8))
  l=21;
 if (m==(-18))
  l=20;
}
if (l==18)
 {
   m=strcmp (a,"fibonacci sequence");
   if (m==0)
    l=18;
   if (m==13)
    l=81;
   if (m==12)
    l=80;
 }
 if (l==19)
  {
  	m=strcmp (a,"compare two strings");
  	 if (m==0)
  	   l=19;
  	  if (m==17)
  	  l=71;
  }
	switch (l)
	  {
	    case 17:fun17 ();
		    break;
	    case 34:fun34 ();
		    break;
	    case 35:fun35 ();
		    break;
		case 25:fun25 ();
		   break;
		 case 8:fun8 ();
		   break;
		 case 24:fun24 ();
		   break;
  	  case 7:fun7 ();
		   break;
		 case 20:fun20 ();
		  break;
		 case 5:fun5 ();
		  break;
		 case 11:fun11 ();
		  break;
	    case 48:fun48 ();
		  break;
		 case 10:fun10 ();
		  break;
		  case 58:fun58 ();
		   break;
		   case 59:fun59 ();
		   break;
		   case 13:fun13 ();
		   break;
		   case 60:fun60 ();
		    break;
		    case 23:fun23 ();
             break;
          case 22:fun22 ();
             break;
           case 19:fun19 ();
           break;
           case 71:fun71 ();
           break;
        case 21:fun21 ();
            break;
        case 72:fun72 ();
             break;
          case 18:fun18 ();
            break;
          case 81:fun81 ();
            break;
          case 80:fun80 ();
            break;
	    default: printf("shit");
	   }
	    getch ();
	  }
	  void funall ()/*keys to all programmes*/
{
	printf("\n::::::::kspk's programmes::::::::\n");
	printf("\ndecimal to binary\n"
	            "binary to decimal\n"
    	        "pallindrome\n" 
    	        "deleting a string\n"
    	        "linear search\n"
    	        "binary search\n"
    	        "prime numbers\n"
    	        "largest of three integers\n"
    	          "swapping\n"
    	          "1s compliment\n"
    	          "2s compliment\n"
    	          "sorting\n"
    	          "lower triangular matrix\n"
    	          "upper triangular matrix\n"
    	          "max and min of an array\n"
    	          "concatinate two strings\n"
    	          "multiplication table\n"
    	          "reversing a string\n"
    	          "fibonacci sequence\n"
    	          "symmetri of matrix\n"
    	          "compare two strings\n"
    	          "transpose of matrix\n"
    	          "trace\n"
    	          "sum of individual digits\n"
    	          "mul of individual digits\n");
}
	  void fun17 ()          /*decimal to binary  */
	  {
	  long int a[24],n,j,i=0;
	  printf("\nenter a decimal number \n");
	  scanf("%ld",&n);
	  while (n>0)
	  {
	  a[i]=n%2;
	  i++;
	  n=n/2;
	  }
	  printf("\nbinary form \n");
	  for(j=i-1;j>=0;j--)
	  {
	  printf("%ld",a[j]);
	  }
	  }
	  void fun34 () /*deleting a string   */
	  {
	  long int i,p,n,l1;
	  char a[24];
	  printf("\nenter string\n");  /*entering string */
	  gets (a);
	  l1=strlen (a); /*finding length of string  */
	  printf("\nenter position\n");
	  scanf("%ld",&p);/* entering position */
	  printf("\nenter number of characters had to be delected \n");
	  scanf("%ld",&n);
	  /*logic  */
	  for (i=p-1;i<l1;i++)
	  {
	    a[i]=a[i+n];
	  }
	  /*printing the desired string  */
	  printf("\nyour desired string\n");
	  puts (a);
	  }
	  void fun35 ()/*binary to decimal*/
	  {
	  long int bn,d=0,i=1,r=0;
	  printf("\nenter a binary number \n");
	  scanf("%ld",&bn);
	  while (bn!=0)
	   {
	    r=bn%10;
	    d+=r*i;
	    i=i*2;
	    bn=bn/10;
	   }
	   printf("\ndecimal form is %ld",d);
	  }
	  /*largest of three integers*/ 
void fun25 ()
{
long int a,b,c,g;
/*entering three integer values*/
printf("\nenter three integer values\n");
 scanf("%ld%ld%ld",&a,&b,&c);
 /*logic*/ g=(a>b)&&(a>c)?a:((b>c)?b:c);
 /*printing the highest value obtained*/
 printf("gratest=%ld",g);
}
void fun8 ()/*swapping*/
{
	long int a,b,c,*p,*q;
	/*entering two integer values*/
	printf("\nenter two integer values\n");
	scanf("%ld%ld",&a,&b);
	p=&a,q=&b;
	/*logic*/
	c=*p;
	*p=*q;
	*q=c;
	/*printing the swapped values*/
	printf("values after swapping are\n%ld\t%ld",*p,*q);
}
/*sum of individual digits of a number*/ 
/*sum of individual digits*/
void fun24 ()
{
	long int n,d,sum=0;
printf("\nenter an integer number\n");
scanf("%ld",&n); 
/*logic*/
while (n>0) 
{ 
d=n%10;
 sum+=d;
  n=n/10;
 } 
 /*printing the sum*/
  printf("\nsum of all digits in the number=%ld",sum);
}
	
/*soting*/
void fun7 ()
{
	char a[24][24],b[242];
	long int i,j,n;
	 printf("enter no of items\n");
	 scanf("%d",&n);/*reading number of elements*/
	 printf("\nenter items\n");
	 for (i=0;i<n;i++)
	 scanf("%s",&a[i]);/*reading items*/
	 for (i=0;i<(n-1);i++)
	 {
	 	for (j=i+1;j<n;j++)/*logic*/
	 	{
	 		if (strcmp(a[i],a[j])>0)
	 		{
	 			strcpy (b,a[i]);
	 			strcpy (a[i],a[j]);
	 			strcpy (a[j],b);
	 		}
	 	}
	 }
	 printf("\nitems arranged in assending order are\n\n");
	 for (i=0;i<n;i++)
	 printf("%s\n",a[i]);
	 printf("\nitems arranged in descending order are\n\n");
	  for (i=n-1;i>=0;i--)
	  printf("%s\n",a[i]);
}
void fun20 ()/*multiplication table*/
  {
   long int n,m,i=1; 
   printf("\nenter which table you want\n");
scanf ("%ld",&n);
printf("enter upto where you want\n");
scanf ("%ld",&m);
printf("\nyou desired for %ld table",n);
for (i=1;i<=m;i++)
printf("\n%ld*%ld=%ld",n,i,n*i);
  }
  /*trace*/
void fun5 ()
{
 long int t=0,a[24][24],i,j,m,n;
/*entering order of matrix*/
 printf("\nenter order of matrix\n");
 scanf("%ld%ld",&m,&n);
 /*entering elements*/
 printf("\nenter elements\n");
 for (i=0;i<m;i++)
 {
  for (j=0;j<n;j++)
  {
   scanf("%ld",&a[i][j]);
  }
 }
 printf("\nmatrix\n");
 for (i=0;i<m;i++)
 {
 	printf("\n");
 	for (j=0;j<n;j++)
 	{
 		printf("%ld\t",a[i][j]);
 	}
 }
 /*logic*/
 for (i=0;i<m;i++)
 {
  for (j=0;j<n;j++)
  {
   if (i==j)
   t+=a[i][i];
  }
 }
  printf("\ntrace=%ld",t);
}
/*palindrome*/
void fun11 ()
 {
 long int i,j,f,l;
char a[24];
printf("\nenter a digit or string\n");
gets (a);
l=strlen (a);
for (i=0,j=l-1;i<j;i++,j--)
{
	if (a[i]==a[j])
	{
		f=1;
	}
	else
	{
		f=0;
		break;
	}
}
if (f==1)
printf("\npallindrome");
else
printf("\nnot a pallindrome");
 }
/*mul of individual digits of a number*/ 
 void fun48 ()
{
	long int n,d,mul=1;
printf("\nenter an integer number\n");
scanf("%ld",&n); /*entering an integer value*/
/*logic*/
while (n>0) 
{ 
d=n%10;
 mul*=d;
  n=n/10;
 } 
 /*printing the desired output*/
  printf("\nmultiplication of all digits in the number=%ld",mul);
}
/*linear search*/
void fun13 ()
{
 long int  i,n,m,j=0,f;
 char a[24][24],b[24];
 printf("\nenter number of items\n");
 scanf("%ld",&n);/*entering no of items*/
 printf("\nenter items\n");
 for (i=0;i<n;i++)
 scanf("%s",a[i]);/*entering items*/
 printf("\nenter element to be searched\n");
 scanf("%s",b);
 /*logic*/
 for (i=0;i<n;i++)
 {
  m=strcmp (a[i],b);
  if (m==0)
   {
    f=1;
    j++;
   }
  }
  if (f==1)
  {
   printf("\nrequested item found\n");
   /*printing the repetition*/
   printf("\nit is repeated %ld times",j);
  }
  else
  printf("\ndesired item doesn't exist");
}
 /*binary search*/
void fun10 ()
{
 long int  i,n,m,j=0,f,low=0,high,mid;
 char a[24][24],b[24];
 printf("\nenter number of items\n");
 scanf("%ld",&n);/*entering no of items*/
 printf("\nenter items\n");
 for (i=0;i<n;i++)
 scanf("%s",&a[i]);/*entering items*/
 printf("\nenter element to be searched\n");
 scanf("%s",b);
 high=n-1;
 /*logic*/
 while (low<=high)
 {
  mid=(low+high)/2;
  m=strcmp (b,a[mid]);
  if (m==0)
   {
    f=1;
    j++;
    break;
   }
  else
  {
  if (m<0)
  high=mid-1;
  else
  low=mid+1;
  }
 }
  if (f==1)
   printf("\nrequested item found\n");
  else
  printf("\ndesired item doesn't exist");
}
/*prime numbers*/
void fun60 ()
{
long  int i,n,f,j;
  printf("enter upto where prime numbers had to be printed\n");
  scanf("%ld",&n);
  printf("\nprime numbers upto %ld are\n",n);
  for (i=1;i<=n;++i)
  {
  	f=0;
  	for (j=2;j<=i/2;++j)
  	{
  		if (i%j==0)
  		f=1;
  	}
  if (f==0)
  printf("%ld\t",i);
  }
}
/*2's compliment*/
void fun58 ()
{
 char a[242];
 int i,f,l;
 /*entering a binary number*/
 printf("enter a binary number\n");
 gets (a);
 /*finding the length of the binary*/
 l=strlen (a);
 /*logic*/
 for (i=l-1;i>=0;i--)
 {
  if ((f==0)&&(a[i]=='0'))
     a[i]='1';
 else if ((f==0)&&(a[i]=='1'))
      f=1;
 else if ((f==1)&&(a[i]=='0'))
    a[i]='1';
 else if ((f==1)&&(a[i]=='1'))
    a[i]='0';
 }
 /*printing the 2's compliment*/
 printf("\n2's compliment is\n");
 puts (a);
}
 /*l's compliment*/
void fun59 ()
{
 long int i=0,l;
 char a[24];
/*entering a binary number*/
 printf("\nenter a binary number\n");
 gets (a);
 l=strlen (a);
 /*logic*/
 for (i=0;i<l;i++)
 {
  if (a[i]=='0')
    a[i]='1';
  else
    if (a[i]=='1')
       a[i]='0';
 }
/*printing 1s compliment*/
 printf("\nl's compliment is\n");
 puts (a);
}
 /*upper triangular matrix*/
void fun23 ()
{
 long int m,n,i,j,a[24][24];
 printf("\nenter order of matrix\n");
 scanf("%ld%ld",&m,&n);/*reading order of matrix*/
 printf("\nenter elements\n");
 for (i=0;i<m;i++)
 {
  for (j=0;j<n;j++)/*reding elements*/
  {
   scanf("%ld",&a[i][j]);
  }
 }
 printf("\nmatrix");
  for (i=0;i<m;i++)
 {
 	printf("\n");
  for (j=0;j<n;j++)/*printing matrix*/
  {
   printf("%ld\t",a[i][j]);
  }
 }
 printf("\n\nrequested upper triangular matrix");/*logic*/
 for (i=0;i<m;i++)
 {
  printf("\n");
  for (j=0;j<n;j++)
  {
   if (i>j)
   printf("%ld\t",a[i][j]);
  }
 }
}
 /*lower  triangular matrix*/
void fun22 ()
{
 long int m,n,i,j,a[24][24];
 printf("\nenter order of matrix\n");
 scanf("%ld%ld",&m,&n);/*reading order of matrix*/
 printf("\nenter elements\n");
 for (i=0;i<m;i++)
 {
  for (j=0;j<n;j++)/*reding elements*/
  {
   scanf("%ld",&a[i][j]);
  }
 }
 printf("\nmatrix");
  for (i=0;i<m;i++)
 {
 	printf("\n");
  for (j=0;j<n;j++)/*printing matrix*/
  {
   printf("%ld\t",a[i][j]);
  }
 }
 printf("\n\nrequested lower triangular matrix");/*logic*/
 for (i=0;i<m;i++)
 {
  printf("\n");
  for (j=0;j<n;j++)
  {
   if (i<j)
   printf("%ld\t",a[i][j]);
  }
 }
}
 /*max and min of an array*/
void fun21 ()
{
 long int a[24],max,min,i,n;
 printf("\nenter number of elements\n");
 scanf("%ld",&n);/*reading no of elements*/
 printf("\nenter elements\n");
 for (i=0;i<n;i++)
 scanf("%ld",&a[i]);/*reading elements*/
 max=a[0];
 min=a[0];
 for (i=0;i<n;i++)
 {
  if (a[i]>max)
   max=a[i];
  if (a[i]<min)
   min=a[i];
 }
 printf("\nmaximum is %ld\nminimum is %ld",max,min);
}
/*concatinate two strings*/
void fun72 ()
{
 char a[242],b[242],c[242];
 long int i=0,j=0;
 printf("\nenter first string\n");
 gets (a);
 printf("\nenter second string\n");
 gets (b);
 while (a[i]!='\0')
 {
  c[i]=a[i];
  i++;
 }
 while (b[j]!='\0')
 {
  c[i]=b[j];
  j++;
  i++;
 }
 printf("\ncombined string is\n");
 puts (c);
} 
 /*fibonacci sequence*/
void fun18 ()
{
 long int a,b,c,n;
 /*entering a value*/
 printf("enter a value\n");
 scanf("%ld",&n);
 b=1;
 /*printing the obtained sequence*/
 printf("\nfibonacci sequence upto %ld is\n",n);
 printf("%ld\t%ld\t",a,b);
 c=a+b;
 /*logic*/
 while (c<=n)
 {
    printf("%d\t",c);
  a=b;
  b=c;
  c=a+b;
 }
}/*checking symmetri of a matrix*/
void fun81 ()
 {
long int a[24][24],i,j,m,n,f; 
printf("\nenter order of matrix\n"); /*reading order*/
scanf("%ld%ld",&m,&n); 
	 printf("\nenter elements \n"); /*reading elements*/
	 for (i=0;i<m;i++)
	  {
      for (j=0;j<n;j++) 
       { 
        scanf("%ld",&a[i][j]);
        } 
      } 
 printf("\nmatrix");/*logic*/
 	 for (i=0;i<m;i++)
	  {
	  	printf("\n");
      for (j=0;j<n;j++) 
       { 
        printf("%ld\t",a[i][j]);
        } 
      } 
 for (i=0;i<m;i++)
  {
for (j=0;j<n;j++) 
{
if (a[i][j]==a[j][i])
 f=1;
  else
  {
   f=0; 
   break;
  }
} 
  }
 if (f==1) 
 printf("\n\nmatrix is symmetric"); 
 else
 printf("\n\nmatrix is not symetric");
}
/*reversing a string*/
void fun80 ()
{
 char a[24],b[24];
long  int l,I,j;
printf("\nenter string\n");
 gets (a);
 l=strlen (a);
 /*logic*/
 for (j=0,I=l-1;j!=l;j++,I--)
 {
  b[j]=a[I];
 }
 strcpy (a,b);
 printf("\nreversed string\n");
 puts (a);
}
 /*copmare two strings*/
void fun19()
{
 long int i,f,n;
 char a[242],b[242];
 printf("\nenter first string\n");
 gets (a);
 printf("\nenter second string\n");
 gets (b);
 for (i=0;((a[i]!='\0')||(b[i]!='\0'));i++)
 {
  if (a[i]==b[i])
  {
   f=1;
  }
 else
  {
  	f=0;
  	n=a[i]-b[i];
      break;
  }
 }
 if (f==1)
 printf("\nentered two strings are ::equal::\n");
 else
 {
 printf("\nentered two strings are ::not equal::\n");
 printf("\ndifference in their ASCII value is %ld",n);
 }
}
/*transpose of matrix*/
void fun71 ()
{
 long int m,n,i,j,a[24][24],b[24][24];
 printf("\nenter order of matrix\n");
 scanf("%ld%ld",&m,&n);/*reading order of matrix*/
 printf("\nenter elements\n");
 for (i=0;i<m;i++)
 {
  for (j=0;j<n;j++)
  {
   scanf("%ld",&a[i][j]);
  }
 }
 printf("\nmatrix");
 for (i=0;i<m;i++)
 {
  printf("\n");
  for (j=0;j<n;j++)
  {
   printf("%ld\t",a[i][j]);
  }
 } 
  /*logic*/
 for (i=0;i<m;i++)
 {
  for (j=0;j<n;j++)
  {
   b[j][i]=a[i][j];
  }
 }
 printf("\n\ntransposed matrix");
 for (i=0;i<n;i++)
 {
 	printf("\n");
  for (j=0;j<m;j++)
  {
   printf("%ld\t",b[i][j]);
  }
 } 
}

 