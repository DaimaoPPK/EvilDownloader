/*
* ISC License
*
* Copyright <2021> <Phone Pyae Kyaw>
*
* Permission to use, copy, modify, and/or distribute this software for any purpose with or without fee is hereby granted, provided that the above copyright notice and this permission notice appear in all copies.
*
* THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.
*/

use std::{io::prelude::Write, fs::File, fmt};

#[derive(Clone)]
pub struct URL{
    pub url: String
}

pub struct Download{
    pub url: URL,
    pub path: String,
}

pub struct Summary{
    pub url: URL,
    pub status: i32,
    pub filename: String
}

impl URL{
    pub fn new(url: &str) -> URL{
        URL{url: String::from(url)}
    }

    pub fn segments(&self) -> Vec<String>{
        let mut ret: Vec<String> = Vec::new();
        for s in self.url.split("/"){
            ret.push(String::from(s));
        }
        ret
    }
}

impl std::convert::From<&str> for URL{
    fn from(str: &str) -> Self{
        URL{
            url: String::from(str)
        }
    }
}

impl std::convert::From<String> for URL{
    fn from(str: String) -> Self{
        URL{
            url: String::from(str)
        }
    }
}

impl Download{
    pub fn new(url: &str, path: &str) -> Download{
        let url = URL::from(url);
        Download{
            url,
            path: String::from(path)
        }
    }

    pub fn download(&self) -> Result<Summary, minreq::Error>{
        let filename = self.url.segments().last().unwrap().clone();
        let response = minreq::get(&self.url.url).send()?;
        let mut file = match File::create(filename.as_str()) {
            Err(err) => panic!("couldn't create {}", err),
            Ok(file) => file,
        };
        file.write_all(response.as_bytes())?;
        Ok(Summary::new(self.url.clone(), response.status_code, filename))
    }
}

impl Summary{
    pub fn new(url: URL, status: i32, filename: String) -> Summary{
        Summary{
            url,
            status,
            filename
        }
    }
}

impl fmt::Display for URL{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        write!(f, "{}", self.url)
    }
}