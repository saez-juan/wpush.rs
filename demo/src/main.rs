use wpush::Notification;

fn main() {
    let mut notification = Notification::new();
    notification.set_app_id("example.app");
    notification.set_title(Some("WPush"));
    notification.set_message(Some("This push notification comes from WPush!"));

    notification.push();
}
