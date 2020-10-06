use regex::Regex;
use std::collections::HashMap;
use std::ops::Index;

pub fn parse<'a>(html: &'a str) -> Node<'a> {
  let mut stack: Vec<&str> = Vec::new();
  let mut in_tag = false;
  let mut start_index = 0;
  let mut last_index = 0;

  for (pos, char) in html.char_indices() {
    if in_tag {
      if char != '>' {
        last_index += 1;
      } else {
        stack.push(&html[start_index..pos+1]);
        in_tag = false;
        start_index = pos + 1;
        last_index = pos + 1;
      }
    } else {
      if char == '<' {
        if start_index != last_index {
          let re = regex::Regex::new(r"^\s*(\S?[\w\s]*\S)\s*$").unwrap();
          if let Some(captures) = re.captures(&html[start_index..pos]) {
            let text = &captures[1];
            let i = String::from(&html[start_index..last_index+1]).find(text).unwrap();
            stack.push(&html[start_index+i..start_index+text.len()]);
          }
        }
        in_tag = true;
        start_index = pos;
        last_index = pos;
      } else {
        last_index += 1;
      }
    }
  }

  println!("{:#?}", stack);
  gen_tree(&stack)
}

fn gen_tree<'a>(stack: &Vec<&'a str>) -> Node<'a> {
  let mut root = Node::new(&stack[0]);
  let node = &mut root;
  let mut start_index = 1;
  let mut last_index = stack.len() - 2;

  while start_index <= last_index {
    let tag = &stack[start_index];

    if &tag[0..1] != "<" && &tag[tag.len()-1..tag.len()] != ">" {
      node.children.push(Node::new(&stack[start_index]));
      start_index += 1;
      continue;
    }

    if &tag[0..2] == "</" {
      panic!("Tag `{}` is not matched.", tag);
    }

    let mut close_tag = String::from("</");
    close_tag.push_str(&stack[start_index][1..]);
    if close_tag == stack[last_index] || start_index == last_index {
      node.children.push(Node::new(&stack[start_index]));
      start_index += 1;
      last_index -= 1;
    } else {
      node.children.push(Node::new(&stack[start_index]));
      start_index += 1;
    }
  }

  root
}

pub fn parse_props<'a>(str: &'a str)
  -> (String, HashMap<&'a str, &'a str>, Option<&'a str>)
{
  let mut prop_map: HashMap<&str, &str> = HashMap::new();
  let tag_re = Regex::new(r"([\w\d]+)\s*").unwrap();
  let re = Regex::new(r#"\s*([^=\s/>]+)\s*(?:=\s*['"](\S+)['"])*"#).unwrap();

  if &str[0..1] != "<" && &str[str.len()-1..str.len()] != ">" {
    return (String::from(""), prop_map, Some(str));
  }
  
  let tag_captures = tag_re.captures(str).unwrap();
  let props_start_index = tag_captures.index(0).len() + 1;
  let tag = String::from(tag_captures.index(1));

  let str = &str[props_start_index..];
  for captures in re.captures_iter(str) {
    let mut iter = captures.iter();
    iter.next();
    loop {
      let cap = match iter.next() {
        Some(v) => v,
        None => break
      };
      let key = if let Some(k) = cap {
        k.as_str()
      } else {
        break;
      };

      let cap = match iter.next() {
        Some(v) => v,
        None => break
      };
      let prop = if let Some(p) = cap {
        p.as_str()
      } else {
        ""
      };
      prop_map.insert(key, prop);
    }
  }

  (tag, prop_map, None)
}

#[derive(Debug)]
enum Type {
  Element,
  Text,
}

#[derive(Debug)]
pub struct Node<'a> {
  tag: String,
  node_type: Type,
  text: Option<&'a str>,
  props: HashMap<&'a str, &'a str>,
  children: Vec<Node<'a>>,
}

impl<'a> Node<'a> {
  fn new(tag_str: &str) -> Node {
    let (tag, props, text) = parse_props(tag_str);
    let mut node_type = Type::Element;
    if &tag == "" {
      node_type = Type::Text;
    }

    Node {
      tag,
      node_type,
      text,
      props,
      children: Vec::new()
    }
  }
}

#[cfg(test)]
mod tests {
  use super::parse_props;
  use std::collections::HashMap;

  #[test]
  fn test_parse_props() {
    let expect_tag = "img";
    let mut expect_props = HashMap::new();
    expect_props.insert("src", "xxx");
    expect_props.insert("alt", "yyy");

    let (tag, prop_map, text) = parse_props("<img src=\"xxx\" alt=\"yyy\" />");
    assert_eq!(tag, expect_tag);
    assert_eq!(prop_map, expect_props);
    assert_eq!(text, None);
  }
}
