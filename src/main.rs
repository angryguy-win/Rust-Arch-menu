// Import the necessary crossterm modules
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
// Import the necessary ratatui modules
use ratatui::{
    backend::{Backend, CrosstermBackend}, 
    layout::{Alignment, Constraint, Direction, Layout, Rect}, 
    style::{Color, Modifier, Style}, text::{Span, Spans}, 
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph}, 
    Frame, 
    Terminal
};
// Import the necessary serde modules
use serde::{Deserialize, Serialize};
// Import the necessary toml modules
use toml;
// Import the necessary std modules
use std::{io, thread, time::Duration};

// Structure to hold the Arch Linux configuration
#[derive(Serialize, Deserialize, Clone)]
struct ArchConfig {
    hostname: String,
    username: String,
    password: String,
    timezone: String,
    locale: String,
    keyboard_layout: String,
    format_type: String,
    package_manager: String,
    bootloader: String,
    desktop_environment: String,
    reflector_country: String,
    enable_ssh: bool,

}
// Enum to represent different types of questions
enum QuestionType {
    MultipleChoice { options: Vec<String> },
    FreeText,
    Boolean,
}
// Structure to represent a question
struct Question {
    prompt: &'static str,
    question_type: QuestionType,
}
// =========== MAIN FUNCTION =======================================
// Main function to set up the terminal and run the application
fn main() -> Result<(), io::Error> {
    // Setup terminal for TUI
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    // Create app state
    // Initialize the configuration struct
    let mut config = ArchConfig {
        hostname: String::new(),
        username: String::new(),
        password: String::new(),
        timezone: String::new(),
        locale: String::new(),
        keyboard_layout: String::new(),
        format_type: String::new(),
        package_manager: String::new(),
        bootloader: String::new(),
        desktop_environment: String::new(),
        reflector_country: String::new(),
        enable_ssh: false,
    };

    // Run the main application loop
    run_app(&mut terminal, &mut config)?;

    // Restore terminal to original state
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;
    Ok(())
}
// =========== HELPER FUNCTIONS ====================================
// Helper function to draw the splash screen
fn draw_splash_screen<B: Backend>(f: &mut Frame<B>) -> io::Result<()> {
    // Get the size of the terminal
    let size = f.size();
    let block = Block::default().borders(Borders::ALL);
    f.render_widget(block, size);

    let logo = r#"
    _____                .__      .____    .__                     
   /  _  \   _______  __ |  |__   |    |   |__| ____  __ _____  ___
  /  /_\  \ /___/  / |  ||  |  \  |    |   |  |/    \|  |  \  \/  /
 /    |    <  /\  /_/  ||   Y  \ |    |___|  |   |  \  |  />    < 
/\____|__  /\___  /|____|___|  / |_______ \__|___|  /____//__/\_ \
        \/     \/           \/          \/       \/            \/
"#;
    // Split the logo into lines
    let logo_lines: Vec<&str> = logo.lines().collect();
    let logo_height = logo_lines.len() as u16;
    let logo_width = logo_lines.iter().map(|line| line.len()).max().unwrap_or(0) as u16;
    // Calculate the padding for the logo
    let vertical_padding = (size.height.saturating_sub(logo_height)) / 2;
    let horizontal_padding = (size.width.saturating_sub(logo_width)) / 2;
    // Render the logo
    for (i, line) in logo_lines.iter().enumerate() {
        let y = vertical_padding + i as u16;
        if y < size.height {
            let text = Paragraph::new(line.to_string())
                .style(Style::default().fg(Color::Green))
                .alignment(Alignment::Left);
            let area = Rect::new(horizontal_padding, y, logo_width, 1);
            f.render_widget(text, area);
        }
    }

    Ok(())
}

// =========== MAIN APPLICATION LOOP ===============================
// Main application loop
fn run_app<B: ratatui::backend::Backend>(
    terminal: &mut Terminal<B>,
    config: &mut ArchConfig,
    ) -> io::Result<()> {
    // Show splash screen
    terminal.draw(|f| {
        draw_splash_screen(f).expect("Failed to draw splash screen");
    })?;
    thread::sleep(Duration::from_secs(3));

    // Define the list of questions to be asked
    let questions = vec![
        Question {
            prompt: "Hostname",
            question_type: QuestionType::FreeText,
        },
        Question {
            prompt: "Username",
            question_type: QuestionType::FreeText,
        },
        Question {
            prompt: "Password",
            question_type: QuestionType::FreeText,
        },
        Question {
            prompt: "Timezone",
            question_type: QuestionType::MultipleChoice {
                options: vec!["UTC", "America/New_York", "Europe/London", "Asia/Tokyo", "Australia/Sydney"]
                    .into_iter().map(String::from).collect(),
            },
        },
        Question {
            prompt: "Locale",
            question_type: QuestionType::MultipleChoice {
                options: vec!["en_US.UTF-8", "de_DE.UTF-8", "fr_FR.UTF-8", "ja_JP.UTF-8", "zh_CN.UTF-8"]
                    .into_iter().map(String::from).collect(),
            },
        },
        Question {
            prompt: "Keyboard Layout",
            question_type: QuestionType::MultipleChoice {
                options: vec!["us", "de", "fr", "es", "jp"]
                    .into_iter().map(String::from).collect(),
            },
        },
        Question {
            prompt: "Format Type",
            question_type: QuestionType::MultipleChoice {
                options: vec!["btrfs", "ext4", "xfs"]
                    .into_iter().map(String::from).collect(),
            },
        },
        Question {
            prompt: "Package Manager",
            question_type: QuestionType::MultipleChoice {
                options: vec!["pacman", "yay", "paru"]
                    .into_iter().map(String::from).collect(),
            },
        },
        Question {
            prompt: "Bootloader",
            question_type: QuestionType::MultipleChoice {
                options: vec!["grub", "systemd-boot"]
                    .into_iter().map(String::from).collect(),
            },
        },
        Question {
            prompt: "Desktop Environment",
            question_type: QuestionType::MultipleChoice {
                options: vec!["gnome", "kde", "xfce", "dwm", "wayland"]
                    .into_iter().map(String::from).collect(),
            },
        },
        Question {
            prompt: "Reflector Country",
            question_type: QuestionType::MultipleChoice {
                options: vec!["US", "DE", "FR", "CA", "JP"]
                    .into_iter().map(String::from).collect(),
            },
        },  
        Question {
            prompt: "Enable SSH",
            question_type: QuestionType::Boolean,
        },    
    ];

    // Initialize variables for managing the current state
    let mut current_question = 0;
    let mut selected_option = 0;
    let mut input_value = String::new();
    let mut filter = String::new();
    let mut list_state = ListState::default();      

    // Main event loop
    loop {
        // Draw the UI
        terminal.draw(|f| {
            // Create the layout for the UI
            let size = f.size();
            // Create the chunks for the layout
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Min(3),
                    Constraint::Length(3),
                ].as_ref())
                .split(size);
            // Create the main block (border around the entire UI)
            let main_block = Block::default()
                .title("Arch Linux Installer")
                .borders(Borders::ALL);

            // Create the inner block (border around the question)
            if current_question < questions.len() {
            let question = &questions[current_question];
            let inner_block = Block::default().borders(Borders::ALL).title(question.prompt);
        
            // Render the appropriate widget based on the question type
            match &question.question_type {
                // Render the multiple choice question
                QuestionType::MultipleChoice { options } => {
                    // Add question prompt
                    let question_prompt = Paragraph::new(&*question.prompt)
                        .style(Style::default().fg(Color::Green))
                        .block(Block::default().borders(Borders::NONE));
                    f.render_widget(question_prompt , chunks[0]);
                    // Adjust the chunks to make room for the question prompt
                    let inner_chunks = Layout::default()
                        .direction(Direction::Vertical)
                        .constraints([Constraint::Length(3), Constraint::Min(1), Constraint::Length(3)].as_ref())
                        .split(chunks[0]);

                    // Filter and render the list of options
                    let filtered_options: Vec<&String> = options
                        .iter()
                        .filter(|option| option.to_lowercase().contains(&filter.to_lowercase()))
                        .collect();
                    // Create the list widget
                    let items: Vec<ListItem> = filtered_options
                        .iter()
                        .enumerate()
                        .map(|(i, &option)| {
                            let style = if i == selected_option {
                                Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
                            } else {
                                Style::default()
                            };
                            ListItem::new(Spans::from(vec![Span::styled(option.clone(), style)]))
                        })
                        .collect();
                    let list = List::new(items)
                        .block(inner_block)
                        .highlight_style(Style::default().add_modifier(Modifier::BOLD))
                        .highlight_symbol("> ");
                    // Render the list of options
                    list_state.select(Some(selected_option));
                    f.render_stateful_widget(list, inner_chunks[1], &mut list_state);
                    // Render filter input
                    let footer = Paragraph::new(Spans::from(vec![
                        Span::raw("Press Enter to confirm, Arrow keys to navigate, 'q' to quit, Type to filter the choice:"),
                        Span::styled(&filter, Style::default().fg(Color::Yellow)),
                    ]))
                    // Create the footer widget
                    .block(Block::default().borders(Borders::ALL));
                    f.render_widget(footer, chunks[1]);
                },
                // Render the free text input
                QuestionType::FreeText => {
                    let text = vec![
                        Spans::from(vec![
                            Span::raw(question.prompt),
                            Span::raw(": "),
                            Span::styled(&input_value, Style::default().fg(Color::Yellow))
                        ]),
                    ];
                    // Render body frame
                    let body = Paragraph::new(text)
                        .block(inner_block)
                        .wrap(ratatui::widgets::Wrap { trim: true });
                    // Render footer frame
                    let footer = Paragraph::new(Spans::from(vec![
                        Span::raw("Press Enter to confirm, 'q' to quit: "),
                        Span::styled(&filter, Style::default().fg(Color::Yellow)),
                    ]))
                    .block(Block::default().borders(Borders::ALL));
                    // Render the main block and footer
                    f.render_widget(body, chunks[0]);
                    f.render_widget(footer, chunks[1]);
                },
                // Render the boolean question
                QuestionType::Boolean => {
                    // Add question prompt
                    let question_prompt = Paragraph::new(&*question.prompt)
                        .style(Style::default().fg(Color::Green))
                        .block(Block::default().borders(Borders::NONE));
                    f.render_widget(question_prompt, chunks[0]);
                    // Adjust the chunks to make room for the question prompt
                    let inner_chunks = Layout::default()
                        .direction(Direction::Vertical)
                        .constraints([Constraint::Length(3), Constraint::Min(1), Constraint::Length(3)].as_ref())
                        .split(chunks[0]);
                    // Render the list of options
                    let options = vec!["Yes", "No"];
                    let items: Vec<ListItem> = options
                        .iter()
                        .enumerate()
                        .map(|(i, &option)| {
                            let style = if i == selected_option {
                                Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
                            } else {
                                Style::default()
                            };
                            ListItem::new(Spans::from(vec![Span::styled(option, style)]))
                        })
                        .collect();
                    // Create the list widget
                    let list = List::new(items)
                        .block(inner_block)
                        .highlight_style(Style::default().add_modifier(Modifier::BOLD))
                        .highlight_symbol("> ");
                    // Render the list of options
                    list_state.select(Some(selected_option));
                    f.render_stateful_widget(list, inner_chunks[1], &mut list_state);
                    // Render footer
                    let footer = Paragraph::new(Spans::from(vec![
                        Span::raw("Press Enter to confirm, Arrow keys to navigate, 'q' to quit"),
                    ]))
                    // Create the footer widget
                    .block(Block::default().borders(Borders::ALL));
                    f.render_widget(footer, chunks[1]);
                }
            }
            // Render the main block
            f.render_widget(main_block, size);
        }
        })?;

        // Handle key events
        if let Event::Key(key) = event::read()? {
            match key.code {
                // Handle the 'q' key
                KeyCode::Char('q') => return Ok(()),
                KeyCode::Enter => {
                    // Get the selected value based on the current question type
                    let selected_value = match &questions[current_question].question_type {
                        QuestionType::MultipleChoice { options } => {
                            let filtered_options: Vec<&String> = options
                                .iter()
                                .filter(|option| option.to_lowercase().contains(&filter.to_lowercase()))
                                .collect();
                            filtered_options[selected_option].clone()
                        },
                        QuestionType::FreeText => input_value.clone(),
                        QuestionType::Boolean => {
                            if selected_option == 0 { "true".to_string() } else { "false".to_string() }
                        },
                    };
                    // Handle the current question
                    match current_question {
                        0 => config.hostname = selected_value,
                        1 => config.username = selected_value,
                        2 => config.password = selected_value,
                        3 => config.timezone = selected_value,
                        4 => config.locale = selected_value,
                        5 => config.keyboard_layout = selected_value,
                        6 => config.format_type = selected_value,
                        7 => config.package_manager = selected_value,
                        8 => config.bootloader = selected_value,
                        9 => config.desktop_environment = selected_value,
                        10 => config.reflector_country = selected_value,
                        11 => config.enable_ssh = selected_value == "true",
                        _ => {}
                    }
                    current_question += 1;
                    selected_option = 0;
                    input_value.clear();
                    filter.clear();
                    if current_question >= questions.len() {
                        break;
                    }
                },
                // Handle up and down arrow keys
                KeyCode::Up | KeyCode::Down => {
                    let option_count = match &questions[current_question].question_type {
                        QuestionType::MultipleChoice { options } => options.len(),
                        QuestionType::Boolean => 2,
                        _ => 0,
                    };
                    if option_count > 0 {
                        if key.code == KeyCode::Up && selected_option > 0 {
                            selected_option -= 1;
                        } else if key.code == KeyCode::Down && selected_option < option_count - 1 {
                            selected_option += 1;
                        }
                    }
                },
                KeyCode::Char(c) => {
                    match &questions[current_question].question_type {
                        QuestionType::FreeText => input_value.push(c),
                        QuestionType::MultipleChoice { .. } => {
                            filter.push(c);
                            selected_option = 0;
                        },
                        _ => {},
                    }
                },
                KeyCode::Backspace => {
                    match &questions[current_question].question_type {
                        QuestionType::FreeText => { input_value.pop(); },
                        QuestionType::MultipleChoice { .. } => {
                            filter.pop();
                            selected_option = 0;
                        },
                        _ => {},
                    }
                },
                _ => {}
            }
        }
    }
    // Save the final configuration to a file
    let config_toml = toml::to_string_pretty(config).map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
    std::fs::write("arch_config.toml", config_toml)?;

    Ok(())
}
