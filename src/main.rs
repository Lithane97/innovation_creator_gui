use iced::{
    button, executor, pick_list, text_input, Application, Button, Column, Command, Container, Element, PickList, Settings, Text, TextInput, Length, Row, Space,
};
use std::{fs, io::Write, path::Path, borrow::Cow};

pub fn main() -> iced::Result {
    MyApp::run(Settings::default())
}

struct MyApp {
    template_pick_list_state: pick_list::State<Template>,
    selected_template: Template,
    keyword1_input: String,
    keyword1_state: text_input::State,
    keyword2_input: String,
    keyword2_state: text_input::State,
    keyword3_input: String,
    keyword3_state: text_input::State,
    replace_button_state: button::State,
    content: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Template {
    Template1,
    Template2,
    Template3,
}

impl Template {
    fn all() -> &'static [Self] {
        &[Self::Template1, Self::Template2, Self::Template3]
    }

    fn file_name(&self) -> &'static str {
        match self {
            Self::Template1 => "template1.txt",
            Self::Template2 => "template2.txt",
            Self::Template3 => "template3.txt",
        }
    }

    fn innovation_file_name(&self) -> &'static str {
        match self {
            Self::Template1 => "innovation_sgui_template1.txt",
            Self::Template2 => "innovation_sgui_template2.txt",
            Self::Template3 => "innovation_sgui_template3.txt",
        }
    }
}

impl ToString for Template {
    fn to_string(&self) -> String {
        String::from(match self {
            Self::Template1 => "Template 1",
            Self::Template2 => "Template 2",
            Self::Template3 => "Template 3",
        })
    }
}

#[derive(Debug, Clone)]
enum Message {
    TemplateSelected(Template),
    Keyword1InputChanged(String),
    Keyword2InputChanged(String),
    Keyword3InputChanged(String),
    ReplaceAndSave,
}

impl Application for MyApp {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Self::Message>) {
        (
            Self {
                template_pick_list_state: pick_list::State::default(),
                selected_template: Template::Template1,
                keyword1_input: String::new(),
                keyword1_state: text_input::State::new(),
                keyword2_input: String::new(),
                keyword2_state: text_input::State::new(),
                keyword3_input: String::new(),
                keyword3_state: text_input::State::new(),
                replace_button_state: button::State::new(),
                content: String::new(),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        "Innovation Creator".into()
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::TemplateSelected(template) => {
                self.selected_template = template;
                // Read and store the selected template's content.
                let path = Path::new("Templates").join(template.file_name());
                self.content = fs::read_to_string(&path).unwrap_or_else(|_| String::new());
            },
            Message::Keyword1InputChanged(value) => {
                self.keyword1_input = value;
            },
            Message::Keyword2InputChanged(value) => {
                self.keyword2_input = value;
            },
            Message::Keyword3InputChanged(value) => {
                self.keyword3_input = value;
            },
            Message::ReplaceAndSave => {
                // Process primary template
                let final_content = self.content
                    .replace("{{keyword1}}", &self.keyword1_input)
                    .replace("{{keyword2}}", &self.keyword2_input)
                    .replace("{{keyword3}}", &self.keyword3_input);
            
                // Append to 'output.txt'
                let mut file = fs::OpenOptions::new()
                    .append(true)
                    .create(true)
                    .open("output.txt")
                    .expect("Unable to open or create file");
                writeln!(file, "{}\n", final_content).expect("Failed to write to file");
            
                // Process corresponding "innovation" template
                let path = Path::new("Templates").join(self.selected_template.innovation_file_name());
                let innovation_content = fs::read_to_string(&path)
                    .unwrap_or_else(|_| String::new())
                    .replace("{{keyword1}}", &self.keyword1_input)
                    .replace("{{keyword2}}", &self.keyword2_input)
                    .replace("{{keyword3}}", &self.keyword3_input);
            
                // Append to 'innovation_sgui_output.txt'
                let mut innovation_file = fs::OpenOptions::new()
                    .append(true)
                    .create(true)
                    .open("innovation_sgui_output.txt")
                    .expect("Unable to open or create innovation file");
                writeln!(innovation_file, "{}\n", innovation_content).expect("Failed to write to innovation file");

                // Process the innovation_present_effect template corresponding to the selected template
                let path_effect = Path::new("Templates").join(format!("innovation_present_effect_template{}.txt", self.selected_template.to_string().split_whitespace().last().unwrap()));
                let effect_content = fs::read_to_string(&path_effect)
                    .unwrap_or_else(|_| String::new())
                    .replace("{{keyword1}}", &self.keyword1_input)
                    .replace("{{keyword2}}", &self.keyword2_input)
                    .replace("{{keyword3}}", &self.keyword3_input);

                // Append to 'innovation_present_effect_output.txt', without adding an extra newline
                let mut effect_file = fs::OpenOptions::new()
                    .append(true)
                    .create(true)
                    .open("innovation_present_effect_output.txt")
                    .expect("Unable to open or create effect file");
                writeln!(effect_file, "{}", effect_content).expect("Failed to write to effect file"); // Use write! to avoid extra newline
            },            
        }
        Command::none()
    }
    

    fn view(&mut self) -> Element<Message> {
        let title = Text::new("Innovation Creator").size(30);
        let subtitle = Text::new("By: Lithane").size(20);
    
        // Wrap text in a Row with Spaces for horizontal centering
        let title_row = Row::new()
            .push(Space::with_width(Length::Fill))
            .push(title)
            .push(Space::with_width(Length::Fill));
    
        let subtitle_row = Row::new()
            .push(Space::with_width(Length::Fill))
            .push(subtitle)
            .push(Space::with_width(Length::Fill));

        // Keyword input fields
        let keyword1_input = TextInput::new(
            &mut self.keyword1_state,
            "Enter keyword1...",
            &self.keyword1_input,
            Message::Keyword1InputChanged,
        )
        .padding(10);

        let keyword1_row = Row::new()
            .push(Space::with_width(Length::Fill))
            .push(keyword1_input)
            .push(Space::with_width(Length::Fill));

        let keyword2_input = TextInput::new(
            &mut self.keyword2_state,
            "Enter keyword2...",
            &self.keyword2_input,
            Message::Keyword2InputChanged,
        )
        .padding(10);

        let keyword2_row = Row::new()
            .push(Space::with_width(Length::Fill))
            .push(keyword2_input)
            .push(Space::with_width(Length::Fill));

        let keyword3_input = TextInput::new(
            &mut self.keyword3_state,
            "Enter keyword3...",
            &self.keyword3_input,
            Message::Keyword3InputChanged,
        )
        .padding(10);

        let keyword3_row = Row::new()
            .push(Space::with_width(Length::Fill))
            .push(keyword3_input)
            .push(Space::with_width(Length::Fill));

        // Generate button
        let generate_button = Button::new(
            &mut self.replace_button_state,
            Text::new("Generate Output")
        )
        .on_press(Message::ReplaceAndSave)
        .padding(10);

        let button_row = Row::new()
            .push(Space::with_width(Length::Fill))
            .push(generate_button)
            .push(Space::with_width(Length::Fill));

        // Template selection dropdown (PickList)
        let template_pick_list = PickList::new(
            &mut self.template_pick_list_state,
            Cow::Borrowed(Template::all()),
            Some(self.selected_template),
            Message::TemplateSelected,
        )
        .padding(10); // Ensure this declaration is before any usage
    
        // Constructing the template_pick_list_row with the template_pick_list
        let template_pick_list_row = Row::new()
            .push(Space::with_width(Length::Fill))
            .push(template_pick_list) // This uses the `template_pick_list`
            .push(Space::with_width(Length::Fill));

        // Combine everything in a Column
        let content = Column::new()
            .spacing(20)
            .push(title_row) // Assuming you have a title_row defined
            .push(subtitle_row) // Assuming you have a subtitle_row defined
            .push(keyword1_row)
            .push(keyword2_row)
            .push(keyword3_row)
            .push(template_pick_list_row)
            .push(button_row);

        // Use a Container for the Column to occupy the full window space
        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}
