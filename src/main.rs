use fltk::{app::*, enums::*, frame::*, input::*, prelude::*, button::*, window::*};
use rustube;
use fltk::text::{TextBuffer, SimpleTerminal};

// imported fltk and rustube dependencies

const WIDGET_HEIGHT: i32 = 50;
const WIDGET_PADDING: i32 = 10;
const WIDGET_WIDTH: i32 = 200;

// establish widget dimensions

#[derive(Clone, Copy)]
enum Message {
    HasLink,
    DownloadReady,
    DownloadFinished,
}

// create state enums

fn main() {
    let app = App::default().with_scheme(Scheme::Gtk);
    let mut wind = Window::default()
        .with_size(
            WIDGET_WIDTH * 2 + WIDGET_PADDING * 4,
            WIDGET_HEIGHT * 2 + WIDGET_PADDING,
        )
        .with_label("tubru");

    let (sender, reciever) = channel::<Message>();

    let mut link_input = Input::default()
        .with_size(WIDGET_WIDTH * 2 + (WIDGET_PADDING * 2), WIDGET_HEIGHT / 2)
        .with_pos(WIDGET_PADDING, WIDGET_PADDING);
    link_input.set_trigger(CallbackTrigger::Changed);
    link_input.emit(sender, Message::HasLink);

    let mut download = Button::default()
        .with_size(WIDGET_WIDTH, WIDGET_HEIGHT)
        .below_of(&link_input, WIDGET_PADDING)
        .with_label("download");
    download.emit(sender, Message::DownloadReady);
 
    let mut status = SimpleTerminal::default()
        .with_size(WIDGET_WIDTH + WIDGET_PADDING, WIDGET_HEIGHT)
        .right_of(&download, WIDGET_PADDING);

    download.deactivate();

    link_input.set_color(Color::from_u32(0x2e3440));
    link_input.set_text_color(Color::from_u32(0xeceff4));
    link_input.set_selection_color(Color::from_u32(0x5e81ac));

    download.set_color(Color::from_u32(0x4c566a));
    download.set_label_size(32);

    wind.set_color(Color::from_u32(0x353c4a));

    wind.end();
    wind.show();
    while app.wait() {
        match reciever.recv() {
            Some(Message::HasLink) => {
                status.set_text("Press enter to start download");
                if &link_input.value() != "" {
                    download.activate();
                } else {
                    download.deactivate();
                }
            }
            Some(Message::DownloadReady) => {
                link_input.deactivate();
                status.set_text("Download in progress...");

                let link: String = link_input.value();
                
                video_downloadfn(&link);

                link_input.emit(sender, Message::DownloadFinished);
            }
            Some(Message::DownloadFinished) => {
                // empty for now
            }
            None => {}
        }
    }
}

async fn video_downloadfn(vidLink: &str) -> String { 
    let download_loc = rustube::download_best_quality(&vidLink).await.unwrap();
    return download_loc.into_os_string().into_string().unwrap();
}