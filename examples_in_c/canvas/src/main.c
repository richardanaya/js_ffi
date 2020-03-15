#include "../../js_ffi.h"

export int main() {
	int fn_querySelector = jsffiregister("document.querySelector");
	int fn_getContext = jsffiregister("HTMLCanvasElement.prototype.getContext");
	int fn_setFill = jsffiregister("function(color){ this.fillStyle = color; }");
	int fn_fillRect = jsffiregister("CanvasRenderingContext2D.prototype.fillRect");

	int screen = jsfficall1(DOCUMENT,fn_querySelector,TYPE_STRING,(JSValue)(int)&"#screen");
	int ctx = jsfficall1(screen,fn_getContext,TYPE_STRING,(JSValue)(int)&"2d");


	jsfficall1(ctx,fn_setFill,TYPE_STRING,(JSValue)(int)&"red");
	jsfficall4(ctx,fn_fillRect,TYPE_NUM,0.0,TYPE_NUM,0.0,TYPE_NUM,50.0,TYPE_NUM,50.0);

	jsfficall1(ctx,fn_setFill,TYPE_STRING,(JSValue)(int)&"green");
	jsfficall4(ctx,fn_fillRect,TYPE_NUM,15.0,TYPE_NUM,15.0,TYPE_NUM,50.0,TYPE_NUM,50.0);

	jsfficall1(ctx,fn_setFill,TYPE_STRING,(JSValue)(int)&"blue");
	jsfficall4(ctx,fn_fillRect,TYPE_NUM,30.0,TYPE_NUM,30.0,TYPE_NUM,50.0,TYPE_NUM,50.0);
	return 0;
}