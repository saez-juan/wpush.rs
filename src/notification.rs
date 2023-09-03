///! Notification module
use crate::{powershell::PowerShell, templates::Template};
use serde::Serialize;

/// Notification struct
pub struct Notification {
    pub app_id: String,
    pub title: Option<String>,
    pub message: Option<String>,
    pub icon: Option<String>,
    pub activation_type: String,
    pub activation_arguments: String,
    pub actions: Option<Vec<Action>>,
    pub audio: Option<Audio>,
    pub loopeable: bool,
    pub duration: Duration,
}

impl Notification {
    pub fn new() -> Self {
        Self {
            app_id: "Windows App".into(),
            title: None,
            message: None,
            icon: None,
            activation_type: "protocol".into(),
            activation_arguments: String::from(""),
            actions: None,
            audio: None,
            loopeable: false,
            duration: Duration::Short,
        }
    }

    pub fn set_app_id(&mut self, id: &str) {
        self.app_id = id.into();
    }

    pub fn set_title(&mut self, title: Option<&str>) {
        self.title = match title {
            Some(t) => Some(t.into()),
            None => None,
        };
    }

    pub fn set_message(&mut self, message: Option<&str>) {
        self.message = match message {
            Some(m) => Some(m.into()),
            None => None,
        };
    }

    pub fn set_icon(&mut self, path: &str) {
        self.icon = Some(path.into());
    }

    pub fn set_audio(&mut self, audio: Option<Audio>) {
        self.audio = audio;
    }

    pub fn push(&self) -> bool {
        let notification_content = Template::get_notification_content(&self);
        let script_content = Template::get_script_content(&self.app_id, notification_content);

        PowerShell::execute(script_content)
    }
}

#[derive(Serialize, Clone, Debug)]
pub struct Action {
    action_type: String,
    label: String,
    arguments: String,
}

pub enum Audio {
    Default,
    IM,
    Mail,
    Remainder,
    SMS,
    LoopingAlarm1,
    LoopingAlarm2,
    LoopingAlarm3,
    LoopingAlarm4,
    LoopingAlarm5,
    LoopingAlarm6,
    LoopingAlarm7,
    LoopingAlarm8,
    LoopingAlarm9,
    LoopingAlarm10,
    LoopingCall1,
    LoopingCall2,
    LoopingCall3,
    LoopingCall4,
    LoopingCall5,
    LoopingCall6,
    LoopingCall7,
    LoopingCall8,
    LoopingCall9,
    LoopingCall10,
}

impl Audio {
    pub fn as_str(&self) -> &'static str {
        match self {
            Audio::Default => "ms-winsoundevent:Notification.Default",
            Audio::IM => "ms-winsoundevent:Notification.IM",
            Audio::Mail => "ms-winsoundevent:Notification.Mail",
            Audio::Remainder => "ms-winsoundevent:Notification.Remainder",
            Audio::SMS => "ms-winsoundevent:Notification.SMS",
            Audio::LoopingAlarm1 => "ms-winsoundevent:Notification.Looping.Alarm",
            Audio::LoopingAlarm2 => "ms-winsoundevent:Notification.Looping.Alarm2",
            Audio::LoopingAlarm3 => "ms-winsoundevent:Notification.Looping.Alarm3",
            Audio::LoopingAlarm4 => "ms-winsoundevent:Notification.Looping.Alarm4",
            Audio::LoopingAlarm5 => "ms-winsoundevent:Notification.Looping.Alarm5",
            Audio::LoopingAlarm6 => "ms-winsoundevent:Notification.Looping.Alarm6",
            Audio::LoopingAlarm7 => "ms-winsoundevent:Notification.Looping.Alarm7",
            Audio::LoopingAlarm8 => "ms-winsoundevent:Notification.Looping.Alarm8",
            Audio::LoopingAlarm9 => "ms-winsoundevent:Notification.Looping.Alarm9",
            Audio::LoopingAlarm10 => "ms-winsoundevent:Notification.Looping.Alarm10",
            Audio::LoopingCall1 => "ms-winsoundevent:Notification.Looping.Call",
            Audio::LoopingCall2 => "ms-winsoundevent:Notification.Looping.Call2",
            Audio::LoopingCall3 => "ms-winsoundevent:Notification.Looping.Call3",
            Audio::LoopingCall4 => "ms-winsoundevent:Notification.Looping.Call4",
            Audio::LoopingCall5 => "ms-winsoundevent:Notification.Looping.Call5",
            Audio::LoopingCall6 => "ms-winsoundevent:Notification.Looping.Call6",
            Audio::LoopingCall7 => "ms-winsoundevent:Notification.Looping.Call7",
            Audio::LoopingCall8 => "ms-winsoundevent:Notification.Looping.Call8",
            Audio::LoopingCall9 => "ms-winsoundevent:Notification.Looping.Call9",
            Audio::LoopingCall10 => "ms-winsoundevent:Notification.Looping.Call10",
        }
    }
}

pub enum Duration {
    Short,
    Long,
}

impl Duration {
    pub fn as_str(&self) -> &'static str {
        match self {
            Duration::Long => "long",
            Duration::Short => "short",
        }
    }
}
