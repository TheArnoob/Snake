use strum::{EnumCount, EnumIter, FromRepr, IntoEnumIterator};

#[derive(Debug, Default, Clone, Copy)]
pub enum MenuType {
    #[default]
    MainMenu,
    SettingsMenu,
}
#[derive(Debug, Default)]
pub struct Menu {
    selected_option: SelectedOption,
    setting: Settings,
    menu_type: MenuType,
}

pub enum MenuAction {
    NoOp,
    NewGame,
}

impl Menu {
    pub fn selected_option(&self) -> u8 {
        match self.menu_type {
            MenuType::MainMenu => self.selected_option as u8,
            MenuType::SettingsMenu => self.setting.selected_setting as u8,
        }
    }

    pub fn next_difficulty(&mut self) {
        self.setting.next_difficulty();
    }
    pub fn previous_difficulty(&mut self) {
        self.setting.previous_difficulty();
    }

    #[cfg(test)]
    pub fn new() -> Menu {
        Menu {
            selected_option: SelectedOption::default(),
            setting: Settings::default(),
            menu_type: MenuType::default(),
        }
    }

    pub fn menu_type(&self) -> MenuType {
        self.menu_type
    }

    pub fn settings(&self) -> Settings {
        self.setting
    }

    pub fn all_possibilities(&self) -> Vec<String> {
        let all_selectable_options: Vec<SelectedOption> = SelectedOption::iter().collect();
        let all_settings: Vec<String> = Settings::all_possibilities(&self.setting);

        let all_options = all_selectable_options
            .iter()
            .map(|t| t.menu_text().to_string())
            .collect::<Vec<_>>();

        match self.menu_type {
            MenuType::MainMenu => all_options,
            MenuType::SettingsMenu => all_settings,
        }
    }

    pub fn update(&mut self, _now: web_time::Instant) {}

    pub fn enter_or_space_pressed(&mut self) -> MenuAction {
        match self.menu_type() {
            MenuType::MainMenu => match self.selected_option {
                SelectedOption::Settings => {
                    self.set_menu_type(MenuType::SettingsMenu);
                    MenuAction::NoOp
                }
                SelectedOption::NewGame => MenuAction::NewGame,
            },
            MenuType::SettingsMenu => {
                match self.selected_setting() {
                    SelectedSetting::Difficulty => {
                        self.setting.difficulty.next_difficulty();
                    }
                    SelectedSetting::Back => {
                        self.set_menu_type(MenuType::MainMenu);
                    }
                }
                MenuAction::NoOp
            }
        }
    }

    pub fn select_next_option(&mut self) {
        match self.menu_type {
            MenuType::MainMenu => self.selected_option.next_selection(),
            MenuType::SettingsMenu => self.setting.selected_setting.next_selection(),
        }
    }

    pub fn select_previous_option(&mut self) {
        match self.menu_type {
            MenuType::MainMenu => self.selected_option.previous_selection(),
            MenuType::SettingsMenu => self.setting.selected_setting.previous_selection(),
        }
    }

    pub fn selected_setting(&self) -> SelectedSetting {
        self.setting.selected_setting
    }
    pub fn set_menu_type(&mut self, menu_type: MenuType) {
        self.menu_type = menu_type
    }
}
#[derive(Debug, Default, EnumIter, PartialEq, FromRepr, Clone, Copy, EnumCount)]
#[repr(u8)]
pub enum SelectedOption {
    #[default]
    NewGame,
    Settings,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Settings {
    selected_setting: SelectedSetting,
    difficulty: Difficulty,
}

impl Settings {
    pub fn difficulty(&self) -> Difficulty {
        self.difficulty
    }

    pub fn next_difficulty(&mut self) {
        self.difficulty.next_difficulty();
    }

    pub fn previous_difficulty(&mut self) {
        self.difficulty.previous_difficulty();
    }

    pub fn all_possibilities(&self) -> Vec<String> {
        SelectedSetting::all_possibilities()
            .iter()
            .map(|t| match t {
                SelectedSetting::Difficulty => format!("Difficulty: {:?}", self.difficulty),
                SelectedSetting::Back => "Back".to_string(),
            })
            .collect()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Default, FromRepr, EnumCount, EnumIter)]
#[repr(u8)]
pub enum SelectedSetting {
    #[default]
    Difficulty,
    Back,
}

impl SelectedSetting {
    pub fn all_possibilities() -> Vec<SelectedSetting> {
        SelectedSetting::iter().collect()
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
}

#[derive(Debug, Clone, Copy, PartialEq, FromRepr, EnumCount, EnumIter, Default)]
pub enum Difficulty {
    VeryEasy,
    Easy,
    #[default]
    Normal,
    Intermediate,
    Hard,
    Expert,
    Extreme,
    Insane,
    Basic,
    VeryHard,
}

impl Difficulty {
    pub fn next_difficulty(&mut self) {
        if Self::COUNT - 1 == *self as usize {
            return *self = Self::from_repr(0).expect("Cannot Fail");
        }
        *self = Self::from_repr(((*self as u8) + 1) as usize).expect("Cannot Fail")
    }

    pub fn previous_difficulty(&mut self) {
        if 0 == *self as usize {
            return *self = Self::from_repr(Self::COUNT - 1).expect("Cannot Fail");
        }
        *self = Self::from_repr(((*self as u8) - 1) as usize).expect("Cannot Fail")
    }
}
impl SelectedOption {
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
            SelectedOption::Settings => "Settings",
        }
    }
}

#[cfg(test)]
mod test {
    use crate::menu::SelectedOption;

    #[test]
    fn all() {
        let mut option = SelectedOption::default();
        assert_eq!(option, SelectedOption::NewGame);
        option.next_selection();
        assert_eq!(option, SelectedOption::Settings);
        option.next_selection();
        assert_eq!(option, SelectedOption::NewGame);
        option.next_selection();
        assert_eq!(option, SelectedOption::Settings);
        option.previous_selection();
        assert_eq!(option, SelectedOption::NewGame);
        option.previous_selection();
        assert_eq!(option, SelectedOption::Settings);
        option.previous_selection();
        assert_eq!(option, SelectedOption::NewGame);
        option.previous_selection();
        assert_eq!(option, SelectedOption::Settings);
    }
}
