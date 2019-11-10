typedef double JSValue;
extern void jsffirelease(JSValue);
extern int jsffiregister(char*);
extern void jsfficall0(JSValue,int);
extern void jsfficall1(JSValue,int,int,JSValue);
extern void jsfficall2(JSValue,int,int,JSValue,int,JSValue);
extern void jsfficall3(JSValue,int,int,JSValue,int,JSValue,int,JSValue);
extern void jsfficall4(JSValue,int,int,JSValue,int,JSValue,int,JSValue,int,JSValue);
extern void jsfficall5(JSValue,int,int,JSValue,int,JSValue,int,JSValue,int,JSValue,int,JSValue);
extern void jsfficall6(JSValue,int,int,JSValue,int,JSValue,int,JSValue,int,JSValue,int,JSValue,int,JSValue);
extern void jsfficall7(JSValue,int,int,JSValue,int,JSValue,int,JSValue,int,JSValue,int,JSValue,int,JSValue,int,JSValue);
extern void jsfficall8(JSValue,int,int,JSValue,int,JSValue,int,JSValue,int,JSValue,int,JSValue,int,JSValue,int,JSValue,int,JSValue);
extern void jsfficall9(JSValue,int,int,JSValue,int,JSValue,int,JSValue,int,JSValue,int,JSValue,int,JSValue,int,JSValue,int,JSValue,int,JSValue);
extern void jsfficall10(JSValue,int,int,JSValue,int,JSValue,int,JSValue,int,JSValue,int,JSValue,int,JSValue,int,JSValue,int,JSValue,int,JSValue,int,JSValue);

JSValue const JS_UNDEFINED = 0.0;
JSValue const JS_NULL = 1.0;
JSValue const JS_FALSE = 0.0;
JSValue const JS_TRUE = 1.0;
JSValue const CONSOLE = 2.0;
JSValue const WINDOW = 3.0;
JSValue const DOCUMENT = 4.0;

int const TYPE_NOTHING = 0;
int const TYPE_NUM = 1;
int const TYPE_STRING = 2;
int const TYPE_BOOL = 3;
int const TYPE_FUNCTION = 4;
int const TYPE_OBJ = 5;
int const TYPE_UINT8_ARRAY = 6;
int const TYPE_INT8_ARRAY = 7;
int const TYPE_UINT8CLAMPED_ARRAY = 8;
int const TYPE_INT16_ARRAY = 9;
int const TYPE_UINT16_ARRAY = 10;
int const TYPE_INT32_ARRAY = 11;
int const TYPE_UINT32_ARRAY = 12;
int const TYPE_F32_ARRAY = 13;
int const TYPE_F64_ARRAY = 14;
int const TYPE_BI64_ARRAY = 15;
int const TYPE_BUI64_ARRAY = 16;
int const TYPE_MEMORY = 17;
