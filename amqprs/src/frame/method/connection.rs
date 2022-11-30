use std::fmt;

use crate::frame::REPLY_SUCCESS;
use amqp_serde::types::{AmqpPeerProperties, LongStr, LongUint, Octect, ShortStr, ShortUint};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Start {
    pub(crate) version_major: Octect,
    pub(crate) version_minor: Octect,
    pub(crate) server_properties: AmqpPeerProperties,
    pub(crate) mechanisms: LongStr,
    pub(crate) locales: LongStr,
}

impl Start {
    pub fn version_major(&self) -> u8 {
        self.version_major
    }

    pub fn version_minor(&self) -> u8 {
        self.version_minor
    }

    pub fn server_properties(&self) -> &AmqpPeerProperties {
        &self.server_properties
    }

    pub fn mechanisms(&self) -> &String {
        &self.mechanisms
    }

    pub fn locales(&self) -> &String {
        &self.locales
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StartOk {
    client_properties: AmqpPeerProperties,
    machanisms: ShortStr,
    response: LongStr,
    locale: ShortStr,
}

impl StartOk {
    pub fn new(
        client_properties: AmqpPeerProperties,
        machanisms: ShortStr,
        response: LongStr,
        locale: ShortStr,
    ) -> Self {
        Self {
            client_properties,
            machanisms,
            response,
            locale,
        }
    }

    pub fn client_properties_mut(&mut self) -> &mut AmqpPeerProperties {
        &mut self.client_properties
    }

    pub fn machanisms_mut(&mut self) -> &mut ShortStr {
        &mut self.machanisms
    }

    pub fn response_mut(&mut self) -> &mut LongStr {
        &mut self.response
    }

    pub fn locale_mut(&mut self) -> &mut ShortStr {
        &mut self.locale
    }
}

impl Default for StartOk {
    fn default() -> Self {
        Self {
            client_properties: AmqpPeerProperties::new(),
            machanisms: "PLAIN".try_into().unwrap(),
            response: "\0user\0bitnami".try_into().unwrap(),
            locale: "en_US".try_into().unwrap(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Tune {
    channel_max: ShortUint,
    frame_max: LongUint,
    heartbeat: ShortUint,
}

impl Tune {
    pub fn channel_max(&self) -> u16 {
        self.channel_max
    }

    pub fn frame_max(&self) -> u32 {
        self.frame_max
    }

    pub fn heartbeat(&self) -> u16 {
        self.heartbeat
    }
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct TuneOk {
    // RabbitMQ doesn't put a limit on channel-max, and treats any number in tune-ok as valid.
    // It does put a limit on frame-max, and checks that the value sent in tune-ok
    // is less than or equal.
    channel_max: ShortUint,
    frame_max: LongUint,
    heartbeat: ShortUint,
}

impl TuneOk {
    pub fn new(channel_max: ShortUint, frame_max: LongUint, heartbeat: ShortUint) -> Self {
        Self {
            channel_max,
            frame_max,
            heartbeat,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Open {
    virtual_host: ShortStr,
    /// Deprecated: "capabilities", must be zero
    capabilities: ShortStr,
    /// Deprecated: "insist", must be zero
    insist: Octect,
}

impl Open {
    pub fn new(virtual_host: ShortStr) -> Self {
        Self {
            virtual_host,
            capabilities: ShortStr::default(),
            insist: 0,
        }
    }
}

impl Default for Open {
    fn default() -> Self {
        Self {
            virtual_host: "/".try_into().unwrap(),
            capabilities: ShortStr::default(),
            insist: 0,
        }
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct OpenOk {
    ///  Deprecated: "known-hosts", must be zero
    know_hosts: ShortStr,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Close {
    reply_code: ShortUint,
    reply_text: ShortStr,
    class_id: ShortUint,
    method_id: ShortUint,
}
impl fmt::Display for Close {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!(
            "Close connection due to '{}: {}', (class_id = {}, method_id = {})",
            self.reply_code(),
            self.reply_text(),
            self.class_id(),
            self.method_id()
        ))
    }
}
impl Close {
    pub fn new(
        reply_code: ShortUint,
        reply_text: ShortStr,
        class_id: ShortUint,
        method_id: ShortUint,
    ) -> Self {
        Self {
            reply_code,
            reply_text,
            class_id,
            method_id,
        }
    }

    pub fn reply_code(&self) -> u16 {
        self.reply_code
    }

    pub fn reply_text(&self) -> &String {
        &self.reply_text
    }

    pub fn class_id(&self) -> u16 {
        self.class_id
    }

    pub fn method_id(&self) -> u16 {
        self.method_id
    }
}
impl Default for Close {
    fn default() -> Self {
        Self {
            reply_code: REPLY_SUCCESS,
            reply_text: ShortStr::default(),
            class_id: 0,
            method_id: 0,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct CloseOk;

#[derive(Debug, Serialize, Deserialize)]
pub struct Secure {
    challenge: LongStr,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct SecureOk {
    response: LongStr,
}

impl SecureOk {
    pub fn new(response: LongStr) -> Self {
        Self { response }
    }
}

// below from https://www.rabbitmq.com/resources/specs/amqp0-9-1.extended.xml
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Blocked {
    reason: ShortStr,
}

impl Blocked {
    pub fn new(reason: ShortStr) -> Self {
        Self { reason }
    }

    pub fn reason(&self) -> &String {
        &self.reason
    }
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Unblocked;

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateSecret {
    new_secret: LongStr,
    reason: ShortStr,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct UpdateSecretOk;
