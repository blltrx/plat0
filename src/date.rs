use chrono;

pub struct Date {
    pub year: u32,
    pub month: u8,
    pub day: u8,
}

impl Date {
    pub fn get_iso(&self) -> String {
        //! Returns a string in the format "<year>-<month>-<day>" from self.
        //! ```
        //! println!("date: {}" date.get_iso();
        //! ```
        format!("{:04}-{:02}-{:02}", self.year, self.month, self.day)
    }

    pub fn from_iso(date_string: &str) -> Option<Date> {
        //! Get a date struct from a string in format `"<year>-<month>-<day>"`.
        //!
        //! Splits string at `"-"` and converts to u32, u8, and u8 before assinging to struct.
        //! Returns `Option(None)` if date given is not a valid date with `Date::valid()`
        //!  ```
        //! let mut date = Date::from_iso("2024-04-21");
        //! ```
        let date_split: [u32; 3] = date_string
            .split('-')
            .map(|x| x.parse::<u32>().expect("COULD NOT PARSE TO INT"))
            .collect::<Vec<u32>>()
            .as_slice()
            .try_into()
            .expect("TOO MANY '-'");
        let date = Date {
            year: date_split[0],
            month: u8::try_from(date_split[1]).expect("MONTH NUMBER TO LARGE"),
            day: u8::try_from(date_split[2]).expect("DAY NUMBER TOO LARGE"),
        };
        if !date.valid() {
            return None;
        };
        Some(date)
    }

    fn valid(&self) -> bool {
        if self.year > 2100 {
            return false;
        };
        if self.year < 2001 {
            return false;
        };
        if self.day < 1 {
            return false;
        };
        let max_days = match self.month {
            1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
            4 | 6 | 9 | 11 => 30,
            2 => {
                if self.year % 4 == 0 {
                    30
                } else {
                    29
                }
            }
            _ => 0,
        };
        if self.day > max_days {
            return false;
        }
        true
    }

    pub fn increment_day(&mut self) {
        self.day += 1;
        if !self.valid() {
            self.day = 1;
            self.month += 1;
            if !self.valid() {
                self.month = 1;
                self.year += 1;
            }
        };
    }
}

pub fn get_current_date_iso() -> String {
    let date = chrono::Local::now().format("%Y-%m-%d").to_string();
    date
}
