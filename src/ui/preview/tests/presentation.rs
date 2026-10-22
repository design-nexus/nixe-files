// SPDX-License-Identifier: MIT
use super::*;
use crate::ui::preferences::PreviewStyle;

#[test]
fn preview_presentation_popup_is_singleton_and_style_changes_cancel_old_loads() {
    crate::test_support::gtk_test(
        "ui::preview::tests::presentation::preview_presentation_popup_is_singleton_and_style_changes_cancel_old_loads",
        || {
            let provider = Rc::new(Provider::default());
            let first = PreviewDrawer::new(provider.clone(), false);
            let second = PreviewDrawer::new(provider, false);
            first.state.change_presentation(PreviewStyle::Popup);
            second.state.change_presentation(PreviewStyle::Popup);
            first.show(entry("/tmp/first.txt"), None);
            let window = first.state.popup.borrow().clone().expect("popup opens");
            assert!(window.is_visible());
            second.show(entry("/tmp/second.txt"), None);
            assert!(!first.is_enabled());
            assert_eq!(second.state.popup.borrow().as_ref(), Some(&window));
            let request = second.state.current_request.get();
            second.state.change_presentation(PreviewStyle::Panel);
            assert!(!window.is_visible());
            assert!(second.is_enabled());
            assert!(!second.state.floating.get());
            assert_ne!(second.state.current_request.get(), request);
            second.close();
            assert!(!second.is_enabled());
            assert!(second.state.load.borrow().is_none());
        },
    );
}
