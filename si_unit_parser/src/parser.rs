use std::{collections::VecDeque, ops::Index};

use slk_tokenstream::TokenStream;


#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Hash)]
pub struct FixedPointDecimal {
    integer: VecDeque<Digit>,
    decimal: VecDeque<Digit>,

    unit: Option<UnitType>,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Hash)]
pub struct UnitFixedPointDecimal {
    integer: VecDeque<Digit>,
    decimal: VecDeque<Digit>,

    unit: UnitType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Digit {
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum UnitType {
    Gram,
    Liter,
    Second,
    Meter,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum UnitPrefix {
    Nano,
    Micro,
    Milli,
    Centi,
    Deci,
    None,
    Deca,
    Hecto,
    Kilo,
    Mega,
    Giga,
}

impl FixedPointDecimal {
    pub fn parse(s: impl Into<String>) -> Option<FixedPointDecimal> {
        let backer: Box<[_]> = s.into().chars().collect();
        let mut ts = TokenStream::new(&backer);
        

        let (integer, decimal) = Self::get_parts(&mut ts)?;
        
        let unit = Self::parse_suffix(&mut ts)?;

        let mut unscaled = FixedPointDecimal {
            integer,
            decimal,
            unit: unit.unit,
        };

        unscaled.apply_unit_prefix(unit.prefix);
        Some(unscaled)
    }

    fn get_parts(ts: &mut slk_tokenstream::TokenStream<char>) -> Option<(VecDeque<Digit>, VecDeque<Digit>)> {

        let number = Self::parse_float_parts(ts);

        let (integer, decimal) = match number {
            Some((integer, decimal)) => {
                (integer, decimal)
            },
            None => {
                let i = Self::parse_int(ts);
                if i.is_empty() {
                    return None;
                }
                (i, VecDeque::new())
            },
        };

        Some((integer, decimal))
    }

    fn parse_float_parts(ts: &mut slk_tokenstream::TokenStream<char>) -> Option<(VecDeque<Digit>, VecDeque<Digit>)> {

        let start = ts.mark();
        let integer = Self::parse_int(ts);

        let _ = ts.expect(|c| *c == '.', start)?;

        let decimal = Self::parse_int(ts);

        if decimal.is_empty() && integer.is_empty() {
            return None;
        }

        Some(
            (integer, decimal)
        )
    }

    fn parse_int(ts: &mut slk_tokenstream::TokenStream<char>) -> VecDeque<Digit> {
        let mut digits = VecDeque::new();

        while let Some(digit) = ts.consume_if_some(|c| Digit::try_from_char(*c)) {
            digits.push_back(digit);
        }

        digits
    }

    fn parse_suffix(ts: &mut slk_tokenstream::TokenStream<char>) -> Option<PrefixedUnit>{
        let value = match ts.peek()? {
            'm' => {
                ts.skip();
                if let Some(unit) = UnitType::try_from_ts(ts) {
                    ts.skip();
                    PrefixedUnit { unit: Some(unit), prefix: UnitPrefix::Milli }
                } else {
                    PrefixedUnit { unit: Some(UnitType::Meter), prefix: UnitPrefix::None }
                }
            }
            _ => {
                let prefix = UnitPrefix::from_ts(ts);
                let unit = UnitType::try_from_ts(ts);

                PrefixedUnit { unit, prefix: prefix.unwrap_or(UnitPrefix::None) }
            },
        };
        Some(value)
    }

    fn apply_unit_prefix(&mut self, unit_prefix: UnitPrefix) {
        let amount_to_move = unit_prefix.move_amount();


        if amount_to_move > 0 {
            self.shift_left(amount_to_move as usize);
        } else if amount_to_move < 0 {
            self.shift_right((-amount_to_move) as usize)
        }

        self.trim_zeros();
    }

    fn shift_left(&mut self, amount: usize) {
        for _ in 0..amount {
            self.integer.push_back(self.decimal.pop_front().unwrap_or(Digit::Zero));
        }
    }
    fn shift_right(&mut self, amount: usize) {
        for _ in 0..amount {
            self.decimal.push_front(self.integer.pop_back().unwrap_or(Digit::Zero));
        }
    }

    fn trim_zeros(&mut self) {
        while let Some(_) = self.integer.pop_front_if(|c| *c == Digit::Zero) {}
        while let Some(_) = self.decimal.pop_back_if(|c| *c == Digit::Zero) {}


        if self.integer.is_empty() {
            self.integer.push_front(Digit::Zero);
        }

        if self.decimal.is_empty() {
            self.decimal.push_front(Digit::Zero);
        }
    }

    pub fn decimal<'a>(&'a self) -> NumberSlice<'a> {
        let (front, back) = self.decimal.as_slices();
        NumberSlice { front, back }
    }

    pub fn integer<'a>(&'a self) -> NumberSlice<'a> {
        let (front, back) = self.integer.as_slices();
        NumberSlice { front, back }
    }

    pub fn unit_type(&self) -> Option<UnitType> {
        self.unit
    }
}

impl UnitFixedPointDecimal {

    pub fn decimal<'a>(&'a self) -> NumberSlice<'a> {
        let (front, back) = self.decimal.as_slices();
        NumberSlice { front, back }
    }

    pub fn integer<'a>(&'a self) -> NumberSlice<'a> {
        let (front, back) = self.integer.as_slices();
        NumberSlice { front, back }
    }

    fn apply_unit_prefix(&mut self, unit_prefix: UnitPrefix) {
        let amount_to_move = unit_prefix.move_amount();


        if amount_to_move > 0 {
            self.shift_left(amount_to_move as usize);
        } else if amount_to_move < 0 {
            self.shift_right((-amount_to_move) as usize)
        }

        self.trim_zeros();
    }

    fn shift_left(&mut self, amount: usize) {
        for _ in 0..amount {
            self.integer.push_back(self.decimal.pop_front().unwrap_or(Digit::Zero));
        }
    }
    fn shift_right(&mut self, amount: usize) {
        for _ in 0..amount {
            self.decimal.push_front(self.integer.pop_back().unwrap_or(Digit::Zero));
        }
    }

    fn trim_zeros(&mut self) {
        while let Some(_) = self.integer.pop_front_if(|c| *c == Digit::Zero) {}
        while let Some(_) = self.decimal.pop_back_if(|c| *c == Digit::Zero) {}


        if self.integer.is_empty() {
            self.integer.push_front(Digit::Zero);
        }

        if self.decimal.is_empty() {
            self.decimal.push_front(Digit::Zero);
        }
    }

    pub fn parse_require_unit(s: impl Into<String>) -> Option<UnitFixedPointDecimal> {
        let backer: Box<[_]> = s.into().chars().collect();
        let mut ts = TokenStream::new(&backer);
        

        let (integer, decimal) = FixedPointDecimal::get_parts(&mut ts)?;

        let unit = FixedPointDecimal::parse_suffix(&mut ts)?;

        let mut unscaled = UnitFixedPointDecimal {
            integer,
            decimal,
            unit: unit.unit?,
        };

        unscaled.apply_unit_prefix(unit.prefix);
        Some(unscaled)
    }

    pub fn unit_type(&self) -> UnitType {
        self.unit
    }

    pub const fn new(integer: VecDeque<Digit>, decimal: VecDeque<Digit>, unit: UnitType) -> Self {
        Self { integer, decimal, unit }
    }
}

impl Digit {
    fn try_from_char(c: char) -> Option<Self> {
        let q = match c {
            '0' => Self::Zero,
            '1' => Self::One,
            '2' => Self::Two,
            '3' => Self::Three,
            '4' => Self::Four,
            '5' => Self::Five,
            '6' => Self::Six,
            '7' => Self::Seven,
            '8' => Self::Eight,
            '9' => Self::Nine,

            _ => return None,
        };
        Some(q)
    }
}

impl UnitType {
    fn try_from_ts(ts: &mut TokenStream<char>) -> Option<Self> {
        let v = match ts.peek()? {
            'g' => Self::Gram,
            'L' => Self::Liter,
            's' => Self::Second,
            'm' => Self::Meter,
            _ => return None,
        };
        ts.advance(1);
        Some(v)
    }

    pub fn to_type_string(&self) -> &str {
        match self {
            UnitType::Gram => "Gram",
            UnitType::Liter => "Liter",
            UnitType::Second => "Second",
            UnitType::Meter => "Meter",
        }
    }
}

impl UnitPrefix {
    fn from_ts(ts: &mut TokenStream<char>) -> Option<Self> {
        let v = match ts.peek()? {
            'n' => Self::Nano,
            'u' => Self::Micro,
            'm' => Self::Milli,
            'c' => Self::Centi,
            'd' => {
                if Some(&'a') == ts.peek_offset(1) {
                    ts.advance(1);
                    Self::Deca
                } else {
                    Self::Deci
                }
            }
            'h' => Self::Hecto,
            'k' => Self::Kilo,
            'M' => Self::Mega,
            'G' => Self::Giga,
            _ => return None,
        };

        ts.advance(1);
        Some(v)
    }

    fn move_amount(&self) -> i32 {
        match self {
            UnitPrefix::None => 0,
            UnitPrefix::Nano => -9,
            UnitPrefix::Micro => -6,
            UnitPrefix::Milli => -3,
            UnitPrefix::Centi => -2,
            UnitPrefix::Deci => -1,
            UnitPrefix::Deca => 1,
            UnitPrefix::Hecto => 2,
            UnitPrefix::Kilo => 3,
            UnitPrefix::Mega => 6,
            UnitPrefix::Giga => 9,
        }
    }
}


pub struct NumberSlice<'a> {
    front: &'a [Digit],
    back: &'a [Digit],
}

impl<'a> Index<usize> for NumberSlice<'a> {
    type Output = Digit;

    fn index(&self, index: usize) -> &Self::Output {
        if index < self.front.len() {
            &self.front[index]
        } else {
            &self.back[index - self.front.len()]
        }
    }
}

struct PrefixedUnit {
    unit: Option<UnitType>,
    prefix: UnitPrefix,
}