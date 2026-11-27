use alex_xp::*;

#[test]
fn everyday() {
    val!(label = s!("Ada"));
    val!(again = label);
    assert_eq!(label, "Ada");
    assert_eq!(again, "Ada");
    show!(label);

    var!(count = 1);
    add!(count, 1);
    assert_eq!(count, 2);
    set!(count = 4);
    assert_eq!(count, 4);

    def!(double = |n| n * 2);
    assert_eq!(double(count), 8);

    give!(handed = s!("bye"));
    assert_eq!(handed, "bye");
}

#[test]
fn setup_greeter() {
    obj!(Greeter {
        who: String,
        fn new(who: String) -> Self {
            Self { who }
        }
        fn hello(&self) -> String {
            s!(self.who)
        }
    });

    val!(person = Greeter::new(s!("Ada")));
    assert_eq!(person.hello(), "Ada");
}

#[test]
fn ownership_tree() {
    struct Node {
        value: i32,
        next: Option<Box<Node>>,
    }

    give!(
        root = Node {
            value: 0,
            next: Some(heap!(Node {
                value: 1,
                next: None
            })),
        }
    );
    assert_eq!(root.next.unwrap().value, 1);
}

#[test]
fn oop_person_student() {
    interface! {
        Person {
            fn name(&self) -> String;
        }
    }

    interface! {
        Student, Person {
            fn university(&self) -> String;
        }
    }

    class! {
        ConcretePerson : [Person {
            fn name(&self) -> String {
                self.name.clone()
            }
        }] {
            name: String,
        } {
            fn new(name: String) -> Self {
                Self { name }
            }
        }
    }

    class! {
        ConcreteStudent : [
            Person {
                fn name(&self) -> String {
                    self.name.clone()
                }
            },
            Student {
                fn university(&self) -> String {
                    self.university.clone()
                }
            }
        ] {
            name: String,
            university: String,
        } {
            fn new(name: String, university: String) -> Self {
                Self { name, university }
            }
        }
    }

    let person = ConcretePerson::new(s!("Alice"));
    let student = ConcreteStudent::new(s!("Bob"), s!("MIT"));
    let people: Vec<&dyn Person> = vec![&person, &student];
    assert_eq!(people[0].name(), "Alice");
    assert_eq!(people[1].name(), "Bob");
    assert_eq!(student.university(), "MIT");
}

#[test]
fn solid_single_responsibility() {
    obj!(Greeter {
        who: String,
        fn new(who: String) -> Self { Self { who } }
        fn hello(&self) -> String { s!(self.who) }
    });

    obj!(Counter {
        n: i32,
        fn new() -> Self { Self { n: 0 } }
        fn bump(&mut self) { self.n += 1; }
    });

    val!(greeter = Greeter::new(s!("Ada")));
    var!(counter = Counter::new());
    counter.bump();
    assert_eq!(greeter.hello(), "Ada");
    assert_eq!(counter.n, 1);
}

#[test]
fn solid_open_closed() {
    iface!(Alarm {
        fn ring(&self) -> String;
    });

    class! {
        Bell : [Alarm {
            fn ring(&self) -> String { s!("ding") }
        }] {} {
            fn new() -> Self { Self {} }
        }
    }

    class! {
        Siren : [Alarm {
            fn ring(&self) -> String { s!("wail") }
        }] {} {
            fn new() -> Self { Self {} }
        }
    }

    let bell = Bell::new();
    let siren = Siren::new();
    let alarms: Vec<&dyn Alarm> = vec![&bell, &siren];
    assert_eq!(alarms[0].ring(), "ding");
    assert_eq!(alarms[1].ring(), "wail");
}

#[test]
fn solid_liskov() {
    interface! {
        Person {
            fn name(&self) -> String;
        }
    }

    interface! {
        Student, Person {
            fn university(&self) -> String;
        }
    }

    class! {
        ConcreteStudent : [
            Person {
                fn name(&self) -> String { self.name.clone() }
            },
            Student {
                fn university(&self) -> String { self.university.clone() }
            }
        ] {
            name: String,
            university: String,
        } {
            fn new(name: String, university: String) -> Self {
                Self { name, university }
            }
        }
    }

    let student = ConcreteStudent::new(s!("Bob"), s!("MIT"));
    let person: &dyn Person = &student;
    assert_eq!(person.name(), "Bob");
}

#[test]
fn solid_interface_segregation() {
    iface!(Printer {
        fn print(&self) -> String;
    });

    iface!(Scanner {
        fn scan(&self) -> String;
    });

    class! {
        Simple : [Printer {
            fn print(&self) -> String { s!("page") }
        }] {} {
            fn new() -> Self { Self {} }
        }
    }

    class! {
        Combo : [
            Printer {
                fn print(&self) -> String { s!("page") }
            },
            Scanner {
                fn scan(&self) -> String { s!("scan") }
            }
        ] {} {
            fn new() -> Self { Self {} }
        }
    }

    fn printed(p: &dyn Printer) -> String {
        p.print()
    }

    let simple = Simple::new();
    let combo = Combo::new();
    assert_eq!(printed(&simple), "page");
    assert_eq!(printed(&combo), "page");
    assert_eq!(combo.scan(), "scan");
}

#[test]
fn solid_dependency_inversion() {
    iface!(Alarm {
        fn ring(&self) -> String;
    });

    class! {
        Bell : [Alarm {
            fn ring(&self) -> String { s!("ding") }
        }] {} {
            fn new() -> Self { Self {} }
        }
    }

    class! {
        Desk {
            alarm: Box<dyn Alarm>,
        } {
            fn new(alarm: Box<dyn Alarm>) -> Self { Self { alarm } }
            fn alert(&self) -> String { self.alarm.ring() }
        }
    }

    give!(desk = Desk::new(heap!(Bell::new())));
    assert_eq!(desk.alert(), "ding");
}

#[test]
fn pattern_factory_method() {
    iface!(Button {
        fn paint(&self) -> String;
    });

    class! {
        WinButton : [Button {
            fn paint(&self) -> String { s!("win") }
        }] {} {
            fn new() -> Self { Self {} }
        }
    }

    iface!(Dialog {
        fn create(&self) -> Box<dyn Button>;
    });

    class! {
        WinDialog : [Dialog {
            fn create(&self) -> Box<dyn Button> { heap!(WinButton::new()) }
        }] {} {
            fn new() -> Self { Self {} }
        }
    }

    let dialog = WinDialog::new();
    let button = dialog.create();
    assert_eq!(button.paint(), "win");
}

#[test]
fn pattern_abstract_factory() {
    iface!(Button {
        fn paint(&self) -> String;
    });

    iface!(Checkbox {
        fn paint(&self) -> String;
    });

    class! {
        WinButton : [Button {
            fn paint(&self) -> String { s!("win-button") }
        }] {} {
            fn new() -> Self { Self {} }
        }
    }

    class! {
        WinCheckbox : [Checkbox {
            fn paint(&self) -> String { s!("win-box") }
        }] {} {
            fn new() -> Self { Self {} }
        }
    }

    iface!(WidgetFactory {
        fn button(&self) -> Box<dyn Button>;
        fn checkbox(&self) -> Box<dyn Checkbox>;
    });

    class! {
        WinFactory : [WidgetFactory {
            fn button(&self) -> Box<dyn Button> { heap!(WinButton::new()) }
            fn checkbox(&self) -> Box<dyn Checkbox> { heap!(WinCheckbox::new()) }
        }] {} {
            fn new() -> Self { Self {} }
        }
    }

    let factory = WinFactory::new();
    assert_eq!(factory.button().paint(), "win-button");
    assert_eq!(factory.checkbox().paint(), "win-box");
}

#[test]
fn pattern_builder() {
    obj!(Burger {
        cheese: bool,
        fn new() -> Self { Self { cheese: false } }
        fn cheese(self) -> Self { Self { cheese: true } }
    });

    let burger = Burger::new().cheese();
    assert!(burger.cheese);
}

#[test]
fn pattern_prototype() {
    obj!(Sheep {
        name: String,
        fn new(name: String) -> Self { Self { name } }
    });

    val!(original = Sheep::new(s!("dolly")));
    val!(copy = original);
    assert_eq!(copy.name, "dolly");
    assert_eq!(original.name, "dolly");
}

#[test]
fn pattern_singleton() {
    obj!(Config {
        path: String,
        fn new(path: String) -> Self { Self { path } }
    });

    fn run(config: &Config) -> String {
        s!(config.path)
    }

    let config = Config::new(s!("app.toml"));
    assert_eq!(run(&config), "app.toml");
}

#[test]
fn pattern_adapter() {
    obj!(Legacy {
        fn new() -> Self { Self {} }
        fn old(&self) -> String { s!("old") }
    });

    iface!(Target {
        fn request(&self) -> String;
    });

    class! {
        Adapter : [Target {
            fn request(&self) -> String { self.legacy.old() }
        }] {
            legacy: Legacy,
        } {
            fn new(legacy: Legacy) -> Self { Self { legacy } }
        }
    }

    give!(adapter = Adapter::new(Legacy::new()));
    assert_eq!(adapter.request(), "old");
}

#[test]
fn pattern_bridge() {
    iface!(Engine {
        fn go(&self) -> String;
    });

    class! {
        Electric : [Engine {
            fn go(&self) -> String { s!("zap") }
        }] {} {
            fn new() -> Self { Self {} }
        }
    }

    iface!(Switch {
        fn flip(&self) -> String;
    });

    class! {
        Toggle : [Switch {
            fn flip(&self) -> String { self.engine.go() }
        }] {
            engine: Box<dyn Engine>,
        } {
            fn new(engine: Box<dyn Engine>) -> Self { Self { engine } }
        }
    }

    give!(toggle = Toggle::new(heap!(Electric::new())));
    assert_eq!(toggle.flip(), "zap");
}

#[test]
fn pattern_composite() {
    iface!(Graphic {
        fn draw(&self) -> String;
    });

    class! {
        Dot : [Graphic {
            fn draw(&self) -> String { s!("dot") }
        }] {} {
            fn new() -> Self { Self {} }
        }
    }

    class! {
        Group : [Graphic {
            fn draw(&self) -> String {
                self.parts.iter().map(|part| part.draw()).collect::<Vec<_>>().join(",")
            }
        }] {
            parts: Vec<Box<dyn Graphic>>,
        } {
            fn new(parts: Vec<Box<dyn Graphic>>) -> Self { Self { parts } }
        }
    }

    give!(group = Group::new(vec![heap!(Dot::new()), heap!(Dot::new())]));
    assert_eq!(group.draw(), "dot,dot");
}

#[test]
fn pattern_decorator() {
    iface!(Named {
        fn name(&self) -> String;
    });

    class! {
        Plain : [Named {
            fn name(&self) -> String { s!("ada") }
        }] {} {
            fn new() -> Self { Self {} }
        }
    }

    class! {
        Loud : [Named {
            fn name(&self) -> String { format!("!!{}", self.inner.name()) }
        }] {
            inner: Box<dyn Named>,
        } {
            fn new(inner: Box<dyn Named>) -> Self { Self { inner } }
        }
    }

    give!(loud = Loud::new(heap!(Plain::new())));
    assert_eq!(loud.name(), "!!ada");
}

#[test]
fn pattern_facade() {
    obj!(Cpu {
        fn new() -> Self { Self {} }
        fn on(&self) -> String { s!("cpu") }
    });

    obj!(Screen {
        fn new() -> Self { Self {} }
        fn on(&self) -> String { s!("screen") }
    });

    obj!(Computer {
        fn new() -> Self { Self {} }
        fn start(&self) -> String {
            format!("{} {}", Cpu::new().on(), Screen::new().on())
        }
    });

    val!(computer = Computer::new());
    assert_eq!(computer.start(), "cpu screen");
}

#[test]
fn pattern_flyweight() {
    obj!(Glyph {
        font: String,
        ch: String,
        fn new(font: String, ch: String) -> Self { Self { font, ch } }
    });

    val!(font = s!("sans"));
    give!(left = Glyph::new(s!(font), s!("A")));
    give!(right = Glyph::new(s!(font), s!("B")));
    assert_eq!(left.font, "sans");
    assert_eq!(right.font, "sans");
    assert_eq!(left.ch, "A");
    assert_eq!(right.ch, "B");
}

#[test]
fn pattern_proxy() {
    iface!(Worker {
        fn work(&self) -> String;
    });

    class! {
        Real : [Worker {
            fn work(&self) -> String { s!("done") }
        }] {} {
            fn new() -> Self { Self {} }
        }
    }

    class! {
        Guard : [Worker {
            fn work(&self) -> String {
                if self.enabled { self.inner.work() } else { s!("blocked") }
            }
        }] {
            inner: Box<dyn Worker>,
            enabled: bool,
        } {
            fn new(inner: Box<dyn Worker>, enabled: bool) -> Self { Self { inner, enabled } }
        }
    }

    give!(blocked = Guard::new(heap!(Real::new()), false));
    give!(open = Guard::new(heap!(Real::new()), true));
    assert_eq!(blocked.work(), "blocked");
    assert_eq!(open.work(), "done");
}

#[test]
fn pattern_chain() {
    iface!(Support {
        fn handle(&self, n: i32) -> String;
    });

    class! {
        Limit : [Support {
            fn handle(&self, n: i32) -> String {
                if n <= self.max {
                    format!("{} handled {}", self.label, n)
                } else if let Some(next) = &self.next {
                    next.handle(n)
                } else {
                    s!("none")
                }
            }
        }] {
            label: String,
            max: i32,
            next: Option<Box<dyn Support>>,
        } {
            fn new(label: String, max: i32, next: Option<Box<dyn Support>>) -> Self {
                Self { label, max, next }
            }
        }
    }

    give!(
        chain = Limit::new(
            s!("small"),
            10,
            Some(heap!(Limit::new(s!("big"), 100, None))),
        )
    );
    assert_eq!(chain.handle(5), "small handled 5");
    assert_eq!(chain.handle(50), "big handled 50");
}

#[test]
fn pattern_command() {
    iface!(Command {
        fn execute(&self) -> String;
    });

    class! {
        Say : [Command {
            fn execute(&self) -> String { self.word.clone() }
        }] {
            word: String,
        } {
            fn new(word: String) -> Self { Self { word } }
        }
    }

    class! {
        Remote {
            slot: Box<dyn Command>,
        } {
            fn new(slot: Box<dyn Command>) -> Self { Self { slot } }
            fn press(&self) -> String { self.slot.execute() }
        }
    }

    give!(remote = Remote::new(heap!(Say::new(s!("hello")))));
    assert_eq!(remote.press(), "hello");
}

#[test]
fn pattern_interpreter() {
    iface!(Expr {
        fn eval(&self) -> i32;
    });

    class! {
        Num : [Expr {
            fn eval(&self) -> i32 { self.value }
        }] {
            value: i32,
        } {
            fn new(value: i32) -> Self { Self { value } }
        }
    }

    class! {
        Add : [Expr {
            fn eval(&self) -> i32 { self.left.eval() + self.right.eval() }
        }] {
            left: Box<dyn Expr>,
            right: Box<dyn Expr>,
        } {
            fn new(left: Box<dyn Expr>, right: Box<dyn Expr>) -> Self {
                Self { left, right }
            }
        }
    }

    give!(sum = Add::new(heap!(Num::new(2)), heap!(Num::new(3))));
    assert_eq!(sum.eval(), 5);
}

#[test]
fn pattern_iterator() {
    obj!(Counter {
        at: i32,
        stop: i32,
        fn new(stop: i32) -> Self { Self { at: 0, stop } }
        fn next(&mut self) -> Option<i32> {
            if self.at >= self.stop {
                None
            } else {
                self.at += 1;
                Some(self.at)
            }
        }
    });

    var!(counter = Counter::new(2));
    assert_eq!(counter.next(), Some(1));
    assert_eq!(counter.next(), Some(2));
    assert_eq!(counter.next(), None);
}

#[test]
fn pattern_mediator() {
    obj!(Tower {
        fn new() -> Self { Self {} }
        fn announce(&self, from: &str, msg: &str) -> String {
            format!("{from}: {msg}")
        }
    });

    obj!(Plane {
        name: String,
        fn new(name: String) -> Self { Self { name } }
        fn radio(&self, tower: &Tower, msg: &str) -> String {
            tower.announce(&self.name, msg)
        }
    });

    val!(tower = Tower::new());
    val!(plane = Plane::new(s!("north")));
    assert_eq!(plane.radio(&tower, "landing"), "north: landing");
}

#[test]
fn pattern_memento() {
    obj!(Editor {
        text: String,
        fn new(text: String) -> Self { Self { text } }
        fn write(&mut self, text: String) { self.text = text; }
    });

    var!(editor = Editor::new(s!("a")));
    val!(saved = editor);
    editor.write(s!("b"));
    assert_eq!(editor.text, "b");
    assert_eq!(saved.text, "a");
}

#[test]
fn pattern_observer() {
    iface!(Listener {
        fn on(&self, msg: &str) -> String;
    });

    class! {
        Log : [Listener {
            fn on(&self, msg: &str) -> String { s!(msg) }
        }] {} {
            fn new() -> Self { Self {} }
        }
    }

    class! {
        Feed {
            subs: Vec<Box<dyn Listener>>,
        } {
            fn new() -> Self { Self { subs: Vec::new() } }
            fn subscribe(&mut self, sub: Box<dyn Listener>) { self.subs.push(sub); }
            fn publish(&self, msg: &str) -> Vec<String> {
                self.subs.iter().map(|sub| sub.on(msg)).collect()
            }
        }
    }

    let mut feed = Feed::new();
    feed.subscribe(heap!(Log::new()));
    assert_eq!(feed.publish("hi"), vec!["hi".to_string()]);
}

#[test]
fn pattern_state() {
    iface!(Mode {
        fn name(&self) -> String;
    });

    class! {
        Off : [Mode {
            fn name(&self) -> String { s!("off") }
        }] {} {
            fn new() -> Self { Self {} }
        }
    }

    class! {
        On : [Mode {
            fn name(&self) -> String { s!("on") }
        }] {} {
            fn new() -> Self { Self {} }
        }
    }

    class! {
        Machine {
            mode: Box<dyn Mode>,
        } {
            fn new(mode: Box<dyn Mode>) -> Self { Self { mode } }
            fn label(&self) -> String { self.mode.name() }
            fn set_mode(&mut self, mode: Box<dyn Mode>) { self.mode = mode; }
        }
    }

    let mut machine = Machine::new(heap!(Off::new()));
    assert_eq!(machine.label(), "off");
    machine.set_mode(heap!(On::new()));
    assert_eq!(machine.label(), "on");
}

#[test]
fn pattern_strategy() {
    iface!(SortStrategy {
        fn label(&self) -> String;
    });

    class! {
        ByName : [SortStrategy {
            fn label(&self) -> String { s!("name") }
        }] {} {
            fn new() -> Self { Self {} }
        }
    }

    class! {
        Report {
            strategy: Box<dyn SortStrategy>,
        } {
            fn new(strategy: Box<dyn SortStrategy>) -> Self { Self { strategy } }
            fn sorted(&self) -> String { self.strategy.label() }
        }
    }

    give!(report = Report::new(heap!(ByName::new())));
    assert_eq!(report.sorted(), "name");
}

#[test]
fn pattern_template_method() {
    interface! {
        Report {
            fn title(&self) -> String;
            fn render(&self) -> String {
                format!("# {}", self.title())
            }
        }
    }

    class! {
        Note : [Report {
            fn title(&self) -> String { s!("note") }
        }] {} {
            fn new() -> Self { Self {} }
        }
    }

    let note = Note::new();
    assert_eq!(note.render(), "# note");
}

#[test]
fn pattern_visitor() {
    enum Shape {
        Circle(i32),
        Square(i32),
    }

    fn area(shape: &Shape) -> i32 {
        match shape {
            Shape::Circle(radius) => radius * radius,
            Shape::Square(side) => side * side,
        }
    }

    let circle = Shape::Circle(3);
    let square = Shape::Square(3);
    assert_eq!(area(&circle), 9);
    assert_eq!(area(&square), 9);
}

#[test]
fn tutorial_program() {
    struct Ticket(i32);

    obj!(Greeter {
        who: String,
        fn new(who: String) -> Self { Self { who } }
        fn hello(&self) -> String { s!(self.who) }
    });

    iface!(Alarm {
        fn ring(&self) -> String;
    });

    class! {
        Bell : [Alarm {
            fn ring(&self) -> String { s!("ding") }
        }] {} {
            fn new() -> Self { Self {} }
        }
    }

    class! {
        Siren : [Alarm {
            fn ring(&self) -> String { s!("wail") }
        }] {} {
            fn new() -> Self { Self {} }
        }
    }

    val!(person = Greeter::new(s!("Ada")));
    assert_eq!(person.hello(), "Ada");
    show!(person.hello());

    var!(count = 1);
    add!(count, 1);
    assert_eq!(count, 2);
    show!(count);
    set!(count = 4);
    assert_eq!(count, 4);
    show!(count);

    val!(label = s!("Ada"));
    val!(again = label);
    assert_eq!(label, "Ada");
    assert_eq!(again, "Ada");
    show!(label);
    show!(again);

    give!(ticket = Ticket(7));
    assert_eq!(ticket.0, 7);
    show!(ticket.0);

    let bell = Bell::new();
    let siren = Siren::new();
    let alarms: Vec<&dyn Alarm> = vec![&bell, &siren];
    assert_eq!(alarms[0].ring(), "ding");
    assert_eq!(alarms[1].ring(), "wail");
    show!(alarms[0].ring());
    show!(alarms[1].ring());
}
