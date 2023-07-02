use crate::utils::Rect;

use super::{guimanager::Component, renderable::gui::Gui as Interface};

pub const GUI_IMG_WIDTH: f32 = 100.0;
pub const GUI_IMG_HEIGHT: f32 = 100.0;

pub fn test_gui(interface: &mut Interface, screen: &Rect<f32>) {
    // Crosshair 7 * 7
    interface.add(Component::new(
        0.5 - 7.0 / screen.right * 5.0,
        0.5 - 7.0 / screen.top * 5.0,
        7.0 / screen.right * 5.0,
        7.0 / screen.top * 5.0,
        Rect {
            left: 0.0,
            top: 0.0,
            right: 7.0 / GUI_IMG_WIDTH,
            bottom: 7.0 / GUI_IMG_HEIGHT,
        },
        0.0,
    ));

    // Healthbar 46 * 14 // 0, 10
    interface.add(Component::new(
        1.0 / screen.right * 6.0,
        1.0 - 1.0 / screen.top * 6.0 - 14.0 / screen.top * 6.0,
        46.0 / screen.right * 6.0,
        14.0 / screen.top * 6.0,
        Rect {
            left: 0.0,
            top: 10.0 / GUI_IMG_HEIGHT,
            right: 46.0 / GUI_IMG_WIDTH,
            bottom: 14.0 / GUI_IMG_HEIGHT,
        },
        0.1,
    ));

    // Health (n * 8) * 8 // n * 8, 24
    for n in 0..5 {
        interface.add(Component::new(
            n as f32 * 8.0 / screen.right * 6.0 + 4.0 / screen.right * 6.0,
            1.0 - 4.0 / screen.top * 6.0 - 8.0 / screen.top * 6.0,
            8.0 / screen.right * 6.0,
            8.0 / screen.top * 6.0,
            Rect {
                left: n as f32 * 8.0 / GUI_IMG_WIDTH,
                top: 24.0 / GUI_IMG_HEIGHT,
                right: 8.0 / GUI_IMG_WIDTH,
                bottom: 8.0 / GUI_IMG_HEIGHT,
            },
            0.2,
        ));
    }

    interface.init();
}
