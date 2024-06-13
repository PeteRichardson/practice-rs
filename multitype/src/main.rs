use std::fmt::Display;

#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

// Structs can be reused as fields of another struct
#[derive(Debug)]
struct Rect {
    // A rectangle can be specified by where the top left and bottom right
    // corners are in space.
    top_left: Point,
    bottom_right: Point,
}

impl Display for Rect {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{},{}]", self.top_left, self.bottom_right)
    }
}

#[derive(Debug)]
struct Thing {
    name: String,
    caption: String,
}

impl Thing {
    fn new(name: &str, caption: &str) -> Self {
        Self {
            name: name.to_string(),
            caption: caption.to_string(),
        }
    }
}

impl Widget for Thing {
    fn render(&self, area: &Rect) {
        println!(
            "# {} renders as '{}', area={}",
            self.name, self.caption, area
        );
    }
}

impl EventHandler for Thing {
    fn handle_event(&self, event: Event) {
        match event {
            Event::Enter => println!("# {} says hello!", self.name),
            Event::Leave => println!("# {} says buh-bye!", self.name),
        }
    }
}
impl AppEntity for Thing {}

#[derive(Debug)]
struct Cat {}

impl Cat {
    fn new() -> Self {
        Self {}
    }
}
impl Widget for Cat {
    fn render(&self, area: &Rect) {
        println!("# Cat renders as 'Meow!', area={}", area)
    }
}
impl EventHandler for Cat {
    fn handle_event(&self, event: Event) {
        match event {
            Event::Enter => println!("# Cat says meow!!  (Hello)"),
            Event::Leave => println!("# Cat says meow!!  (Goodbye)"),
        }
    }
}

// Why is this needed?  shouldn't the fact that Cat
// implements EventHandler and Widget good enough?
// There's nothing else to AppEntity ?¿
impl AppEntity for Cat {}

enum Event {
    Enter, // event to trigger a greeting
    Leave, // event to trigger a goodbye message
}

trait EventHandler {
    fn handle_event(&self, event: Event);
}

trait Widget {
    fn render(&self, area: &Rect);
}

trait AppEntity: EventHandler + Widget {}

struct App {
    entities: Vec<Box<dyn AppEntity>>,
}

impl App {
    fn new(entities: Vec<Box<dyn AppEntity>>) -> Self {
        Self { entities }
    }
}

fn main() {
    let t1 = Thing::new("Bob", "I like pickles!");
    let c = Cat::new();
    let t2 = Thing::new("Suzy", "What a glorious day!");

    let app = App::new(vec![Box::new(t1), Box::new(c), Box::new(t2)]);
    let rect = Rect {
        top_left: Point { x: 4.0, y: 5.0 },
        bottom_right: Point { x: 8.0, y: 11.0 },
    };
    for ae in app.entities {
        ae.handle_event(Event::Enter);
        ae.render(&rect);
        ae.handle_event(Event::Leave);
    }
}
