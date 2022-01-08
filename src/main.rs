mod quiz_win;

use gtk::prelude::*;

use quiz_win::QuizWindow;

fn main() {
    let app = gtk::ApplicationBuilder::new()
        .flags(gtk::gio::ApplicationFlags::FLAGS_NONE)
        .application_id("ru.jopasquad.yesquiz")
        .build();

    app.connect_activate(|app| {
        let qwin = QuizWindow::new(&app);
        qwin.show();
    });

    app.run();
}