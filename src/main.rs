use iced::widget::{column, keyed, scrollable, text_input, Column, Text};
use iced::window;
use iced::Length::Fill;
use iced::{Center, Size};

pub fn main() -> iced::Result {
    iced::application("what now?", GlobalState::update, GlobalState::view)
        .window(window::Settings {
            size: Size::new(360., 540.),
            position: window::Position::Centered,
            resizable: false,
            decorations: false,
            transparent: true,
            level: window::Level::AlwaysOnTop,
            ..Default::default()
        })
        .run()
}

#[derive(Default)]
struct GlobalState {
    search_query: SearchQuery,
}

enum SearchQuery {
    Program(String),
    Google(String),
}

impl Default for SearchQuery {
    fn default() -> Self {
        Self::Program(String::new())
    }
}

impl GlobalState {
    fn parse_input(&mut self, input: String) {
        self.search_query = if input.starts_with("!g") {
            SearchQuery::Google(input)
        } else {
            SearchQuery::Program(input)
        };
    }

    fn text(&self) -> &String {
        match &self.search_query {
            SearchQuery::Program(v) => v,
            SearchQuery::Google(v) => v,
        }
    }
}

#[derive(Debug, Clone)]
enum Message {
    Input(String),
}

fn programs() -> Vec<String> {
    vec![
        "prog 1", "prog 2", "prog 3", "prog 4", "prog 5", "prog 5", "prog 6", "prog 7", "prog 8",
        "prog 9", "prog 1", "prog 2", "prog 3", "prog 4", "prog 5", "prog 5", "prog 6", "prog 7",
        "prog 8", "prog 9", "prog 1", "prog 2", "prog 3", "prog 4", "prog 5", "prog 5", "prog 6",
        "prog 7", "prog 8", "prog 9", "prog 1", "prog 2", "prog 3", "prog 4", "prog 5", "prog 5",
        "prog 6", "prog 7", "prog 8", "prog 9", "prog 1", "prog 2", "prog 3", "prog 4", "prog 5",
        "prog 5", "prog 6", "prog 7", "prog 8", "prog 9",
    ]
    .into_iter()
    .map(|x| x.to_string())
    .collect()
}

impl GlobalState {
    fn update(&mut self, message: Message) {
        match message {
            Message::Input(v) => self.parse_input(v),
        }
    }

    fn view(&self) -> Column<Message> {
        let text = self.text();
        let input = text_input("Type something worthwhile.", text)
            .width(Fill)
            .on_input(Message::Input)
            .padding(5)
            .size(25)
            .align_x(Center);

        let result = column![input].spacing(10).width(Fill).align_x(Center);

        match self.search_query {
            SearchQuery::Program(_) if !text.is_empty() => result.push(
                scrollable(
                    programs()
                        .into_iter()
                        .filter(|x| x.contains(text))
                        .enumerate()
                        .fold(keyed::Column::new(), |column, (i, program)| {
                            column.push(i, Text::new(program))
                        }),
                )
                .spacing(10)
                .height(Fill)
                .width(Fill),
            ),
            _ => result,
        }
    }
}
