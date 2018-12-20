extern create iron;
#[macro_use] extern crate mine;

use iron::prelude::*;
use iron::status;

fn main(){
    println!("serving on :3000")
    Iron::new(get_form).http("localhost:3000").unwrap();
}

fn get_form(_request: &mut Request) -> IronResult<Response>{
    let mut response = Response::new();

    response.set_mut(status::Ok);
    response.set_mut(mine!(Text/Html; Charset=Utf8));
    response.set_mut(r#"
        <title>GDC Calculator</title>
        <form action="/gcd" method="post">
            <input type="text" name="n"/>
            <input type="text" name="n"/>
            <button type="submit">Compute GCD</button>
        </form>
    "#)
}
