

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Input {
	TurnLeft,
	TurnRight,
	MoveLeft,
	MoveRight,
	Forward,
	Back,
	Nothing,
	Quit
}
