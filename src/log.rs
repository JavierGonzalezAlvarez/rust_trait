use std::fs;
use chrono::{DateTime, Utc, Duration}; 
use std::time::SystemTime;

pub struct NewLog {
    pub text: String,
    pub file: String,
}

pub trait Logger {
    fn loggtext(&self) -> String;    
    fn finding(&self, gettext: String, getfile: String);    
}

impl Logger for NewLog {
    fn loggtext(&self) -> String {
        format!("checking data in .log (text to search, file) => {} - {}", self.text, self.file)
    }
    
    fn finding(&self, gettext: String, getfile: String) {    
        let now = SystemTime::now();
        let datetime: DateTime<Utc> = now.into();
        let days = datetime - Duration::days(1);    
        println!("date to search => {}", days.format("%Y-%m-%d"));    
        let days = days.format("%Y-%m-%d").to_string();
        let days_tostring = &days;    
               
        let filename = format!("./files/{}", getfile);
    
        let contents = fs::read_to_string(filename).expect("Reading file log");    
        let mut line_num: usize = 1;
        let mut value: bool = false;
        
        for line in contents.lines() {    
            if line.contains(&gettext) && line.contains(days_tostring) {
                println!{"error in line No => {}: string => {}", line_num, line}
                value = true;
            }        
            line_num += 1;
        }
        if value {                        
            println!("found text");
        }   
    }
}

pub fn find() {    
    //one search
    let log1 = NewLog {        
        text: "javier".to_string(),
        file: "data.log".to_string()
    };                       
    println!("log1 {}", log1.loggtext());
    log1.finding(log1.text.to_string(), log1.file.to_string());
        
    //another search
    let log2 = NewLog {        
        text: "data".to_string(),
        file: "data.log".to_string()
    };                       
    println!("log2 {}", log2.loggtext());
    log1.finding(log2.text.to_string(), log2.file.to_string()); 
}