use lepticons_animate::Easing;

#[test]
fn easing_as_css_values() {
    assert_eq!(Easing::Linear.as_css(), "linear");
    assert_eq!(Easing::Ease.as_css(), "ease");
    assert_eq!(Easing::EaseIn.as_css(), "ease-in");
    assert_eq!(Easing::EaseOut.as_css(), "ease-out");
    assert_eq!(Easing::EaseInOut.as_css(), "ease-in-out");
}

#[test]
fn easing_display_matches_as_css() {
    let variants = [
        Easing::Linear,
        Easing::Ease,
        Easing::EaseIn,
        Easing::EaseOut,
        Easing::EaseInOut,
    ];
    for v in variants {
        assert_eq!(format!("{v}"), v.as_css());
    }
}

#[test]
fn easing_default_is_ease_in_out() {
    assert_eq!(Easing::default(), Easing::EaseInOut);
}

#[test]
fn easing_css_tokens_contain_no_spaces() {
    let variants = [
        Easing::Linear,
        Easing::Ease,
        Easing::EaseIn,
        Easing::EaseOut,
        Easing::EaseInOut,
    ];
    for v in variants {
        assert!(
            !v.as_css().contains(' '),
            "CSS value has space: {}",
            v.as_css()
        );
    }
}

#[test]
fn easing_is_copy() {
    let a = Easing::EaseOut;
    let b = a;
    assert_eq!(a, b);
}
