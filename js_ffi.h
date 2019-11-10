typedef double JSValue;
extern int jsffiregister(char*);
extern void jsfficall1(JSValue,int,int,JSValue);

JSValue const UNDEFINED = 0.0;
int const TYPE_STRING = 2;
