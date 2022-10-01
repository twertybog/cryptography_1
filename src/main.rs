#![windows_subsystem = "windows"]
use fltk::enums::{Align};
use fltk::prelude::{WidgetExt, WidgetBase, MenuExt, GroupExt};
use fltk::{window, app, menu, group, frame, input, button};
mod actions;
use actions::{emit, group_mod_creator};

#[derive(Clone, Copy)]
pub enum Message {
    Choice,
    ModularExp,
    LinEq,
    Generate
}
fn main() {
    let app = app::App::default().with_scheme(app::Scheme::Gtk);

    let mut window = window::Window::default().with_size(400, 300);
    window.set_label("");

    let mut choice = menu::Choice::new(20, 20, 
        360, 30, "");

    choice.add_choice("a mod m = x та a^b mod m = x");
    choice.add_choice("a*x ≡ b mod m");
    choice.add_choice("Генерація числа в діапазоні від A до B");
    
    let mut group_mod_1 = group::Group::new(0, 70, 400, 230, "");

    let (mut m,mut task,
        mut result_frame, mut mod_exp) = group_mod_creator();
    result_frame.set_label_size(18);
    
    let mut expl = frame::Frame::new(5, 70, 
        400, 30, "Ввести {a} чи {a^b}");
    expl.set_align(Align::BottomLeft);

    group_mod_1.end();
    group_mod_1.hide();

    let mut group_mod_2 = group::Group::new(0, 70, 400, 230, "");
    
    let (mut m_2,mut task_2, 
        mut result_frame_2, mut lin_eq) = group_mod_creator();
    result_frame_2.set_label_size(18);
    
    let mut expl_lin = frame::Frame::new(5, 70, 
        400, 30, "Ввести {а} і через пробіл {b}");
    expl_lin.set_align(Align::BottomLeft);

    group_mod_2.end();
    group_mod_2.hide();

    let mut group_generate = group::Group::new(0, 50, 400, 230, "");

    let mut expl_gen = frame::Frame::new(5, 50,
        400, 10, "Ввести {а} і через пробіл {b}");
    expl_gen.set_align(Align::BottomLeft);

    let mut num_range = input::Input::new(80, 80, 
        150, 30, "Діапазон");
    
    let mut gen_button = button::Button::new(240, 80, 
        100, 30, "Згенерувати");

    let mut result_frame_3 = frame::Frame::new(0, 140,
        400, 30, "");

    group_generate.end();
    group_generate.hide();
    
    window.make_resizable(true);
    window.end();
    window.show();
    

    let (s, r) = app::channel();

    choice.emit(s, Message::Choice);
    mod_exp.emit(s, Message::ModularExp);
    lin_eq.emit(s, Message::LinEq);
    gen_button.emit(s, Message::Generate);

    while app.wait() {
        if let Some(msg) = r.recv() {
            emit(msg, &mut choice, 
                &mut group_mod_1, 
                &mut group_mod_2, 
                &mut m, 
                &mut task, 
                &mut result_frame, 
                &mut m_2, 
                &mut task_2, 
                &mut result_frame_2,
                &mut group_generate,
                &mut num_range,
                &mut result_frame_3
            );
        }
    }

    app.run().unwrap();
}