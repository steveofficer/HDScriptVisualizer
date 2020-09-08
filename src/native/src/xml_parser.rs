use nom::{
    IResult,
    multi::{ many_till, fold_many0 },
    branch::alt,
    combinator::{ map, opt, value, peek, not },
    bytes::complete::{ take_while, tag, take },
    character::complete::{ anychar, multispace0 },
    sequence::{ separated_pair, preceded, tuple, delimited }
  };
  
  use std::iter::{ FromIterator, IntoIterator };
  
  use std::collections::HashMap;

  #[derive(Debug, PartialEq)]
  pub enum Node<'a> {
    XmlElement(XmlElement<'a>),
    Value(String)
  }
  
  impl Node<'_> {
    pub fn as_value(&self) -> &str {
      match self {
        Node::Value(value) => value,
        _ => "undefined"
      }
    }
  
    pub fn as_element(&self) -> &XmlElement {
      match self {
        Node::XmlElement(e) => e,
        _ => panic!("Node is not an element")
      }
    }
  }
  
  #[derive(Debug, PartialEq)]
  pub struct XmlElement<'a> {
    pub name: &'a str,
    pub attributes: HashMap<&'a str, String>,
    pub children: Vec<Node<'a>>
  }
  
  fn unescape_xml_literal(bad_char: char) -> impl Fn(&str) -> IResult<&str, &str> {
        move |input| {
            let _ = peek(not(nom::character::complete::char(bad_char)))(input)?;
            let (input, ch) = take(1usize)(input)?;

            if ch == "&" {
                alt((
                    value("<", tag("lt;")),
                    value(">", tag("gt;")),
                    value("\"", tag("quot;")),
                    value("'", tag("apos;")),
                    value("&", tag("amp;")),
                    value("\r", tag("#xD;")),
                    value("\n", tag("#xA;"))
                ))(input)
            } else {
                Ok((input, ch))
            }
        }
    }

  fn read_element_start(input: &str) -> IResult<&str, ()> { 
    value((), preceded(multispace0, tag("<")))(input)
  }
  
  fn read_element_name(input: &str) -> IResult<&str, &str> {
      take_while(|c: char| c.is_alphanumeric() || c == '_' || c == '.' || c == ':')(input)
  }
  
  fn read_element_end(input: &str) -> IResult<&str, bool> { 
    alt((
      value(false, tag(">")),
      value(true, tag("/>"))
    ))(input)
  }
  
  fn read_attribute(input: &str) -> IResult<&str, (&str, String)> {
    let (input, (attr_name, attr_value)) =
      separated_pair(
        take_while(|c| c != '='),
        tag("="),
        delimited(tag("\""), fold_many0(unescape_xml_literal('"'), String::new(), |mut s: String, r| { s.push_str(r); s }), tag("\""))
      )(input)?;
  
    Ok((input, (attr_name, attr_value)))
  }
  
  fn read_text_element(input: &str) -> IResult<&str, String> {
    fold_many0(unescape_xml_literal('<'), String::new(), |mut s: String, r| { s.push_str(r); s } )(input)
  }
  
  fn read_element(input: &str) -> IResult<&str, XmlElement> {
    let (input, (name, (attributes, is_closed))) =
      tuple((
        preceded(read_element_start, read_element_name),
        many_till(preceded(multispace0, read_attribute), preceded(multispace0, read_element_end))
      ))
      (input)?;
    
    if is_closed {
      // The element is self closing, therefore it cannot have children
      Ok((input, XmlElement { name: name, attributes: HashMap::from_iter(attributes.into_iter()), children: vec![] }))
    } else {

      // This element is still open, so it could have children
      let (input, (children, _)) = many_till(
        map(
          opt(alt((
              map(read_element, Node::XmlElement),
              map(read_text_element, Node::Value)
          ))), 
          |r| match r {
              Option::None => Node::Value(String::from("")),
              Option::Some(a) => a
          }
        ), 
        preceded(multispace0, tag(format!("</{}>", name).as_str()))
      )(input)?;
      
      Ok((input, XmlElement { name: name, attributes: HashMap::from_iter(attributes.into_iter()), children: children }))
    }
  }
  
  fn read_decl(input: &str) -> IResult<&str, ()> {
      let (input, _) = 
          preceded(
              opt(tag("\u{feff}")),
              tuple((
                  tag("<?"),
                  many_till(anychar, tag("?>"))
              ))
          )(input)?;
      Ok((input, ()))
  }
  
  pub fn parse(input: &str) -> IResult<&str, XmlElement> {
    preceded(opt(read_decl), read_element)(input)
  }