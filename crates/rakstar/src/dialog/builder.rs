use omp::{dialogs::DialogStyle, players::Player};

pub struct DialogBuilder {
    title: String,
    body: String,
    buttons: [String; 2],
    type_dialog: DialogStyle,
}

impl DialogBuilder {
    pub fn new() -> Self {
        return Self {
            title: "".to_string(),
            body: "".to_string(),
            buttons: ["".to_string(), "".to_string()],
            type_dialog: DialogStyle::MsgBox,
        };
    }

    pub fn title(mut self, title: String) -> Self {
        self.title = title;
        return self;
    }

    pub fn body(mut self, body: String) -> Self {
        self.body = body;
        return self;
    }

    pub fn buttons(mut self, button1: String, button2: String) -> Self {
        self.buttons[0] = button1;
        self.buttons[1] = button2;
        return self;
    }

    pub fn style(mut self, style: DialogStyle) -> Self {
        self.type_dialog = style;
        return self;
    }

    pub fn send(self, to: Player) -> Self {
        to.show_dialog(
            1,
            DialogStyle::List,
            &self.title,
            &self.body,
            &self.buttons[0],
            &self.buttons[1],
        );
        self
    }
}
