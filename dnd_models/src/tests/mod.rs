use crate::player::Player;

#[test]
fn test_level_up() {
    let mut player = Player::new().level(0);
    player.level_up();
    assert_eq!(&player.level, &Some(1));
}
