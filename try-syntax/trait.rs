struct NewsArticle {
    headline: String,
    location: String,
    author: String,
    content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

struct Tweet {
    username: String,
    content: String,
    reply: bool,
    retweet: bool,
}

struct Tweet2 {
    username: String,
    content: String,
    reply: bool,
    retweet: bool,
}

impl Summary for Tweet2 {}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

trait Summary {
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}

fn notify<T: Summary>(item: T) {
    // 新ニュース！ {}
    println!("Breaking news! {}", item.summarize());
}

fn main () {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        // もちろん、ご存知かもしれないようにね、みなさん
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };
    println!("1 new tweet: {}", tweet.summarize());

    let article = NewsArticle {
        // ペンギンチームがスタンレーカップチャンピオンシップを勝ち取る！
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        // ピッツバーグ、ペンシルベニア州、アメリカ
        location: String::from("Pittsburgh, PA, USA"),
        // アイスバーグ
        author: String::from("Iceburgh"),
        // ピッツバーグ・ペンギンが再度NHL(National Hockey League)で最強のホッケーチームになった
        content: String::from("The Pittsburgh Penguins once again are the best
        hockey team in the NHL."),
    };

    // 新しい記事が利用可能です！ {}
    println!("New article available! {}", article.summarize());

    let tweet = Tweet2 {
        username: String::from("horse_ebooks"),
        // もちろん、ご存知かもしれないようにね、みなさん
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };
    println!("1 new tweet: {}", tweet.summarize());
}