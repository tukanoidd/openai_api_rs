use std::fmt::{Display, Formatter};

use crate::model::Model;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error, miette::Diagnostic)]
pub enum Error {
    ReqwestError(Box<dyn std::error::Error>),
    ParseError(Box<dyn std::error::Error>),
    ModelError(Box<dyn std::error::Error>),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ReqwestError(e) | Self::ParseError(e) | Self::ModelError(e) => e.fmt(f),
        }
    }
}

#[derive(Debug, thiserror::Error, miette::Diagnostic)]
pub enum ParseError {
    FieldNotFound(String),
    FailedToParseFromValue,
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::FieldNotFound(field_name) => {
                write!(f, "\"{field_name}\" not found")
            }
            ParseError::FailedToParseFromValue => {
                write!(f, "Failed to parse from value")
            }
        }
    }
}

#[derive(Debug, thiserror::Error, miette::Diagnostic)]
pub enum ModelError {
    NotCompatibleWithCompletion,
}

impl Display for ModelError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotCompatibleWithCompletion => {
                write!(f, "Model is not compatible with completion endpoint, please use one of these models: {:?}", Model::COMPLETIONS_COMPATIBLE)
            }
        }
    }
}

macro_rules! from_err {
    ($($name:ident [$ty:path]),* $(,)*) => {
        $(
            impl From<$ty> for Error {
                fn from(e: $ty) -> Self {
                    Self::$name(Box::new(e))
                }
            }
        )*
    };
}

from_err!(
    ReqwestError[reqwest::Error],
    ParseError[ParseError],
    ModelError[ModelError],
);
