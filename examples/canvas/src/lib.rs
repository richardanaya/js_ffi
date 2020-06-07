use js_ffi::*;

#[no_mangle]
fn main() {
    let screen = register_function("document.querySelector").call_1(&DOCUMENT, "#screen").to_js_object();
    let ctx = register_function("HTMLCanvasElement.prototype.getContext").call_1(&screen, "2d").to_js_object();

    let fill_style = register_function("function(color){
        this.fillStyle = color;
    }");
    let fill_rect = register_function("CanvasRenderingContext2D.prototype.fillRect");

    fill_style.call_1(&ctx, "red");
    fill_rect.call_4(&ctx, 0.0, 0.0, 50.0, 50.0);

    fill_style.call_1(&ctx, "green");
    fill_rect.call_4(&ctx, 15.0, 15.0, 50.0, 50.0);

    fill_style.call_1(&ctx, "blue");
    fill_rect.call_4(&ctx, 30.0, 30.0, 50.0, 50.0);
}
