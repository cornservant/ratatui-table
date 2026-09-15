//! # [Ratatui] `Table` example
//!
//! The latest version of this example is available in the [widget examples] folder in the
//! repository.
//!
//! Please note that the examples are designed to be run against the `main` branch of the Github
//! repository. This means that you may not be able to compile with the latest release version on
//! crates.io, or the one that you have installed locally.
//!
//! See the [examples readme] for more information on finding examples that match the version of the
//! library you are using.
//!
//! [Ratatui]: https://github.com/ratatui/ratatui
//! [widget examples]: https://github.com/ratatui/ratatui/blob/main/ratatui-widgets/examples
//! [examples readme]: https://github.com/ratatui/ratatui/blob/main/examples/README.md

use color_eyre::Result;
use crossterm::event::{self, KeyCode};
use ratatui::Frame;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::{Color, Style, Stylize};
use ratatui::text::{Line, Span};
use ratatui_table::{Row, Table, TableState};

fn main() -> Result<()> {
    color_eyre::install()?;

    let mut table_state = TableState::default();
    table_state.select_first();
    table_state.select_first_column();
    ratatui::run(|terminal| {
        loop {
            terminal.draw(|frame| render(frame, &mut table_state))?;
            if let Some(key) = event::read()?.as_key_press_event() {
                match key.code {
                    KeyCode::Char('q') | KeyCode::Esc => return Ok(()),
                    KeyCode::Char('j') | KeyCode::Down => table_state.select_next(),
                    KeyCode::Char('k') | KeyCode::Up => table_state.select_previous(),
                    KeyCode::Char('l') | KeyCode::Right => table_state.select_next_column(),
                    KeyCode::Char('h') | KeyCode::Left => table_state.select_previous_column(),
                    KeyCode::Char('g') => table_state.select_first(),
                    KeyCode::Char('G') => table_state.select_last(),
                    _ => {}
                }
            }
        }
    })
}

/// Render the UI with a table.
fn render(frame: &mut Frame, table_state: &mut TableState) {
    let layout = Layout::vertical([Constraint::Length(1), Constraint::Fill(1)]).spacing(1);
    let [top, main] = frame.area().layout(&layout);

    let title = Line::from_iter([
        Span::from("Table Widget").bold(),
        Span::from(" (Press 'q' to quit and arrow keys to navigate)"),
    ]);
    frame.render_widget(title.centered(), top);

    render_table(frame, main, table_state);
}

/// Render a table with some rows and columns.
pub fn render_table(frame: &mut Frame, area: Rect, table_state: &mut TableState) {
    let header = Row::new(["Ingredient", "Quantity", "Macros"])
        .style(Style::new().bold())
        .bottom_margin(1);

    let rows = [
        Row::new(["Eggplant", "1 medium", "25 kcal, 6g carbs, 1g protein"]),
        Row::new(["Tomato", "2 large", "44 kcal, 10g carbs, 2g protein"]),
        Row::new(["Zucchini", "1 medium", "33 kcal, 7g carbs, 2g protein"]),
        Row::new(["Bell Pepper", "1 medium", "24 kcal, 6g carbs, 1g protein"]),
        Row::new(["Garlic", "2 cloves", "9 kcal, 2g carbs, 0.4g protein"]),
        Row::new([
            "Sunsilks (Solar Wheat)",
            "1 sheaf",
            "180 kcal, 38g carbs, 4g protein",
        ]),
        Row::new(["Dragon Root", "1 node", "110 kcal, 22g carbs, 3g protein"]),
        Row::new([
            "Voidcap Mushroom",
            "3 caps",
            "12 kcal, 1g carbs, 3g protein",
        ]),
        Row::new([
            "Starlight Kelp",
            "1 strand",
            "8 kcal, 2g carbs, 0.5g protein",
        ]),
        Row::new(["Moon-Melon", "1 slice", "65 kcal, 16g carbs, 1g protein"]),
        Row::new(["Glowberry", "1 handful", "42 kcal, 9g carbs, 0.8g protein"]),
        Row::new([
            "Frost-Lichen",
            "1 pinch",
            "2 kcal, 0.5g carbs, 0.1g protein",
        ]),
        Row::new(["Nebula Beans", "1 cup", "210 kcal, 32g carbs, 14g protein"]),
        Row::new(["Ember Pepper", "1 pod", "15 kcal, 3g carbs, 0.6g protein"]),
        Row::new(["Aether Plum", "1 fruit", "55 kcal, 13g carbs, 0.7g protein"]),
        Row::new([
            "Shadow-Stalk Celery",
            "2 ribs",
            "14 kcal, 3g carbs, 0.7g protein",
        ]),
        Row::new(["Titan Pumpkin", "1 wedge", "85 kcal, 20g carbs, 2g protein"]),
        Row::new([
            "Crystalized Nectar",
            "1 crystal",
            "60 kcal, 15g carbs, 0g protein",
        ]),
        Row::new(["Phased Onion", "1 bulb", "38 kcal, 9g carbs, 1.2g protein"]),
        Row::new([
            "Bioluminescent Fern",
            "1 frond",
            "5 kcal, 1g carbs, 0.3g protein",
        ]),
        Row::new([
            "Chrono-Vine Leaf",
            "4 leaves",
            "18 kcal, 4g carbs, 1g protein",
        ]),
        Row::new(["Plasma Gourd", "1 small", "40 kcal, 8g carbs, 1.5g protein"]),
        Row::new([
            "Iron-Bark Potato",
            "1 tuber",
            "160 kcal, 36g carbs, 4g protein",
        ]),
        Row::new(["Astral Truffle", "1 oz", "75 kcal, 8g carbs, 5g protein"]),
        Row::new([
            "Mana-Infused Rice",
            "1 cup",
            "220 kcal, 48g carbs, 4.5g protein",
        ]),
        Row::new(["Silver Spores", "1 pinch", "0 kcal, 0g carbs, 0g protein"]),
        Row::new([
            "Solaris Orange",
            "1 fruit",
            "62 kcal, 15g carbs, 1.2g protein",
        ]),
        Row::new([
            "Abyssal Algae",
            "1 sheet",
            "10 kcal, 1g carbs, 1.8g protein",
        ]),
        Row::new([
            "Wyrm-Shatter Bean",
            "1 pod",
            "95 kcal, 14g carbs, 8g protein",
        ]),
        Row::new([
            "Mirage Cucumber",
            "1 medium",
            "16 kcal, 3.8g carbs, 0.7g protein",
        ]),
        Row::new([
            "Celestial Fig",
            "2 dried",
            "110 kcal, 26g carbs, 1g protein",
        ]),
        Row::new([
            "Zero-G Spinach",
            "2 cups",
            "14 kcal, 2g carbs, 1.8g protein",
        ]),
        Row::new([
            "Lava-Dusted Ginger",
            "1 thumb",
            "19 kcal, 4g carbs, 0.4g protein",
        ]),
        Row::new([
            "Vortex Spinach",
            "1 bundle",
            "22 kcal, 3.5g carbs, 2.5g protein",
        ]),
        Row::new(["Echo Apple", "1 fruit", "95 kcal, 25g carbs, 0.5g protein"]),
        Row::new([
            "Quantum Radish",
            "3 slices",
            "6 kcal, 1.2g carbs, 0.2g protein",
        ]),
        Row::new([
            "Banshee Berry",
            "1 handful",
            "30 kcal, 7g carbs, 0.4g protein",
        ]),
        Row::new([
            "Glacier Mint",
            "5 leaves",
            "1 kcal, 0.2g carbs, 0.1g protein",
        ]),
        Row::new([
            "Obsidian Olive",
            "5 pitted",
            "45 kcal, 2g carbs, 0.5g protein",
        ]),
        Row::new([
            "Supernova Squash",
            "1 cup",
            "52 kcal, 13g carbs, 1.4g protein",
        ]),
        Row::new(["Prism Corn", "1 ear", "90 kcal, 19g carbs, 3.2g protein"]),
        Row::new([
            "Flux-Flow Cabbage",
            "1/2 head",
            "35 kcal, 8g carbs, 2g protein",
        ]),
        Row::new([
            "Grav-Grape",
            "1 cluster",
            "70 kcal, 18g carbs, 0.6g protein",
        ]),
        Row::new([
            "Ethereal Turnip",
            "1 small",
            "28 kcal, 6g carbs, 0.9g protein",
        ]),
        Row::new(["Pulse Lettuce", "1 head", "15 kcal, 3g carbs, 1.2g protein"]),
        Row::new([
            "Singularity Seed",
            "1 tbsp",
            "55 kcal, 5g carbs, 2.5g protein",
        ]),
        Row::new([
            "Bramble-Spine Artichoke",
            "1 heart",
            "45 kcal, 10g carbs, 3g protein",
        ]),
        Row::new([
            "Temporal Thyme",
            "2 sprigs",
            "1 kcal, 0.3g carbs, 0.1g protein",
        ]),
        Row::new([
            "Aura Asparagus",
            "5 spears",
            "20 kcal, 3.8g carbs, 2.2g protein",
        ]),
        Row::new([
            "Warp-Weaved Wheatgrass",
            "1 shot",
            "10 kcal, 2g carbs, 1g protein",
        ]),
        Row::new([
            "Dust-Stalker Cactus",
            "1 pad",
            "24 kcal, 5g carbs, 1g protein",
        ]),
        Row::new([
            "Hyper-Hydrated Melon",
            "1 wedge",
            "50 kcal, 12g carbs, 0.8g protein",
        ]),
        Row::new([
            "Dwarven Stone-Bread Fruit",
            "1 chunk",
            "280 kcal, 55g carbs, 9g protein",
        ]),
        Row::new(["Arcane Fennel", "1 bulb", "27 kcal, 6g carbs, 1.1g protein"]),
        Row::new([
            "Siren-Call Strawberry",
            "4 berries",
            "32 kcal, 7.5g carbs, 0.7g protein",
        ]),
        Row::new([
            "Bio-Tech Bamboo Shoot",
            "1 cup",
            "41 kcal, 8g carbs, 3.9g protein",
        ]),
        Row::new([
            "Phantom Parsnip",
            "1 root",
            "75 kcal, 17g carbs, 1.2g protein",
        ]),
        Row::new([
            "Nectarine of the Gods",
            "1 fruit",
            "60 kcal, 14g carbs, 1.4g protein",
        ]),
        Row::new([
            "Spectral Shallot",
            "2 medium",
            "15 kcal, 3.3g carbs, 0.3g protein",
        ]),
        Row::new(["Neutron Nut", "1 nut", "130 kcal, 4g carbs, 3g protein"]),
        Row::new([
            "Spore-Cloud Mustard Leaf",
            "1 cup",
            "15 kcal, 2.7g carbs, 1.6g protein",
        ]),
        Row::new(["Cosmic Yam", "1 medium", "115 kcal, 27g carbs, 2g protein"]),
        Row::new([
            "Faerie Cherry",
            "10 pitless",
            "50 kcal, 12g carbs, 1g protein",
        ]),
        Row::new([
            "Glitch-Grain",
            "1/2 cup",
            "120 kcal, 24g carbs, 3.5g protein",
        ]),
        Row::new([
            "Deep-Core Beet",
            "1 tuber",
            "43 kcal, 10g carbs, 1.6g protein",
        ]),
        Row::new([
            "Chrono-Bloom Petal",
            "1 pinch",
            "0.5 kcal, 0.1g carbs, 0g protein",
        ]),
        Row::new([
            "Astro-Avocado",
            "1/2 fruit",
            "160 kcal, 9g carbs, 2g protein",
        ]),
    ];
    let footer = Row::new([
        "Ratatouille Recipe",
        "",
        "135 kcal, 31g carbs, 6.4g protein",
    ]);
    let widths = [
        Constraint::Percentage(30),
        Constraint::Percentage(20),
        Constraint::Percentage(50),
    ];
    let table = Table::new(rows, widths)
        .header(header)
        .footer(footer.italic())
        .column_spacing(1)
        .style(Color::White)
        .row_highlight_style(Style::new().on_black().bold())
        .column_highlight_style(Color::Gray)
        .cell_highlight_style(Style::new().reversed().yellow())
        .highlight_symbol("🍴 ");

    frame.render_stateful_widget(table, area, table_state);
}
