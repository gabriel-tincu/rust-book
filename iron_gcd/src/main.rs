extern crate iron;
extern crate urlencoded;
extern crate router;

#[macro_use] extern crate mime;
use router::Router;
use std::str::FromStr;
use urlencoded::UrlEncodedBody;
use iron::prelude::*;
use iron::status;

fn main() {
    let mut router = Router::new();
    router.get("/", get_form, "root");
    router.post("/gcd", post_gcd, "gcd");
    println!("Serving on localhost 3000");
    Iron::new(router).http("localhost:3000").unwrap();
}



fn get_form(_request: &mut Request) -> IronResult<Response> {
    let mut response = Response::new();

    response.set_mut(status::Ok);
    response.set_mut(mime!(Text/Html; Charset=Utf8));
    response.set_mut(r#"
        <title>GCD Calculator</title>
        <form action="/gcd" method="post">
            <input type="text" name="n"/>
            <input type="text" name="n"/>
            <button type="submit">Compute GCD</Button>
        </form>
    "#);
    Ok(response)
}

fn post_gcd(request: &mut Request) -> IronResult<Response> {
    let mut response = Response::new();
    let form_data = match request.get_ref::<UrlEncodedBody>() {
        Err(e) => {
            response.set_mut(status::BadRequest);
            response.set_mut(format!("Error parsing form data: {:?}\n",e));
            return Ok(response);
        }
        Ok(map) => map
    };
    let unparsed_numbers = match form_data.get("n") {
        None => {
            response.set_mut(status::BadRequest);
            response.set_mut(format!("form data has no 'n' parameters\n"));
            return Ok(response);
        }
        Some(nums) => nums
    };

    let mut numbers = Vec::new();

    for unparsed in unparsed_numbers {
        match u64::from_str(&unparsed) {
            Err(e) => {
                response.set_mut(status::BadRequest);
                response.set_mut(format!("value for 'n' parameter not a number: {:?}\n\t{}", unparsed, e));
                return Ok(response);
            }
            Ok(num) => {numbers.push(num);}
        }
    }
    let mut n = numbers[0];
    for m in &numbers[1..] {
        n = gcd(n, *m);
    }
    response.set_mut(status::Ok);
    response.set_mut(mime!(Text/Html; Charset=Utf8));
    response.set_mut(format!(
        "The GCD of the numbers {:?} is <b>{}</b>", numbers, n
    ));
    Ok(response)
}

fn gcd(mut n: u64, mut m: u64) -> u64 {
    assert!(m != 0 && n != 0);
    while m != 0  && n != 0{
        if m < n {
            n = n % m;
        } else {
            m = m % n;
        }
    }
    if n > m {
        return n;
    } else {
        return m;
    }
}

