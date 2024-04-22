use relm4::{
    gtk::{
        self,
        prelude::{BoxExt, ButtonExt, DialogExt, GtkWindowExt, OrientableExt, WidgetExt},
    },
    ComponentParts, RelmApp, RelmWidgetExt, SimpleComponent,
};

#[derive(Debug)]
enum ConfirmationMessage {
    Cancel,
    Accept,
}

struct ConfirmationModel {
    command: String,
    command_args: Vec<String>,
    message: String,
    cancel_text: String,
    accept_text: String,
}

struct ConfirmationDialog {
    command: String,
    command_args: Vec<String>,
    message: String,
    cancel_text: String,
    accept_text: String,
}

#[relm4::component]
impl SimpleComponent for ConfirmationDialog {
    type Input = ConfirmationMessage;
    type Output = ();
    type Init = crate::Args;

    view! {

        gtk::MessageDialog {
            set_modal: true,
            set_text: Some(&model.message),
            add_button: (&model.cancel_text, gtk::ResponseType::Cancel),
            add_button: (&model.accept_text, gtk::ResponseType::Accept),
            connect_response[sender] => move |_, resp| {
                sender.input(if resp == gtk::ResponseType::Accept {
                    ConfirmationMessage::Accept
                } else {
                    ConfirmationMessage::Cancel
                })
            }
        },

    }

    fn init(
        args: Self::Init,
        window: Self::Root,
        sender: relm4::prelude::ComponentSender<Self>,
    ) -> relm4::prelude::ComponentParts<Self> {
        let model = ConfirmationDialog {
            command: args.command,
            command_args: args.arguments,
            message: args.message,
            cancel_text: args.cancel_text,
            accept_text: args.accept_text,
        };
        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, _sender: relm4::ComponentSender<Self>) {
        match message {
            ConfirmationMessage::Cancel => std::process::exit(0),
            ConfirmationMessage::Accept => {
                if let Err(e) = run_command(&self.command, &self.command_args) {
                    eprintln!("Failed to run command: {e}");
                    std::process::exit(1);
                }
                std::process::exit(0);
            }
        };
    }
}

#[relm4::component]
impl SimpleComponent for ConfirmationModel {
    type Input = ConfirmationMessage;
    type Output = ();
    type Init = crate::Args;

    view! {
        gtk::Window {
            set_resizable: false,
            set_default_width: 500,
            set_default_height: 200,

            gtk::Box {
                set_orientation: gtk::Orientation::Vertical,
                set_spacing: 5,
                set_margin_all: 5,

                gtk::Label {
                    set_label: &model.message
                },

                gtk::Box {
                    set_orientation: gtk::Orientation::Horizontal,
                    set_spacing: 5,
                    set_margin_all: 5,
                    set_margin_horizontal: 10,
                    set_halign: gtk::Align::Center,

                    gtk::Button::with_label(&model.cancel_text) {
                        connect_clicked => ConfirmationMessage::Cancel,
                    },
                    gtk::Button::with_label(&model.accept_text) {
                        connect_clicked => ConfirmationMessage::Accept,
                    },
                },
            }
        }
    }

    fn init(
        args: Self::Init,
        window: Self::Root,
        sender: relm4::prelude::ComponentSender<Self>,
    ) -> relm4::prelude::ComponentParts<Self> {
        let model = ConfirmationModel {
            command: args.command,
            command_args: args.arguments,
            message: args.message,
            cancel_text: args.cancel_text,
            accept_text: args.accept_text,
        };
        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, _sender: relm4::ComponentSender<Self>) {
        match message {
            ConfirmationMessage::Cancel => std::process::exit(0),
            ConfirmationMessage::Accept => {
                if let Err(e) = run_command(&self.command, &self.command_args) {
                    eprintln!("Failed to run command: {e}");
                    std::process::exit(1);
                }
                std::process::exit(0);
            }
        };
    }
}

pub fn run(args: crate::Args) {
    let app: RelmApp<ConfirmationMessage> =
        RelmApp::new("relm4.test.simple_counter").with_args(Vec::new());
    app.run::<ConfirmationDialog>(args);
}

fn run_command(command: &str, args: &[String]) -> Result<std::process::Child, std::io::Error> {
    std::process::Command::new(command).args(args).spawn()
}
