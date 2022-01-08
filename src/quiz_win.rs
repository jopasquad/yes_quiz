use gtk::prelude::*;

pub struct QuizWindow {
    win: gtk::ApplicationWindow,
    fixed: gtk::Fixed,
    yes_btn: gtk::Button,
    no_btn: gtk::Button
}

impl QuizWindow {
    pub fn new(app: &gtk::Application) -> QuizWindow {
        let fixed = gtk::Fixed::new();

        let win = gtk::ApplicationWindow::builder()
            .child(&fixed)
            .default_height(123)
            .default_width(277)
            .application(app)
            .deletable(false)
            .resizable(false)
            .title("Опрос")
            .build();

        let label = gtk::Label::builder()
            .label("Ты даун?")
            .build();

        let yes_btn = gtk::Button::builder()
            .height_request(38)
            .width_request(80)
            .label("Да")
            .build();

        let no_btn = gtk::Button::builder()
            .height_request(38)
            .width_request(80)
            .label("Нет")
            .build();

        fixed.put(&label, 72, 20);
        fixed.put(&yes_btn, 4, 80);
        fixed.put(&no_btn, 192, 80);

        QuizWindow {
            win, fixed, yes_btn, no_btn
        }
    }

    pub fn show(&self) {
        self.win.show_all();
    }
}