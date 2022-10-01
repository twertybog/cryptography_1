use fltk::{input, frame, button};
use fltk::prelude::WidgetBase;

pub fn group_mod_creator() -> (
    input::Input, 
    input::Input,
    frame::Frame, 
    button::Button)
{   
    (input::Input::new(35, 70, 65, 30, "m = "),
    input::Input::new(5, 120, 165, 30, ""),
    frame::Frame::new(0, 150, 400, 30, ""),
    button::Button::new(185, 120, 100, 30, "Обчислити"))
}