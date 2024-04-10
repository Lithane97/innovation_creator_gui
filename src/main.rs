use iced::{
    button, executor, pick_list, text_input, Application, Button, Column, Command, Container, Element, PickList, Settings, Text, TextInput, Length, Row, Space,
};
use std::{
    borrow::Cow, collections::HashMap, env, fs::{self, OpenOptions}, io::Write, path::PathBuf
};
use regex::Regex;

const INNO_PRESENT_TEMPLATE1: &str = include_str!("C:/Users/litha/OneDrive/Desktop/Shortcuts/Modding/CK3/Rust/innovation_creator_gui/Templates/innovation_present_effect_template1.txt");
const INNO_PRESENT_TEMPLATE2: &str = include_str!("C:/Users/litha/OneDrive/Desktop/Shortcuts/Modding/CK3/Rust/innovation_creator_gui/Templates/innovation_present_effect_template2.txt");
const INNO_PRESENT_TEMPLATE3: &str = include_str!("C:/Users/litha/OneDrive/Desktop/Shortcuts/Modding/CK3/Rust/innovation_creator_gui/Templates/innovation_present_effect_template3.txt");
const INNO_SGUI_TEMPLATE1: &str = include_str!("C:/Users/litha/OneDrive/Desktop/Shortcuts/Modding/CK3/Rust/innovation_creator_gui/Templates/innovation_sgui_template1.txt");
const INNO_SGUI_TEMPLATE2: &str = include_str!("C:/Users/litha/OneDrive/Desktop/Shortcuts/Modding/CK3/Rust/innovation_creator_gui/Templates/innovation_sgui_template2.txt");
const INNO_SGUI_TEMPLATE3: &str = include_str!("C:/Users/litha/OneDrive/Desktop/Shortcuts/Modding/CK3/Rust/innovation_creator_gui/Templates/innovation_sgui_template3.txt");
const TEMPLATE1: &str = include_str!("C:/Users/litha/OneDrive/Desktop/Shortcuts/Modding/CK3/Rust/innovation_creator_gui/Templates/template1.txt");
const TEMPLATE2: &str = include_str!("C:/Users/litha/OneDrive/Desktop/Shortcuts/Modding/CK3/Rust/innovation_creator_gui/Templates/template2.txt");
const TEMPLATE3: &str = include_str!("C:/Users/litha/OneDrive/Desktop/Shortcuts/Modding/CK3/Rust/innovation_creator_gui/Templates/template3.txt");

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
    base_path: PathBuf,
    delete_button_state: button::State,
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

    fn present_effect_file_name(&self) -> &'static str {
        match self {
            Self::Template1 => "innovation_present_effect_template1.txt",
            Self::Template2 => "innovation_present_effect_template2.txt",
            Self::Template3 => "innovation_present_effect_template3.txt",
        }
    }
}

impl ToString for Template {
    fn to_string(&self) -> String {
        String::from(match self {
            Self::Template1 => "Two Innovation Template",
            Self::Template2 => "One Innovation Template",
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
    DeleteOutput,
}

impl Application for MyApp {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Self::Message>) {
        let exe_path = env::current_exe().expect("Failed to get the executable path");
        let base_path = exe_path.parent().expect("Failed to get the executable's directory").to_path_buf();
        
        let app = Self {
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
            base_path, // Initialize this field with the base path
            delete_button_state: button::State::new(),
        };

        app.ensure_directories_exist();

        (app, Command::none())
    }

    fn title(&self) -> String {
        "Innovation Creator".into()
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        let templates_dir = self.base_path.join("Templates");
        
        match message {
            Message::TemplateSelected(template) => {
                self.selected_template = template;
                // Read and store the selected template's content.
                let path = templates_dir.join(template.file_name());
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
                let replacements = vec![
                    ("keyword1".to_string(), self.keyword1_input.clone()),
                    ("keyword2".to_string(), self.keyword2_input.clone()),
                    ("keyword3".to_string(), self.keyword3_input.clone()),
                ];
            
                // Process the primary template.
                let primary_template_path = templates_dir.join(self.selected_template.file_name());
                self.process_template(&primary_template_path, "output.txt", &replacements);
            
                // Process the innovation template.
                let innovation_template_path = templates_dir.join(self.selected_template.innovation_file_name());
                self.process_template(&innovation_template_path, "innovation_sgui_output.txt", &replacements);
            
                // Process the present effect template.
                let present_effect_template_path = templates_dir.join(self.selected_template.present_effect_file_name());
                self.process_template(&present_effect_template_path, "innovation_present_effect_output.txt", &replacements);
            },        
            Message::DeleteOutput => {
                let output_dir = self.base_path.join("Output");
                if let Err(e) = fs::remove_dir_all(&output_dir) {
                    eprintln!("Failed to delete output directory: {}", e);
                }
                fs::create_dir_all(&output_dir).expect("Failed to recreate output directory");
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
            "Enter innovation era...",
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
            "First innovation...",
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
            "Enter second innovation...",
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

        let delete_button = Button::new(&mut self.delete_button_state, Text::new("Delete Output"))
            .on_press(Message::DeleteOutput)
            .padding(10);

        let button_row = Row::new()
            .push(Space::with_width(Length::Fill))
            .push(generate_button)
            .push(Space::with_width(Length::Units(10)))
            .push(delete_button)
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

impl MyApp {
    // Method to ensure the templates directory and files exist
    fn ensure_templates_exist(&self) { // Note &self here
        let templates_dir = self.base_path.join("Templates");

        let template_files = [
            ("template1.txt", TEMPLATE1),
            ("template2.txt", TEMPLATE2),
            ("template3.txt", TEMPLATE3),
            ("innovation_sgui_template1.txt", INNO_SGUI_TEMPLATE1),
            ("innovation_sgui_template2.txt", INNO_SGUI_TEMPLATE2),
            ("innovation_sgui_template3.txt", INNO_SGUI_TEMPLATE3),
            ("innovation_present_effect_template1.txt", INNO_PRESENT_TEMPLATE1),
            ("innovation_present_effect_template2.txt", INNO_PRESENT_TEMPLATE2),
            ("innovation_present_effect_template3.txt", INNO_PRESENT_TEMPLATE3),
        ];

        fs::create_dir_all(&templates_dir).expect("Failed to create Templates directory");

        for (filename, content) in &template_files {
            let filepath = templates_dir.join(filename);
            if !filepath.exists() {
                fs::write(&filepath, content).expect(&format!("Failed to write default content to {}", filename));
            }
        }
    }

    fn write_output_to_file(&self, output_file: &str, content: &str) -> Result<(), std::io::Error> {
        // Construct the path to the output file.
        let file_path = self.base_path.join("Output").join(output_file);
    
        // Attempt to open or create the output file.
        let mut file = OpenOptions::new()
            .append(true)
            .create(true)
            .open(&file_path)?;
    
        // If the file was successfully opened or created, write the content to it.
        if let Err(e) = writeln!(file, "{}", content) {
            eprintln!("Failed to write to output file: {}", e);
            return Err(e);
        }
    
        Ok(())
    }

    fn ensure_directories_exist(&self) {
        self.ensure_templates_exist(); // This now uses self.base_path internally

        // Ensure the output directory exists
        let output_dir = self.base_path.join("Output");
        if !output_dir.exists() {
            fs::create_dir_all(&output_dir).expect("Failed to create output directory");
        }
    }

    fn process_template(&self, template_path: &PathBuf, output_file: &str, replacements: &[(String, String)]) {
        // Attempt to read the template content.
        let template_content = fs::read_to_string(template_path)
            .unwrap_or_else(|_| String::new());

        // Create a HashMap for quick lookup of replacements.
        let replacements_map: HashMap<String, String> = replacements.iter().cloned().collect();

        // Replace each placeholder with its corresponding value in a single pass.
        let re = Regex::new(r"\{\{(\w+)\}\}").unwrap();
        let content = re.replace_all(&template_content, |caps: &regex::Captures| {
            let key = caps.get(1).map_or("", |m| m.as_str());
            replacements_map.get(key).unwrap_or(&String::new()).to_string()
        });

        // Write the processed content to the specified output file.
        let _ = self.write_output_to_file(output_file, &content.to_string());
    }
}