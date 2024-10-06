#[macro_use]
extern crate rocket;

use rocket::response::content::RawHtml;

#[get("/")]
fn index() -> RawHtml<String> {
    // Your HTML content here
    let html = r#"
    <!DOCTYPE html>
    <html lang="en">
    <head>
        <meta charset="UTF-8">
        <meta name="viewport" content="width=device-width, initial-scale=1.0">
        <title>Bitcoin Founder Mystery</title>
        <style>
            /* Your CSS styles here */
        </style>
    </head>
    <body>
        <div class="container">
            <div class="headline" onclick="copyToClipboard('87wEGGtM8vrywhFNSitSLC78jmhWLCiPpWGqqGyQpump')">CA: 87wEGGtM8vrywhFNSitSLC78jmhWLCiPpWGqqGyQpump</div>
            <div class="visual">
                You wanna find out who is the real bitcoin founder?<br>
                Click to the CA and find it out
            </div>
            <img src="https://pump.mypinata.cloud/ipfs/QmQWVNvHNVYj52LuV3UufZoc2eY9UUQ1FVrmDEyGEfm3Fi?img-width=256&img-dpr=2&img-onerror=redirect" alt="Bitcoin Logo" class="btc-logo">
        </div>
        <script>
            function copyToClipboard(text) {
                navigator.clipboard.writeText(text).then(function() {
                    alert('Copied to clipboard: ' + text);
                }, function(err) {
                    console.error('Could not copy text: ', err);
                });
            }
        </script>
    </body>
    </html>
    "#;
    RawHtml(html.to_string())
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}
