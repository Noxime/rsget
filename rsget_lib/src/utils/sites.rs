use regex::Regex;
use Streamable;
use utils::error::StreamError;
use plugins::{douyu, panda, xingyan, xingyan2};
// Option<Box<Streamable + 'static>>
pub fn get_site(input: &str) -> Result<Box<Streamable>, StreamError>
{
    let re_xingyan_panda: Regex = Regex::new(r"^(?:https?://)?xingyan\.panda\.tv/[0-9]+").unwrap();
    let re_panda: Regex = Regex::new(r"^(?:https?://)?(?:www\.)?panda\.tv/[0-9]+").unwrap();
    let re_douyu: Regex = Regex::new(r"^(?:https?://)?(?:www\.)?douyu\.com/[a-zA-Z0-9]+").unwrap();

    match input {
        url if re_panda.is_match(url) => {
            match panda::PandaTv::new(String::from(url)) {
                Ok(s) => Ok(s),
                Err(why) => Err(why),
            }
        },
        url if re_xingyan_panda.is_match(url) => {
            match xingyan::Xingyan::new(String::from(url)) {
                Ok(s) => Ok(s),
                Err(_why) => {
                    match xingyan2::Xingyan2::new(String::from(url)) {
                        Ok(s) => Ok(s),
                        Err(why) => Err(why),
                    }
                },
            }
        },
        url if re_douyu.is_match(url) => {
            match douyu::Douyu::new(String::from(url))  {
                Ok(s) => Ok(s),
                Err(why) => Err(why),
            }
        },
        _ => Err(StreamError::new("Site not supported")),
    }
}
