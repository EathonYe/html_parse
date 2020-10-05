use regex::Regex;
use std::collections::HashMap;
use std::ops::Index;

pub fn parse<'a>(html: &str, stack: &'a mut Vec<String>) -> Node<'a> {
  let mut in_tag = false;
  let mut tag = String::new();
  for char in html.chars() {
    if in_tag {
      if char != '>' {
        tag.push(char);
      } else {
        if &tag[tag.len() -1..tag.len()] == "/" {
          stack.push(String::from(String::from(&tag[0..tag.len()-1]).trim()));
        } else {
          stack.push(tag.clone());
        }
        in_tag = false;
        tag.clear();
      }
    } else {
      if char == '<' {
        in_tag = true;
      }
    }
  }

  println!("{:#?}", stack);
  gen_tree(stack)
}

fn gen_tree(stack: &Vec<String>) -> Node {
  let mut root = Node::new(&stack[0]);
  let node = &mut root;
  let mut start_index = 1;
  let mut last_index = stack.len() - 2;

  while start_index <= last_index {
    let tag = &stack[start_index];
    if &tag[0..1] == "/" {
      panic!("Tag `{}` is not matched.", tag);
    }

    let mut close_tag = String::from("/");
    close_tag.push_str(&stack[start_index]);
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

pub fn parse_props<'a>(str: &'a str) -> (String, HashMap<&'a str, &'a str>) {
  let mut prop_map: HashMap<&str, &str> = HashMap::new();
  let tag_re = Regex::new(r"([\w\d]+)\s*").unwrap();
  let re = Regex::new(r#"\s*([^=]+)\s*(?:=\s*['"](\S+)['"])*"#).unwrap();
  
  let tag_captures = tag_re.captures(str).unwrap();
  let props_start_index = tag_captures.index(0).len();
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

  (tag, prop_map)
}

#[derive(Debug)]
pub struct Node<'a> {
  tag: String,
  props: HashMap<&'a str, &'a str>,
  children: Vec<Node<'a>>,
}

impl<'a> Node<'a> {
  fn new(tag_str: &str) -> Node {
    let (tag, props) = parse_props(tag_str);
    Node {
      tag,
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

    let (tag, prop_map) = parse_props("img src=\"xxx\" alt=\"yyy\"");
    assert_eq!(tag, expect_tag);
    assert_eq!(prop_map, expect_props);
  }
}
