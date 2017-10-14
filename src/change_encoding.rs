use encoding::{EncodingRef, EncoderTrap, DecoderTrap};
use id3::{Tag};

use std::borrow::Cow;
use std::process::Command;

pub fn change(file_name: &str, from: EncodingRef, to: EncodingRef) {
    let tag = Tag::read_from_path(file_name).unwrap();

    let mut tag_encoding = TagEncoding {
        from: from,
        to: to,
        tag: tag
    };

    let mut command = Command::new("eyeD3");

    if let Some(artist) = tag_encoding.change_encoding_or_log_error(Tag::artist) {
        command.arg(format!("--artist={}", artist));
    }

    if let Some(title) = tag_encoding.change_encoding_or_log_error(Tag::title) {
        command.arg(format!("--title={}", title));   
    }

    if let Some(album) = tag_encoding.change_encoding_or_log_error(Tag::album) {
        command.arg(format!("--album={}", album));      
    }
    
    command
         .arg("--set-encoding=utf8")
         .arg("--to-v2.4")
         .arg(file_name);
                        
    let mut child = command
         .spawn()
         .expect("failed to execute child");

    let ecode = child.wait()
                     .expect("failed to wait on child");

    assert!(ecode.success());
}

struct TagEncoding {
    from: EncodingRef,
    to: EncodingRef,
    tag: Tag
}

impl TagEncoding {
    fn change_encoding_or_log_error<E>(&mut self, extractor: E) -> Option<String> where
        E: Fn(&Tag) -> Option<&str> {
        let with_changed_encoding = match self.change_encoding_for(extractor) {
            Err(err) => {
                println!("Could not change encoding: {:?}", err); 
                None
            },
            Ok(res) => res
        };
        with_changed_encoding
    }

    fn change_encoding_for<E>(&mut self, extractor: E) -> Result<Option<String>, Cow<'static, str>> where
        E: Fn(&Tag) -> Option<&str> {
            let mut value_opt: Option<String> = None;
            {
                if let Some(value) = extractor(&self.tag) {
                    value_opt = Some(value.into());
                }
            }
            if let Some(value) = value_opt {
                let encoded = self.from.encode(&value, EncoderTrap::Strict)?;
                let decoded = self.to.decode(&encoded, DecoderTrap::Strict)?;
                return Ok(Some(decoded.into()));
            }
            Ok(None)
    }
}
