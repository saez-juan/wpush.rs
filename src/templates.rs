use crate::notification::{Action, Notification};
use serde::Serialize;
use tinytemplate::{format_unescaped, TinyTemplate};

#[derive(Serialize, Debug)]
pub struct XMLContext {
    activation_type: String,
    activation_arguments: String,
    duration: String,
    icon: String,
    title: String,
    message: String,
    loopeable: bool,
    audio: String,
    actions: Vec<Action>,
}

impl XMLContext {
    pub fn from_notification(n: &Notification) -> Self {
        Self {
            activation_type: n.activation_type.clone(),
            activation_arguments: n.activation_arguments.clone(),
            duration: n.duration.as_str().into(),
            icon: n.icon.clone().unwrap_or(String::from("")),
            title: n.title.clone().unwrap_or(String::from("")),
            message: n.message.clone().unwrap_or(String::from("")),
            audio: match &n.audio {
                Some(a) => a.as_str().into(),
                None => String::from(""),
            },
            actions: match &n.actions {
                Some(v) => v.clone(),
                None => Vec::new(),
            },
            loopeable: n.loopeable,
        }
    }
}

pub struct Template;
impl Template {
    fn get_tt() -> TinyTemplate<'static> {
        let mut tt = TinyTemplate::new();
        tt.set_default_formatter(&format_unescaped);
        return tt;
    }

    pub fn get_notification_content(n: &Notification) -> String {
        let template = include_str!("../assets/notification.xml");
        let context = XMLContext::from_notification(&n);

        let mut tt = Template::get_tt();
        if let Err(e) = tt.add_template("notification", template) {
            dbg!(e);
            return String::new();
        }

        let render = tt.render("notification", &context);
        if let Err(e) = render {
            dbg!(e);
            return String::new();
        }
        return render.unwrap();
    }

    pub fn get_script_content(app_id: &str, xml: String) -> String {
        let template = include_str!("../assets/script.ps1");
        let context = ScriptContext::new(app_id, xml);

        let mut tt = Template::get_tt();
        if let Err(e) = tt.add_template("notification", template) {
            dbg!(e);
            return String::new();
        }

        let render = tt.render("notification", &context);
        if let Err(e) = render {
            dbg!(e);
            return String::new();
        }

        return render.unwrap();
    }
}

#[derive(Serialize, Debug)]
pub struct ScriptContext {
    app_id: String,
    xml_notification: String,
}

impl ScriptContext {
    pub fn new(app_id: &str, xml_notification: String) -> Self {
        Self {
            app_id: app_id.into(),
            xml_notification,
        }
    }
}
