use serde::{
    de,
    de::{MapAccess, Visitor},
    Deserialize, Deserializer,
};
use std::fmt;
use std::marker::PhantomData;

#[derive(Debug)]
pub struct Response<T> {
    pub status: ResponseStatus,
    pub message: String,
    pub result: T,
}

impl<T> Response<T> {
    pub fn new(status: ResponseStatus, message: String, result: T) -> Response<T> {
        Response { status, message, result }
    }
}

#[derive(Debug, Deserialize)]
pub enum ResponseStatus {
    Success = 1,
    Failed = 0,
}

impl<'de, T: Deserialize<'de>> Deserialize<'de> for Response<T> {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> std::result::Result<Self, D::Error> {
        struct ResultVisitor<T>(PhantomData<fn() -> T>);

        impl<'de, T: Deserialize<'de>> Visitor<'de> for ResultVisitor<T> {
            type Value = Response<T>;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct Response<T>")
            }

            fn visit_map<V: MapAccess<'de>>(self, mut map: V) -> std::result::Result<Response<T>, V::Error> {
                const STATUS: &str = "status";
                const MESSAGE: &str = "message";
                const RESULT: &str = "result";

                let mut status = None;
                let mut message = None;
                let mut result = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        STATUS => {
                            if status.is_some() {
                                return Err(de::Error::duplicate_field(STATUS));
                            }
                            let value: String = map.next_value()?;
                            status = Some(match value.as_str() {
                                "1" => ResponseStatus::Success,
                                "0" => ResponseStatus::Failed,
                                _ => panic!("Unknown value: {}", value),
                            });
                        }
                        MESSAGE => {
                            if message.is_some() {
                                return Err(de::Error::duplicate_field(MESSAGE));
                            }
                            message = Some(map.next_value()?);

                            if let Some(ResponseStatus::Failed) = status {
                                if let Some(message) = { &message } {
                                    if message == "NOTOK" {
                                        // Get message from next value
                                        let next: Option<(String, String)> = map.next_entry()?;
                                        if let Some((_, value)) = next {
                                            return Err(de::Error::custom(value));
                                        }
                                    }
                                }
                            }
                        }
                        RESULT => {
                            if result.is_some() {
                                return Err(de::Error::duplicate_field(RESULT));
                            }
                            if status.is_none() {
                                return Err(de::Error::custom("status not deserialised yet"));
                            }

                            // Return result message as error if failed
                            if let Some(ResponseStatus::Failed) = status {
                                if let Some(message) = { &message } {
                                    // Exclude empty result, which returns as status failed
                                    if message != "No transactions found" {
                                        let value: String = map.next_value()?;
                                        return Err(de::Error::custom(value));
                                    }
                                }
                            }

                            result = Some(map.next_value()?);
                        }
                        _ => {}
                    }
                }
                let status = status.ok_or_else(|| de::Error::missing_field("status"))?;
                let message = message.ok_or_else(|| de::Error::missing_field("message"))?;
                let result = result.ok_or_else(|| de::Error::missing_field("result"))?;
                Ok(Response::<T>::new(status, message, result))
            }
        }

        const FIELDS: &[&str] = &["status", "message", "result"];
        deserializer.deserialize_struct("Duration", FIELDS, ResultVisitor(PhantomData))
    }
}
