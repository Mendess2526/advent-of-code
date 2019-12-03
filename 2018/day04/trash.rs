use std::collections::HashMap;
use std::collections::BTreeMap;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

struct TimeTable {
    table: HashMap<Date, BTreeMap<u8, WakeStatus>>,
    guards: HashMap<Date, u32>,
}

#[derive(Clone, PartialEq, Eq)]
enum WakeStatus {
    Woke,
    NotWoke,
}

#[derive(PartialEq, Eq,Hash)]
struct Date {
    day: u8,
    month: u8,
    year: u16,
}

struct TimeStamp {
    date: Date,
    minute: u8,
}

impl TimeTable {
    fn new() -> Self {
        TimeTable {
            table: HashMap::new(),
            guards: HashMap::new(),
        }
    }

    fn add_event(&mut self, tmstmp :TimeStamp, event :WakeStatus) {
        self.table
            .entry(tmstmp.date)
            .or_insert(BTreeMap::new())
            .insert(tmstmp.minute, event);
    }

    fn add_guard(&mut self, tmstmp :TimeStamp, id :u32) {
        self.guards.insert(tmstmp.date, id);
    }

    fn hours_slept(&self, date: Date) -> u32 {
        let mut cur = 0;
        let mut minutes = 0;
        for (min, ev) in self.table.get(&date).unwrap().iter(){
            if *ev == WakeStatus::Woke {
                minutes = (*min - cur).into();
            }
            cur = *min;
        }
        minutes
    }
}

impl<'a> From<&'a str> for TimeStamp {
    fn from(s :&str) -> Self {
        TimeStamp{
            date: Date {
                year: s[1..5].parse().unwrap(),
                month: s[6..8].parse().unwrap(),
                day: s[9..11].parse().unwrap(),
            },
            minute: if &s[12..14] != "00" {
                0
            }else{
                s[16..17].parse().unwrap()
            },
        }
    }
}

impl From<char> for WakeStatus {
    fn from(c :char) -> Self {
        match c {
            'f' => WakeStatus::NotWoke,
            'w' => WakeStatus::Woke,
            _   => unreachable!(),
        }
    }
}

fn main() -> std::io::Result<()> {
    let mut table = TimeTable::new();
    BufReader::new(File::open("input")?)
        .lines()
        .filter_map(|l| l.ok())
        .for_each(|line| {
            let l :&str = &line[..];
            match l.as_bytes()[19] {
                b'G' => table.add_guard(
                    TimeStamp::from(l),
                    l[26..].split(' ').next().unwrap().parse().unwrap()
                    ),
                c   => table.add_event(TimeStamp::from(l), WakeStatus::from(c as char)),
            }
        });
    let mut minutes = HashMap::new();
    for key in table.table.keys().iter() {
        minutes.entry(table.guards.get(key).or_default(-1))
            .and_modify(|x| *x += table.hours_slept(key))
            .or_insert(table.hours_slept(key));
    }
    Ok(())
}
