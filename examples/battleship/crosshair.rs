use crate::{
    battlestate::{BattleState, Position},
    config::Config,
    game::Game,
};
use turtle::Turtle;

pub enum CrosshairType {
    LockTarget,
    Disabled,
}

pub struct Crosshair<'a> {
    pos: (u8, u8),
    state: &'a BattleState,
    turtle: &'a mut Turtle,
}

impl<'a> Crosshair<'a> {
    pub fn new(state: &'a BattleState, turtle: &'a mut Turtle, last_attacked_pos: Option<(u8, u8)>) -> Self {
        let pos;
        // Draw crosshair in disabled mode on last attacked position
        if let Some(attacked_pos) = last_attacked_pos {
            pos = attacked_pos;
            Self::draw_crosshair(pos, turtle, CrosshairType::Disabled);
        // There's no last attacked position -> player's first chance
        // Draw crosshair in abled mode at the center.
        } else {
            pos = (4, 4);
            Self::draw_crosshair(pos, turtle, CrosshairType::LockTarget);
        }
        Self { pos, state, turtle }
    }

    fn draw_crosshair(pos: (u8, u8), turtle: &mut Turtle, crosshair: CrosshairType) {
        let (x, y) = Config::ATTACK_GRID_TOP_LEFT;
        turtle.set_pen_color(if matches!(crosshair, CrosshairType::Disabled) {
            Config::DISABLED_CROSSHAIR_COLOR
        } else {
            Config::CROSSHAIR_COLOR
        });
        turtle.set_pen_size(Config::CROSSHAIR_PEN_SIZE);
        let start = (
            x + Config::CELL_SIZE * (0.5 + pos.1 as f64),
            y - Config::CELL_SIZE * (0.5 + pos.0 as f64),
        );
        turtle.pen_up();
        turtle.go_to(start);
        turtle.pen_down();
        turtle.set_heading(0.0);
        for _ in 0..4 {
            turtle.forward(Config::CROSSHAIR_SIZE);
            turtle.pen_up();
            turtle.backward(Config::CROSSHAIR_SIZE);
            turtle.pen_down();
            turtle.right(90.0);
        }
        turtle.set_pen_color("black");
        turtle.set_pen_size(1.0);
    }

    fn move_to(&mut self, pos: (u8, u8), game: &Game) {
        // Remove crosshair on previous pos by redrawing the Cell at that pos.
        let cell = self.state.attack_grid().get(&self.pos);
        game.draw_cell(cell, Position::AttackGrid(self.pos), self.turtle);

        // Set Crosshair in disabled or abled mode based on new pos
        let crosshair = match self.state.can_bomb(&pos) {
            true => CrosshairType::LockTarget,
            false => CrosshairType::Disabled,
        };

        // Draw Crosshair at new pos
        Self::draw_crosshair(pos, self.turtle, crosshair);
        self.pos = pos;
    }

    pub fn move_left(&mut self, game: &Game) {
        if self.pos.1 > 0 {
            self.move_to((self.pos.0, self.pos.1 - 1), game);
        }
    }

    pub fn move_right(&mut self, game: &Game) {
        if self.pos.1 < 9 {
            self.move_to((self.pos.0, self.pos.1 + 1), game);
        }
    }

    pub fn move_up(&mut self, game: &Game) {
        if self.pos.0 > 0 {
            self.move_to((self.pos.0 - 1, self.pos.1), game);
        }
    }

    pub fn move_down(&mut self, game: &Game) {
        if self.pos.0 < 9 {
            self.move_to((self.pos.0 + 1, self.pos.1), game);
        }
    }

    // Retuns Some(pos) if the crosshair pos is attackable
    // None otherwise
    pub fn try_bomb(&mut self) -> Option<(u8, u8)> {
        if self.state.can_bomb(&self.pos) {
            return Some(self.pos);
        }
        None
    }
}
