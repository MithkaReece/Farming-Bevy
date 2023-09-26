use super::Status;

#[derive(Clone, Debug, PartialEq)]
pub enum AnimalAction {
    Thirsty,
    DrinkWater,
    GoToWater,
    LookForWater,

    Hungry,
    EatFood,
    GoToFood,
    LookForFood,

    InHerd,
    Breed,
    MoveToHerd,
    Wander,
}
