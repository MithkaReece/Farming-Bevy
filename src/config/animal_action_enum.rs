use super::Status;

#[derive(Clone, Debug, PartialEq)]
pub enum AnimalAction {
    DrinkWater,
    GoToWater,
    LookForWater,

    EatFood,
    GoToFood,
    LookForFood,

    Breed,
    MoveToHerd,
    Wander,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum AnimalCondition {
    Thirsty,
    Hungry,
    InHerd
}
