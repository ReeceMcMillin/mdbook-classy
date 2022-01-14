#![warn(clippy::pedantic)]
#![allow(clippy::missing_panics_doc)]
#![allow(clippy::missing_errors_doc)]

use std::fmt::Display;

#[allow(clippy::wildcard_imports)]
use nom::{
    bytes::complete::*, character::complete::*, combinator::*, multi::*, sequence::*, IResult,
};

#[derive(Debug, Clone)]
pub struct Attribute {
    pub key: String,
    pub value: String,
}

impl Display for Attribute {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, r#"{}="{}""#, &self.key, &self.value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Class(pub String);

impl Display for Class {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.0)
    }
}

#[derive(Debug, Clone, Default)]
pub struct AttrListDef {
    pub classes: Vec<Class>,
    pub id: Option<String>,
    pub attributes: Vec<Attribute>,
}

impl Display for AttrListDef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let id = match &self.id {
            Some(id) => format!(r#"id="{id}""#),
            None => String::new(),
        };

        let classes = self
            .classes
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<String>>()
            .join(" ");

        let attrs = self
            .attributes
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<String>>()
            .join(" ");

        write!(f, r#"<div class="{classes}" {id} {attrs}>"#)
    }
}

// TODO: allow more than just alphanumeric
pub fn string(input: &str) -> IResult<&str, &str> {
    delimited(char('"'), take_until("\""), char('"'))(input)
}

// TODO: allow more than just alphanumeric
pub fn class(input: &str) -> IResult<&str, &str> {
    delimited(multispace0, preceded(char('.'), alphanumeric1), multispace0)(input)
}

// TODO: allow more than just alphanumeric
pub fn id(input: &str) -> IResult<&str, &str> {
    delimited(multispace0, preceded(char('#'), alphanumeric1), multispace0)(input)
}

pub fn identifier(input: &str) -> IResult<&str, &str> {
    delimited(multispace0, alphanumeric1, multispace0)(input)
}

// TODO: allow more than just alphanumeric
pub fn attr(input: &str) -> IResult<&str, Attribute> {
    separated_pair(
        identifier,
        delimited(multispace0, char('='), multispace0),
        string,
    )(input)
    .map(|(next, (key, value))| {
        (
            next,
            Attribute {
                key: key.to_string(),
                value: value.to_string(),
            },
        )
    })
}

pub fn attributes(input: &str) -> IResult<&str, Vec<Attribute>> {
    many0(delimited(multispace0, attr, multispace0))(input)
}

pub fn any(input: &str) -> IResult<&str, &str> {
    Ok(("", input))
}

pub fn wrapped(input: &str) -> IResult<&str, &str> {
    delimited(tag("{:"), class, char('}'))(input)
}

pub fn attr_list_def(input: &str) -> IResult<&str, AttrListDef> {
    delimited(
        tag("{:"),
        tuple((many0(class), opt(id), attributes)),
        char('}'),
    )(input)
    .map(|(next, (classes, id, attributes))| {
        (
            next,
            AttrListDef {
                classes: classes.iter().map(ToString::to_string).map(Class).collect(),
                id: id.map(ToString::to_string),
                attributes,
            },
        )
    })
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    pub fn parse_class_is_ok() {
        let input = ".def";
        let parsed = class(input);
        println!("{:#?}", parsed);
        assert!(parsed.is_ok());
    }

    #[test]
    pub fn parse_attr_is_ok() {
        let input = r#"key="value""#;
        let parsed = attr(input);
        println!("{:#?}", parsed);
        assert!(parsed.is_ok());
    }

    #[test]
    pub fn parse_definition_is_ok() {
        let input = r#"{:.myclass .anotherclass #myid key="value" key2 = "two words"}"#;
        let parsed = attr_list_def(input);
        // println!("{:#?}", parsed);
        println!("{}", parsed.as_ref().unwrap().1);
        // println!(
        //     "{}",
        //     parsed.as_ref().unwrap().1.classes[0]
        // );
        assert!(parsed.is_ok());
    }

    #[test]
    pub fn parse_just_def() {
        let input = r#"{:.def}"#;
        let parsed = attr_list_def(input);
        assert!(parsed.is_ok());
    }
}
