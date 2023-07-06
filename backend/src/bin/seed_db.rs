extern crate openssl;

use backend_api::*;
use std::io::{stdin, stdout, Read, Write};
use diesel::{RunQueryDsl, PgConnection};

use termion::event::Key;
use termion::input::TermRead;

pub struct Prompt {
    pub title: &'static str,
    pub actions: &'static[Action],
}

trait Executable {
    fn go(&self) -> ();
    fn create_post(&self, conn: &mut PgConnection) -> ();
    fn create_video(&self, conn: &mut PgConnection) -> ();
    // fn create_listing(&self, conn: &mut PgConnection) -> ();
    // fn create_listing_image(&self, conn: &mut PgConnection) -> ();
    // fn create_review(&self, conn: &mut PgConnection) -> ();
}

pub struct Action {
    pub title: &'static str,
    pub func_id: &'static str,
}

impl Executable for Action {
    fn create_post(&self, conn: &mut PgConnection) -> () {
        use crate::posts::NewPost;
        use crate::schema::posts::table;

        let mut img_url = String::new();
        let mut title = String::new();
        let mut body = String::new();

        println!("What is the post title?");
        stdin().read_line(&mut title).unwrap();
        let title = title.trim_end().to_string();
        println!();

        println!("What is the post's image url?");
        stdin().read_line(&mut img_url).unwrap();
        let img_url = img_url.trim_end().to_string();
        println!();

        println!("Ok! Let's write the post body (Press {EOF} when finished)",);
        stdin().read_to_string(&mut body).unwrap();

        let post = NewPost { title, body, img_url };

        diesel::insert_into(table)
            .values(&post)
            .execute(conn)
            .expect("Failed to insert post into db!");

        println!("\nSuccessfully saved post!");
        println!();
    }

    fn create_video(&self, conn: &mut PgConnection) {
        use crate::videos::NewVideo;
        use crate::schema::videos::table;

        let mut url = String::new();
        let mut title = String::new();

        println!("What is the video title?");
        stdin().read_line(&mut title).unwrap();
        let title = title.trim_end().to_string();
        println!();

        println!("What is the video's url?");
        stdin().read_line(&mut url).unwrap();
        let url = url.trim_end().to_string();
        println!();

        let video = NewVideo { title, video_url: url };

        diesel::insert_into(table)
            .values(&video)
            .execute(conn)
            .expect("Failed to insert video into db!");

        println!("\nSuccessfully saved video!");
        println!();
    }

    fn go(&self) -> () {
        let conn = &mut establish_connection();
        match self.func_id {
            "create_post" => self.create_post(conn),
            "create_video" => self.create_video(conn),
            &_ => todo!(),
        }
    }
}

static PROMPTS: &'static[Prompt] = &[
    Prompt {
        title: "Manage posts",
        actions: &[
            Action { title: "Create post", func_id: "create_post" },
        ]
    },
    Prompt {
        title: "Manage videos",
        actions: &[
            Action { title: "Create video", func_id: "create_video" },
        ]
    }
];

fn create_review() {

}

fn create_listing() {

}

fn create_listing_image() {

}

fn main() {
    println!("Welcome to the CLI for interacting with ma_p_site db!\n");

    loop {
        for (i, p) in PROMPTS.iter().enumerate() {
            println!("{}. {}", i, p.title);
        }

        println!();
        print!("Choose one of the options above to get started: ");
        stdout().flush().unwrap();

        let mut input_line = String::new();
        stdin().read_line(&mut input_line).unwrap();
        let mut option: usize = input_line.trim().parse().unwrap();
        println!();

        if option < PROMPTS.len() {
            let p = PROMPTS.get(option).unwrap();
            
            for (i, a) in p.actions.iter().enumerate() {
                println!("{}. {}", i, a.title);
            }

            println!();
            print!("Choose one of the options above: ");
            stdout().flush().unwrap();

            input_line.clear();
            stdin().read_line(&mut input_line).unwrap();
            option = input_line.trim().parse().unwrap();
            println!();

            if option < p.actions.len() {
                let a = p.actions.get(option).unwrap();
                a.go();
            }
        }

        println!("To restart the CLI, press any key. Press {EOF} or q to quit.",);
        let mut again: bool = true;

        for c in stdin().keys() {
            match c.unwrap() {
                Key::Char('q') => { again = false; },
                Key::Char(c) => break,
                Key::Alt(c) => break,
                Key::Ctrl(c) => break,
                _ => { break; },
            }
            break;
        }

        stdout().flush().unwrap();
        if !again { break; };
    }  
}

#[cfg(not(windows))]
const EOF: &str = "CTRL+D";

#[cfg(windows)]
const EOF: &str = "CTRL+Z";
