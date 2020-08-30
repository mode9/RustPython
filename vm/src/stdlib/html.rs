use crate::vm::VirtualMachine;
use crate::pyobject::{PyResult, PyObjectRef};
use crate::function::OptionalArg;
use crate::obj::objstr::PyStringRef;

fn html_escape(
    s: PyStringRef,
    quote: OptionalArg<bool>,
) -> String {
    let mut s = s.to_string();
    s = s.replace("&", "&amp;"); // Must be done first!
    s = s.replace("<", "&lt;");
    s = s.replace(">", "&gt;");

    if quote.unwrap_or_else(|| true) {
        s = s.replace('"', "&quot;");
        s = s.replace('\'', "&#x27;");
    }
    s
}
pub fn make_module(vm: &VirtualMachine) -> PyObjectRef {
    let ctx = &vm.ctx;

    py_module!(vm, "html", {
        "escape" => ctx.new_function(html_escape),
    })
}