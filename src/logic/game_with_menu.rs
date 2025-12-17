use iced::{Pixels, widget::canvas::Text};

use crate::logic::{
    game::SnakeGame,
    menu::{Menu, SelectedOption},
    traits::DrawableOn,
};

pub struct GameWithMenu {
    game: SnakeGame,
    menu: Menu,
    game_or_menu: GameOrMenu,
}

impl GameWithMenu {
    pub fn new(width: usize, height: usize) -> Self {
        GameWithMenu {
            game: SnakeGame::new(width, height),
            menu: Menu::new(),
            game_or_menu: GameOrMenu::InMenu,
        }
    }

    pub fn update(&mut self, now: std::time::Instant) {
        match self.game_or_menu {
            GameOrMenu::InGame => {
                self.game.update(now);
            }
            GameOrMenu::InMenu => {
                self.menu.update(now);
            }
        }
    }

    pub fn up_pressed(&mut self) {
        match self.game_or_menu {
            GameOrMenu::InGame => return self.game.change_direction(super::Direction::Up),
            GameOrMenu::InMenu => return self.menu.select_previous_option(),
        }
    }

    pub fn left_pressed(&mut self) {
        match self.game_or_menu {
            GameOrMenu::InGame => self.game.change_direction(super::Direction::Left),
            GameOrMenu::InMenu => (),
        }
    }
    pub fn down_pressed(&mut self) {
        match self.game_or_menu {
            GameOrMenu::InGame => self.game.change_direction(super::Direction::Down),
            GameOrMenu::InMenu => self.menu.select_next_option(),
        }
    }

    pub fn right_pressed(&mut self) {
        match self.game_or_menu {
            GameOrMenu::InGame => self.game.change_direction(super::Direction::Right),
            GameOrMenu::InMenu => (),
        }
    }
    pub fn enter_or_space_pressed(&mut self) {
        match self.game_or_menu {
            GameOrMenu::InGame => {
                if self.game.is_over() {
                    *self = GameWithMenu::new(self.game.width(), self.game.height())
                }
                self.game.set_paused(!self.game.is_paused())
            }
            GameOrMenu::InMenu => match self.menu.selected_option() {
                SelectedOption::NewGame => {
                    self.game_or_menu = GameOrMenu::InGame;
                    self.game = SnakeGame::new(25, 25);
                }

                SelectedOption::Options => return,
                SelectedOption::Exit => {
                    std::process::exit(0);
                }
            },
        }
    }

    pub fn draw<T: DrawableOn>(&self, frame: &mut T) {
        let game_width = self.game.width();
        let game_height = self.game.height();

        let selected_color = (255, 255, 0);
        let unselected_color = (255, 255, 255);

        let text_size: u8 = 50;
        let text_gap: u8 = text_size + 15;

        match self.game_or_menu {
            GameOrMenu::InGame => {
                for (snake_x, snake_y) in self.game.snake() {
                    draw_snake_square(
                        frame,
                        (0, 255, 0),
                        (*snake_x, *snake_y),
                        (game_width, game_height),
                    );
                }

                if self.game.is_paused() {
                    frame.draw_text(
                        "Paused",
                        (255, 255, 255),
                        frame.width() / 2,
                        frame.height() / 2,
                        25.,
                    );
                }
                draw_snake_square(
                    frame,
                    (255, 0, 0),
                    self.game.food(),
                    (game_width, game_height),
                );

                frame.draw_text(
                    &format!("Your score: {:?}", self.game.score()),
                    (255, 255, 255),
                    500,
                    700,
                    25f32,
                );

                if self.game.is_over() {
                    frame.draw_text(
                        &format!(
                            "Game Over. Press space to start a new game. Your score: {:?}",
                            self.game.score()
                        ),
                        (255, 0, 0),
                        frame.width() / 2,
                        frame.height() / 2,
                        25f32,
                    );
                }
            }
            GameOrMenu::InMenu => {
                for currrent_selected_option in SelectedOption::all_possibilities() {
                    let color_rgb = if self.menu.selected_option() == currrent_selected_option {
                        selected_color
                    } else {
                        unselected_color
                    };

                    frame.draw_text(
                        currrent_selected_option.menu_text(),
                        color_rgb,
                        frame.width() / 2,
                        frame.height() / 2 + ((currrent_selected_option as u8) * text_gap) as usize,
                        text_size as f32,
                    );
                }
            }
        }
    }
}

/// This function does a transformation from the logic to the graphics and draws the square.
/// Can draw a square in any color or size
pub fn draw_snake_square<T: DrawableOn>(
    frame: &mut T,
    color: (u8, u8, u8),
    (square_x, square_y): (usize, usize),
    (game_square_width, game_square_height): (usize, usize),
) {
    let h_s = frame.height() as usize / game_square_height;
    let w_s = frame.width() as usize / game_square_width;
    let ix = square_x * w_s;
    let iy = square_y * h_s;
    let top_left = (ix as f32, iy as f32);

    frame.fill_rectangle(
        (w_s, h_s),
        (color.0 as u8, color.1 as u8, color.2 as u8),
        (top_left.0 as usize, top_left.1 as usize),
    );
}

#[derive(Default, Debug, PartialEq)]
enum GameOrMenu {
    #[default]
    InGame,
    // TODO: Make InMenu the default
    InMenu,
}

#[cfg(test)]
mod tests {
    use crate::logic::{
        Direction,
        game_with_menu::{GameOrMenu, GameWithMenu},
        menu::SelectedOption,
    };

    #[test]
    fn up_test() {
        let mut game_with_menu = GameWithMenu::new(25, 25);
        assert_eq!(game_with_menu.game_or_menu, GameOrMenu::InMenu);
        game_with_menu.up_pressed();
        assert_eq!(
            game_with_menu.menu.selected_option(),
            SelectedOption::Options
        );

        game_with_menu.game_or_menu = GameOrMenu::InGame;
        game_with_menu.up_pressed();
        assert_eq!(game_with_menu.game.direction(), Direction::Up);
    }

    #[test]
    fn right_test() {
        let mut game_with_menu = GameWithMenu::new(25, 25);
        assert_eq!(game_with_menu.game_or_menu, GameOrMenu::InMenu);
        game_with_menu.right_pressed();
        assert_eq!(
            game_with_menu.menu.selected_option(),
            SelectedOption::NewGame
        );

        game_with_menu.game_or_menu = GameOrMenu::InGame;
        game_with_menu.right_pressed();
        assert_eq!(game_with_menu.game.direction(), Direction::Right);
    }
    #[test]
    fn left_test() {
        let mut game_with_menu = GameWithMenu::new(25, 25);
        assert_eq!(game_with_menu.game_or_menu, GameOrMenu::InMenu);
        game_with_menu.left_pressed();
        assert_eq!(
            game_with_menu.menu.selected_option(),
            SelectedOption::NewGame
        );

        game_with_menu.game_or_menu = GameOrMenu::InGame;
        game_with_menu.left_pressed();
        assert_eq!(game_with_menu.game.direction(), Direction::Left);
    }
    #[test]
    fn down_test() {
        let mut game_with_menu = GameWithMenu::new(25, 25);
        assert_eq!(game_with_menu.game_or_menu, GameOrMenu::InMenu);
        game_with_menu.down_pressed();

        assert_eq!(
            game_with_menu.menu.selected_option(),
            SelectedOption::Options
        );

        game_with_menu.game_or_menu = GameOrMenu::InGame;
        game_with_menu.down_pressed();
        assert_eq!(game_with_menu.game.direction(), Direction::Down);
    }
    #[test]
    fn enter_or_space_test() {
        let mut game_with_menu = GameWithMenu::new(25, 25);
        assert_eq!(game_with_menu.game_or_menu, GameOrMenu::InMenu);
        game_with_menu.enter_or_space_pressed();
        assert_eq!(
            game_with_menu.menu.selected_option(),
            SelectedOption::NewGame
        );
        assert_eq!(game_with_menu.game.is_paused(), false);

        game_with_menu.game_or_menu = GameOrMenu::InGame;
        game_with_menu.enter_or_space_pressed();

        assert_eq!(game_with_menu.game.is_paused(), true);
    }
}
