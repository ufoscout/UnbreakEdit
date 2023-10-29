fn main() {
    let app = MainWindow::new().unwrap();

    let weak = app.as_weak();
    app.on_button_clicked(move || {
        let app = weak.upgrade().unwrap();
        match app.get_status() {
            PlayStatus::Play => {
                app.set_status(PlayStatus::Stop);
            },
            PlayStatus::Stop => {
                app.set_status(PlayStatus::Play);
            }
        }
    });

    app.run().unwrap();
}

// WARN: It is currently not possible to modularize the code below. See:
// - https://github.com/slint-ui/slint/issues/2031
// - https://github.com/slint-ui/slint/issues/784

slint::slint! {

    import { Button, VerticalBox } from "std-widgets.slint";

    enum PlayStatus {
        Play,
        Stop
    }

    export global Play  {

        public pure function statusString(status: PlayStatus) -> string {
            if (status == PlayStatus.Play) {
                @tr("Play")
            } else if (status == PlayStatus.Stop) {
                @tr("Stop")
            } else {
                @tr("Unknown")
            }
        }    
    }


    export component MainWindow inherits Window {

        in property <PlayStatus> status: Stop; 

        callback button_clicked <=> btn.clicked;

        VerticalBox {
            Text {
                text: Play.statusString(status);
                color: green;
            }
            btn := Button {
                text: "click me";
            }
        }
    }
}