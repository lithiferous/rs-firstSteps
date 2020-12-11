pub trait Summary {
    fn sumup_author(&self) -> String;
    fn sumup(&self) -> String {
        format!("Readmore...: {}", self.sumup_author())
    }
}

pub struct NewsArt {
    pub headline: String,
    pub location: String,
    pub author:   String,
    pub content:  String,
}

impl Summary for NewsArt {
    fn sumup_author(&self) -> String
    {
        format!("( )*( ) from {}", self.author)
    }
    fn sumup(&self) -> String
    {
        format!("{}, by {} ({})", self.headline,
                                  self.author,
                                  self.content)
    }
}

pub struct Tweet {
    pub username: String,
    pub content:  String,
    pub reply:    bool,
    pub retweet:  bool,
}

impl Summary for Tweet {
    fn sumup_author(&self) -> String
    {
        format!("( )*( ) from {}", self.username)
    }
    fn sumup(&self) -> String
    {
        format!("{}: {}", self.username,
                          self.content)
    }

}

pub fn notify<T, U>(t: T, u: U)
    where  T: Summary,
           U: Summary
{
    println!("breaking my neurons with rust: {}", t.sumup());
    println!("breaking news I have small pp: {}", u.sumup());
}
