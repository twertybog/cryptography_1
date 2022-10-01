use fltk::prelude::{InputExt, WidgetExt, MenuExt};
use fltk::{group, input, frame, menu};
use rand::Rng;
use crate::Message;
mod window_changer;
use window_changer::generating_new_template;
mod group_mod;
pub use group_mod::group_mod_creator;
mod prime;
use prime::eu_alg;

pub fn emit(
    msg: Message,
    choice: &mut menu::Choice,
    group_mod_1: &mut group::Group,
    group_mod_2: &mut group::Group,
    m: &mut input::Input,
    task: &mut input::Input,
    result_frame: &mut frame::Frame,
    m_2: &mut input::Input,
    task_2: &mut input::Input,
    result_frame_2: &mut frame::Frame,
    group_generate: &mut group::Group,
    num_range: &mut input::Input,
    result_frame_3: &mut frame::Frame
    )
    {
    match msg {
        Message::Choice => {
            group_mod_1.hide();
            group_mod_2.hide();
            group_generate.hide();
            match choice.value() {
                0 => {
                    //a mod m = x та a^b mod m = x
                    group_mod_1.show();
                    generating_new_template(m, task, result_frame);
                }
                1 => {
                    //a*x ≡ b mod m
                    group_mod_2.show();
                    generating_new_template(m_2, task_2, result_frame_2);
                }
                2 => { //Генерація числа в діапазоні від A до B
                    group_generate.show();
                }
                _ => (),
            }
        }
        Message::ModularExp => {
            let m = m.value().trim().parse::<i128>().unwrap_or(-1);

            let line = task.value();

            let mut splitter = line.split("^");

            let mut base = splitter.next().unwrap()
                .trim().parse::<i128>().unwrap_or(0);
            
            match splitter.next() {
                Some(exponent) => {
                    let mut exp = exponent.trim().parse::<i128>()
                        .unwrap_or(1);
                    if exp == 1{
                        result_frame.set_label(&format!("x = {}", 
                            base.remainder(m)));
                    }
                    else{
                        result_frame.set_label(&format!("x = {}", 
                            base.remainder_with_exp(&mut exp, m)));
                    }
                }
                None => {
                    result_frame.set_label(&format!("x = {}", 
                        base.remainder(m)));
                }
            }
        }
        Message::LinEq => {
            let m = m_2.value().trim().parse::<i128>().unwrap_or(1);
            
            let line = task_2.value();

            let mut splitter = line.split_whitespace();

            let mut a = splitter.next().unwrap_or("None")
                .trim().parse::<i128>().unwrap_or(1);
            
            let b =  splitter.next().unwrap_or("None")
            .trim().parse::<i128>().unwrap_or(1);

            match eu_alg(a, m){
                true =>{
                    let mut counter = 0;

                    for i in 2..m{
                        match eu_alg(i, m) {
                            true => {
                                counter += 1;
                            },
                            false => ()
                        }
                    }
                    
                    let buf = a.remainder_with_exp(&mut counter, m);
                    result_frame_2.set_label(&format!("x = {}",
                        (b * buf) % m));
                },
                false =>{
                    result_frame_2.set_label("Неможливо визначити")
                }
            }
        },
        Message::Generate =>{
            let mut rng = rand::thread_rng();

            let line = num_range.value();
            
            let mut splitter = line.split_whitespace();
            
            let a = splitter.next().unwrap_or("None")
                .trim().parse::<i128>().unwrap_or(i128::MIN);
            
            let b = splitter.next().unwrap_or("None")
                .trim().parse::<i128>().unwrap_or(i128::MAX);

            result_frame_3.set_label(&format!("Згенероване число:\n{}", rng.gen_range(a..=b)));
        }
    }
}

trait EuDiv {
    fn remainder(&self, mo: i128) -> u128;
    fn remainder_with_exp(&mut self, exp: &mut i128, mo: i128) -> i128;
}
impl EuDiv for i128 {
    fn remainder(&self, mo: i128) -> u128 {
        if *self >= 0 {
            (*self % mo) as u128
        } else {
            let result = (((*self / mo) * (-1) + 1) * mo + *self) as u128;
            if mo as u128 == result {
                0
            } else {
                result
            }
        }
    }
    fn remainder_with_exp(&mut self, exp: &mut i128, mo: i128) -> i128{
        let mut r = 1;
        while *exp!=0 {
            if *exp%2 ==1{
                r = (r* *self) % mo;
            }
            *exp = *exp / 2;
            *self = *self * *self % mo;
        }
        r
    }
}