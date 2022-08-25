use std::io::{self, Write};
use std::{thread, time};
use rand::Rng;
use rand::rngs::ThreadRng;
use crossterm::{
    execute,
    event::{self, Event, KeyCode, KeyEvent},
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
    terminal::{self, Clear, ClearType},
    cursor::{DisableBlinking, EnableBlinking, MoveTo, Hide, Show},
    ExecutableCommand, Result,
};

struct Machine {
    winnings: f32,
    tokens: Vec<char>,
    roll_tokens: Vec<String>,
    roll_int: Vec<usize>,
    rng: ThreadRng,
}

impl Machine {
    pub fn default() -> Self {
        Self {
            winnings: 10.0,
            tokens: vec!['🔔', '💀', '🍒', '🍋', '🍊', '⭐' ],
            roll_tokens: vec![],
            roll_int: vec![],
            rng: rand::thread_rng(),
        }
    }

    pub fn roll(&mut self) {
        self.roll_int = vec![
            self.rng.gen_range(0..6),
            self.rng.gen_range(0..5),
            self.rng.gen_range(0..5),
        ];
        self.roll_tokens = vec![
            self.tokens[self.roll_int[0]].to_string(),
            self.tokens[self.roll_int[1]].to_string(),
            self.tokens[self.roll_int[2]].to_string(),
        ];
    }

    pub fn check_token_winnings(&self, token_index: usize) -> f32 {
        let count = self.roll_int.iter().filter(|&n| *n == token_index).count();
        match count {
            1 => 0.0,
            2 => {
                if token_index == 1 {
                    -1.0
                } else {
                    0.5
                }
            },
            3 => {
                if token_index == 0 {
                    5.0
                } else if token_index == 1 {
                    -self.winnings
                } else {
                    1.0
                }
            },
            _ => 0.0,
        }
    }
}

fn run<W>(w: &mut W) -> Result<()>
where
    W: Write,
{
    execute!(w, terminal::EnterAlternateScreen)?;
    terminal::enable_raw_mode()?;

    execute!(
        w,
        MoveTo(0, 0),
        Clear(ClearType::All),
        SetForegroundColor(Color::Blue),
        Print("Welcome to Fruit Machine!\n"),
    )?;

    let mut machine = Machine::default();
    loop {
        let welcome_message = format!("Press q to cash out and any key to roll!\n - Current Total: £{}\n - Price per roll: 20p", (machine.winnings * 100.0).round() / 100.0);
        execute!(
            w,
            Hide,
            MoveTo(0, 1),
            SetForegroundColor(Color::Blue),
            Print(welcome_message),
            ResetColor,
        )?;

        match read_char()? {
            'q' => break,
            _ => {},
        };

        execute!(
            w,
            ResetColor,
            SetForegroundColor(Color::Blue),
        );

        machine.winnings -= 0.2;
        for i in (1..50).step_by(2) {
            machine.roll();
            let roll_string: String = machine.roll_tokens.join(" | ");

            execute!(
                w,
                MoveTo(0, 1),
                Clear(ClearType::FromCursorDown),
                Print(roll_string),
            );

            thread::sleep(time::Duration::from_millis(i * 10));
        }

        let mut game_over = false;
        let mut won_anything = false;

        execute!(
            w,
            MoveTo(0, 5),
            Clear(ClearType::CurrentLine),
        );

        for i in 0..machine.tokens.len() {
            let token_winning = machine.check_token_winnings(i);
            if token_winning != 0.0 {
                won_anything = true;
                machine.winnings += token_winning;
                execute!(
                    w,
                    Print(format!("Spin Result: {}", token_winning))
                );
                thread::sleep(time::Duration::from_secs(3));
            }
        }

        if !won_anything {
            execute!(
                w,
                Print(format!("No Winnings"))
            );
        }
        if game_over { break; }

    }

    execute!(
        w,
        Show,
        MoveTo(0, 0),
        Clear(ClearType::All),
    )?;

    Ok(())
}

pub fn read_char() -> Result<char> {
    loop {
        if let Ok(Event::Key(KeyEvent {
            code: KeyCode::Char(c),
            ..
        })) = event::read()
        {
            return Ok(c);
        }
    }
}

fn main() -> Result<()> {
    let mut stdout = io::stdout();
    run(&mut stdout);
    Ok(())
}
