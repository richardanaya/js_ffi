#include "../../js_ffi.h"

export int main() {
	int log = jsffiregister("function(msg) { $(\"body\").html(msg); }");
	jsfficall1(JS_UNDEFINED,log,TYPE_STRING,(JSValue)(int)&"Hello World!");
	return 0;
}