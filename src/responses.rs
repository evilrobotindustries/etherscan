use serde::{
    de,
    de::{MapAccess, Visitor},
    Deserialize, Deserializer,
};
use std::fmt;
use std::marker::PhantomData;

#[derive(Debug)]
pub struct Response<T> {
    pub result: T,
}

#[derive(Debug, Deserialize)]
pub enum ResponseStatus {
    Success = 1,
    Failed = 0,
}

impl<'de, T: Deserialize<'de>> Deserialize<'de> for Response<T> {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> std::result::Result<Self, D::Error> {
        struct ResultVisitor<T>(PhantomData<fn() -> T>);

        const STATUS: &str = "status";
        const MESSAGE: &str = "message";
        const RESULT: &str = "result";

        impl<'de, T: Deserialize<'de>> Visitor<'de> for ResultVisitor<T> {
            type Value = Response<T>;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct Response<T>")
            }

            fn visit_map<V: MapAccess<'de>>(self, mut map: V) -> std::result::Result<Response<T>, V::Error> {
                let mut status = None;
                let mut message: Option<String> = None;
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
                        _ => {
                            // Ignore value
                            let _ = map.next_value::<()>();
                        }
                    }
                }
                status.ok_or_else(|| de::Error::missing_field(STATUS))?;
                message.ok_or_else(|| de::Error::missing_field(MESSAGE))?;
                let result = result.ok_or_else(|| de::Error::missing_field(RESULT))?;
                Ok(Response::<T> { result })
            }
        }

        const FIELDS: &[&str] = &[STATUS, MESSAGE, RESULT];
        deserializer.deserialize_struct("Response", FIELDS, ResultVisitor(PhantomData))
    }
}
