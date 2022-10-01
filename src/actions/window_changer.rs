use fltk::{frame, input};
use fltk::prelude::{WidgetExt, InputExt};

pub fn generating_new_template(
    m: &mut input::Input,
    task: &mut input::Input,
    result_frame: &mut frame::Frame,
) {
    m.set_value("");
    task.set_value("");
    result_frame.set_label("");
    m.show();
    task.show();
    result_frame.show();
}