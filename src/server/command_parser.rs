use regex::Regex;

#[derive(Clone, Copy)]
pub struct Command<'a> {
    pub name: &'a str,
    pub option: &'a str,
}

pub fn parse_command(input: &'_ str) -> Option<Command<'_>> {
  let re: Regex = Regex::new(r"^([a-z\?]+)[ ]*(.*?)$").unwrap();
  let caps = re.captures_iter(input).filter_map(|cap| {
      Some(Command {
          name: match cap.get(1) {
              Some(name) => name.as_str(),
              _ => {
                  return None;
              }
          },
          option: match cap.get(2) {
              Some(option) => option.as_str(),
              _ => "",
          },
      })
  });
  let vec = caps.collect::<Vec<Command>>();
  if vec.len() == 1 {
      Some(vec[0])
  } else {
      None
  }
}

#[cfg(test)]
mod tests {
    use super::parse_command;

    #[test]
    fn test_empty() {
        let command = parse_command("");
        assert!(command.is_none());
    }

    #[test]
    fn test_str_command() {
        let command = parse_command("luc");
        assert!(command.is_some());
        assert_eq!(command.unwrap().name, "luc");
        assert_eq!(command.unwrap().option, "");
    }

    #[test]
    fn test_str_command_inter() {
        let command = parse_command("luc?");
        assert!(command.is_some());
        assert_eq!(command.unwrap().name, "luc?");
        assert_eq!(command.unwrap().option, "");
    }

    #[test]
    fn test_str_command_and_option() {
        let command = parse_command("luc? opt opt ! opt ?");
        assert!(command.is_some());
        assert_eq!(command.unwrap().name, "luc?");
        assert_eq!(command.unwrap().option, "opt opt ! opt ?");
    }
}
