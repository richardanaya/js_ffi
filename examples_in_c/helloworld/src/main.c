 
#define export __attribute__((visibility("default")))
#include "../../js_ffi.h"

export int main() {
	int log = jsffiregister("window.alert");
	jsfficall1(JS_UNDEFINED,log,TYPE_STRING,(JSValue)(int)&"Hello World!");
	return 0;
}