use std::fmt::Display;

#[derive(Clone, Debug)]
pub enum TimeStampActions {
    Start,
    Stop,
}

impl Display for TimeStampActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TimeStampActions::Start => write!(f, "START"),
            TimeStampActions::Stop => write!(f, "STOP"),
        }
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_display_start() {
        assert_eq!("START", format!("{}", TimeStampActions::Start));
    }

    #[test]
    fn test_display_stop() {
        assert_eq!("STOP", format!("{}", TimeStampActions::Stop));
    }
}
