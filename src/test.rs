use crate::history::History;

#[test]
fn test_history() {
    let mut history = History::new("foo.com".to_owned());
    assert_eq!(history.can_backward(), false);
    assert_eq!(history.can_forward(), false);
    assert_eq!(history.current(), "foo.com");
    // foo.com
    //    ^

    history.goto("bar.com".to_owned());
    assert_eq!(history.can_backward(), true);
    assert_eq!(history.can_forward(), false);
    assert_eq!(history.current(), "bar.com");
    // foo.com -> bar.com
    //               ^
    
    assert_eq!(history.backward(), Ok(()));
    assert_eq!(history.can_backward(), false);
    assert_eq!(history.can_forward(), true);
    assert_eq!(history.current(), "foo.com");
    // foo.com -> bar.com
    //    ^

    assert_eq!(history.forward(), Ok(()));
    assert_eq!(history.can_backward(), true);
    assert_eq!(history.can_forward(), false);
    assert_eq!(history.current(), "bar.com");
    // foo.com -> bar.com
    //               ^

    history.goto("baz.com".to_owned());
    assert_eq!(history.can_backward(), true);
    assert_eq!(history.can_forward(), false);
    assert_eq!(history.current(), "baz.com");
    // foo.com -> bar.com -> baz.com
    //                          ^

    assert_eq!(history.backward(), Ok(()));
    assert_eq!(history.can_backward(), true);
    assert_eq!(history.can_forward(), true);
    assert_eq!(history.current(), "bar.com");
    // foo.com -> bar.com -> baz.com
    //               ^

    history.goto("github.com".to_owned());
    assert_eq!(history.can_backward(), true);
    assert_eq!(history.can_forward(), false);
    assert_eq!(history.current(), "github.com");
    // foo.com -> bar.com -> github.com
    //                           ^

    assert_eq!(history.forward(), Err(()));
    assert_eq!(history.backward(), Ok(()));
    assert_eq!(history.backward(), Ok(()));
    assert_eq!(history.backward(), Err(()));
}
