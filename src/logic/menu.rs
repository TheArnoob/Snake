use strum::{EnumCount, EnumIter, FromRepr, IntoEnumIterator};

#[derive(Debug, Copy)]
pub struct Menu {
    selected_option: SelectedOption,
}

impl Clone for Menu {
    fn clone(&self) -> Self {
        Self {
            selected_option: self.selected_option,
        }
    }
}

impl Menu {
    pub fn new() -> Menu {
        Menu {
            selected_option: SelectedOption::default(),
        }
    }

    pub fn update(&mut self, _now: std::time::Instant) {}

    pub fn selected_option(&self) -> SelectedOption {
        self.selected_option
    }

    pub fn select_next_option(&mut self) {
        self.selected_option.next_selection();
    }

    pub fn select_previous_option(&mut self) {
        self.selected_option.previous_selection();
    }
}
#[derive(Debug, Default, EnumIter, PartialEq, FromRepr, Clone, Copy, EnumCount)]
#[repr(u8)]
pub enum SelectedOption {
    #[default]
    NewGame,
    Options, /*(Options)*/
    Exit,
}

// #[derive(Debug, Clone, Copy, PartialEq, Default)]
// #[repr(u8)]
// enum Options {
//     #[default]
//     Difficulty(Difficulty),
//     GrowthPerFood(usize),
//     GameSize((usize, usize)),
// }

// #[derive(Debug, Clone, Copy, PartialEq)]
// enum Difficulty {
//     Easy,
//     Medium,
//     Hard,
// }

impl SelectedOption {
    pub fn all_possibilities() -> Vec<SelectedOption> {
        SelectedOption::iter().collect()
    }

    pub fn next_selection(&mut self) {
        if Self::COUNT - 1 == *self as usize {
            return *self = Self::from_repr(0).expect("Cannot Fail");
        }
        *self = Self::from_repr((*self as u8) + 1).expect("Cannot Fail")
    }

    pub fn previous_selection(&mut self) {
        if 0 == *self as usize {
            return *self = Self::from_repr((Self::COUNT - 1) as u8).expect("Cannot Fail");
        }
        *self = Self::from_repr((*self as u8) - 1).expect("Cannot Fail")
    }

    pub fn menu_text(&self) -> &str {
        match self {
            SelectedOption::NewGame => "New Game",
            SelectedOption::Options => "Options",
            SelectedOption::Exit => "Exit",
        }
    }
}

#[cfg(test)]
mod test {
    use crate::logic::menu::*;

    #[test]
    fn all() {
        let mut option = SelectedOption::default();
        assert_eq!(option, SelectedOption::NewGame);
        option.next_selection();
        assert_eq!(option, SelectedOption::Options);
        option.next_selection();
        assert_eq!(option, SelectedOption::Exit);
        option.next_selection();
        assert_eq!(option, SelectedOption::NewGame);
        option.previous_selection();
        assert_eq!(option, SelectedOption::Exit);
        option.previous_selection();
        assert_eq!(option, SelectedOption::Options);
        option.previous_selection();
        assert_eq!(option, SelectedOption::NewGame);
        option.previous_selection();
        assert_eq!(option, SelectedOption::Exit);
        assert_eq!(
            SelectedOption::all_possibilities(),
            vec![
                SelectedOption::NewGame,
                SelectedOption::Options,
                SelectedOption::Exit
            ]
        );
    }
}
