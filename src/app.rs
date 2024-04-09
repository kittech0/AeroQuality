use std::process::Termination;

use geoclue_zbus::ManagerProxy;
use gtk::{Application, ApplicationWindow};
use gtk::gio::DBusConnection;
use gtk::glib::ExitCode;
use gtk::prelude::{ApplicationExt, ApplicationExtManual, GtkWindowExt};

use crate::APP_ID;
use geoclue_zbus::ManagerProxy;
use std::process::Termination;

use gtk::gio::DBusConnection;
use gtk::glib::ExitCode;
use gtk::prelude::{ApplicationExt, ApplicationExtManual, GtkWindowExt};
use gtk::{Application, ApplicationWindow};

use crate::error_handler::{BoxResult, GtkError};
use crate::APP_ID;

pub struct App {
    application: Application,
}

impl App {
    pub fn new() -> Self {
        App {
            application: Application::builder().application_id(APP_ID).build(),
        }
    }

    pub fn run(self) -> BoxResult<()> {
        self.application.connect_activate(Self::build_ui);
        match self.application.run() {
            ExitCode::SUCCESS => Ok(()),
            err => {
                err.report();
                Err(Box::new(GtkError(err.value())))
            }
        }
    }

    fn build_ui(app: &Application) {
        let window = ApplicationWindow::builder()
            .application(app)
            .title("My GTK App")
            .build();
        let mut connection = DBusConnection::;
        let client = ManagerProxy::new(&app.get).await?;
        window.present();
    }
}
