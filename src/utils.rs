pub struct t {
    input1_state: Entity<InputState>,
    input2_state: Entity<InputState>,
    display_text: SharedString,
    _subscriptions1: Vec<Subscription>,
    _subscriptions2: Vec<Subscription>,
}
