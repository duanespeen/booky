use booky::app::{App, AppResult};
use booky::database;
use booky::event::{Event, EventHandler};
use booky::handler::handle_key_events;
use booky::reader;
use booky::tui::Tui;
use std::io;
use tui::backend::CrosstermBackend;
use tui::Terminal;

fn main() -> AppResult<()> {
    // Create an application.
    let mut app = App::new();

    //Create booky/books.json in $HOME
    //Change this later.. it should only contain config files afterall..
    reader::create_json();

    // test
    //database::get_books();

    let connection = &mut database::establish_connection();
    let book = database::create_book(
        connection,
        "Can't Hurt Me".to_string(),
        "David Goggins".to_string(),
        "Self-help".to_string(),
        10,
        "Finished".to_string(),
    );
    dbg!("We saved {}", book.id);

    // Initialize the terminal user interface.
    let backend = CrosstermBackend::new(io::stderr());
    let terminal = Terminal::new(backend)?;
    let events = EventHandler::new(250);
    let mut tui = Tui::new(terminal, events);
    tui.init()?;

    // Start the main loop.
    while app.running {
        // Render the user interface.
        tui.draw(&mut app)?;
        // Handle events.
        match tui.events.next()? {
            Event::Tick => app.tick(),
            Event::Key(key_event) => handle_key_events(key_event, &mut app)?,
            Event::Mouse(_) => {}
            Event::Resize(_, _) => {}
        }
    }

    // Exit the user interface.
    tui.exit()?;
    Ok(())
}
