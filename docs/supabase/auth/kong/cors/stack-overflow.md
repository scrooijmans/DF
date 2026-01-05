
Modified 1 month ago
Viewed 4.3m times
1370

I'm trying to fetch some data from the REST API of HP Alm. It works pretty well with a small curl script—I get my data.

Now doing that with JavaScript, fetch and ES6 (more or less) seems to be a bigger issue. I keep getting this error message:

Fetch API cannot load . Response to preflight request doesn't pass access control check: No 'Access-Control-Allow-Origin' header is present on the requested resource. Origin 'http://127.0.0.1:3000' is therefore not allowed access. The response had HTTP status code 501. If an opaque response serves your needs, set the request's mode to 'no-cors' to fetch the resource with CORS disabled.

I understand that this is because I am trying to fetch that data from within my localhost and the solution should be using Cross-Origin Resource Sharing (CORS). I thought I actually did that, but somehow it either ignores what I write in the header or the problem is something else.

So, is there an implementation issue? Am I doing it wrong? I can't check the server logs unfortunately. I'm really a bit stuck here.

function performSignIn() {

  let headers = new Headers();

  headers.append('Content-Type', 'application/json');
  headers.append('Accept', 'application/json');

  headers.append('Access-Control-Allow-Origin', 'http://localhost:3000');
  headers.append('Access-Control-Allow-Credentials', 'true');

  headers.append('GET', 'POST', 'OPTIONS');

  headers.append('Authorization', 'Basic ' + base64.encode(username + ":" + password));

  fetch(sign_in, {
      //mode: 'no-cors',
      credentials: 'include',
      method: 'POST',
      headers: headers
    })
    .then(response => response.json())
    .then(json => console.log(json))
    .catch(error => console.log('Authorization failed : ' + error.message));
}
I am using Chrome. I also tried using that Chrome CORS Plugin, but then I am getting another error message:

The value of the 'Access-Control-Allow-Origin' header in the response must not be the wildcard '*' when the request's credentials mode is 'include'. Origin 'http://127.0.0.1:3000' is therefore not allowed access. The credentials mode of requests initiated by the XMLHttpRequest is controlled by the withCredentials attribute.

javascriptcorsfetch-api
Share
Improve this question
Follow
edited Aug 14, 2022 at 12:48
Braiam's user avatar
Braiam
4,4961111 gold badges5050 silver badges8383 bronze badges
asked May 9, 2017 at 13:47
daniel.lozynski's user avatar
daniel.lozynski
16.9k66 gold badges2121 silver badges2121 bronze badges
Add a comment
34 Answers
Sorted by:

Highest score (default)
1
2
Next
1820

This answer covers a lot of ground, so it’s divided into three parts:

How to use a CORS proxy to avoid “No Access-Control-Allow-Origin header” problems
How to avoid the CORS preflight
How to fix “Access-Control-Allow-Origin header must not be the wildcard” problems
How to use a CORS proxy to avoid “No Access-Control-Allow-Origin header” problems

If you don’t control the server your frontend code is sending a request to, and the problem with the response from that server is just the lack of the necessary Access-Control-Allow-Origin header, you can still get things to work—by making the request through a CORS proxy.

You can easily run your own proxy with code from https://github.com/Rob--W/cors-anywhere/.
You can also easily deploy your own proxy to Heroku in just 2-3 minutes, with 5 commands:

git clone https://github.com/Rob--W/cors-anywhere.git
cd cors-anywhere/
npm install
heroku create
git push heroku master
After running those commands, you’ll end up with your own CORS Anywhere server running at, e.g., https://cryptic-headland-94862.herokuapp.com/.

Now, prefix your request URL with the URL for your proxy:

https://cryptic-headland-94862.herokuapp.com/https://example.com
Adding the proxy URL as a prefix causes the request to get made through your proxy, which:

Forwards the request to https://example.com.
Receives the response from https://example.com.
Adds the Access-Control-Allow-Origin header to the response.
Passes that response, with that added header, back to the requesting frontend code.
The browser then allows the frontend code to access the response, because that response with the Access-Control-Allow-Origin response header is what the browser sees.

This works even if the request is one that triggers browsers to do a CORS preflight OPTIONS request, because in that case, the proxy also sends the Access-Control-Allow-Headers and Access-Control-Allow-Methods headers needed to make the preflight succeed.

How to avoid the CORS preflight

The code in the question triggers a CORS preflight—since it sends an Authorization header.

https://developer.mozilla.org/docs/Web/HTTP/Access_control_CORS#Preflighted_requests

Even without that, the Content-Type: application/json header will also trigger a preflight.

What “preflight” means: before the browser tries the POST in the code in the question, it first sends an OPTIONS request to the server, to determine if the server is opting-in to receiving a cross-origin POST that has Authorization and Content-Type: application/json headers.

It works pretty well with a small curl script - I get my data.

To properly test with curl, you must emulate the preflight OPTIONS the browser sends:

curl -i -X OPTIONS -H "Origin: http://127.0.0.1:3000" \
    -H 'Access-Control-Request-Method: POST' \
    -H 'Access-Control-Request-Headers: Content-Type, Authorization' \
    "https://the.sign_in.url"
…with https://the.sign_in.url replaced by whatever your actual sign_in URL is.

The response the browser needs from that OPTIONS request must have headers like this:

Access-Control-Allow-Origin:  http://127.0.0.1:3000
Access-Control-Allow-Methods: POST
Access-Control-Allow-Headers: Content-Type, Authorization
If the OPTIONS response doesn’t include those headers, the browser will stop right there and never attempt to send the POST request. Also, the HTTP status code for the response must be a 2xx—typically 200 or 204. If it’s any other status code, the browser will stop right there.

The server in the question responds to the OPTIONS request with a 501 status code, which apparently means it’s trying to indicate it doesn’t implement support for OPTIONS requests. Other servers typically respond with a 405 “Method not allowed” status code in this case.

So you’ll never be able to make POST requests directly to that server from your frontend JavaScript code if the server responds to that OPTIONS request with a 405 or 501 or anything other than a 200 or 204 or if doesn’t respond with those necessary response headers.

The way to avoid triggering a preflight for the case in the question would be:

if the server didn’t require an Authorization request header but instead, e.g., relied on authentication data embedded in the body of the POST request or as a query param
if the server didn’t require the POST body to have a Content-Type: application/json media type but instead accepted the POST body as application/x-www-form-urlencoded with a parameter named json (or whatever) whose value is the JSON data
How to fix “Access-Control-Allow-Origin header must not be the wildcard” problems

I am getting another error message:

The value of the 'Access-Control-Allow-Origin' header in the response must not be the wildcard '*' when the request's credentials mode is 'include'. Origin 'http://127.0.0.1:3000' is therefore not allowed access. The credentials mode of requests initiated by the XMLHttpRequest is controlled by the withCredentials attribute.

For requests that have credentials, browsers won’t let your frontend JavaScript code access the response if the value of the Access-Control-Allow-Origin header is *. Instead the value in that case must exactly match your frontend code’s origin, http://127.0.0.1:3000.

See Credentialed requests and wildcards in the MDN HTTP access control (CORS) article.

If you control the server you’re sending the request to, a common way to deal with this case is to configure the server to take the value of the Origin request header, and echo/reflect that back into the value of the Access-Control-Allow-Origin response header; e.g., with nginx:

add_header Access-Control-Allow-Origin $http_origin
But that’s just an example; other (web) server systems have similar ways to echo origin values.

I am using Chrome. I also tried using that Chrome CORS Plugin

That Chrome CORS plugin apparently just simplemindedly injects an Access-Control-Allow-Origin: * header into the response the browser sees. If the plugin were smarter, what it would be doing is setting the value of that fake Access-Control-Allow-Origin response header to the actual origin of your frontend JavaScript code, http://127.0.0.1:3000.

So avoid using that plugin, even for testing. It’s just a distraction. To test what responses you get from the server with no browser filtering them, you’re better off using curl -H as above.

As far as the frontend JavaScript code for the fetch(…) request in the question:

headers.append('Access-Control-Allow-Origin', 'http://localhost:3000');
headers.append('Access-Control-Allow-Credentials', 'true');
Remove the lines above. The Access-Control-Allow-* headers are response headers. You never want to send them in requests. The only effect of that is to trigger a browser to execute a preflight.

Share
Improve this answer
Follow
edited Aug 13 at 9:32
Quentin's user avatar
Quentin
948k135135 gold badges1.3k1.3k silver badges1.4k1.4k bronze badges
answered May 9, 2017 at 23:03
sideshowbarker's user avatar
sideshowbarker
89k3030 gold badges218218 silver badges215215 bronze badges
32
Superb answer, my issue was the remote server not responding to OPTIONS requests, so after fiddling about with requests and headers for what seemed like ages I resolved it by removing the headers Content-Type and Access-Control-Allow-Origin - thank you! – 
Morvael
 CommentedJan 29, 2019 at 9:50
1
Fantastic answer to a common problem. FWIW, we had server responding with 405 (Method not supported) when adding a 'Authorization: Bearer' header to a request, so the fix was to move the token to a POST field instead as control of the server response to the OPTIONS check was not possible in the time frame given by the client. – 
Beerswiller
 CommentedJul 17, 2020 at 5:45
2
Actually after upload the new version of extension to the Azure DevOps, you have to update rights for it in the dev.azure.com{your_organization}/_settings/extensions?tab=installed when you add new scopes "scopes": ["vso.build"] in the manifest file. – 
Rodrigo Cipriani da Rosa
 CommentedAug 16, 2020 at 14:14
11
https://cors-anywhere.herokuapp.com/ is now no longer usable. The client will now receive a 403 Forbidden error - unless the developer explicitly requests to get temporarily passed. Here is the announcement: github.com/Rob--W/cors-anywhere/issues/301. I'd suggest to simply remove the cors-anywhere reference from the answer since it's no longer useful. – 
Boghyon Hoffmann
 CommentedFeb 7, 2021 at 11:42
the part of "xxx-Credentials: true " is updated. I think it is stopped to allow access CORS calls when -Origin is set matches the browser url but different to the HTTP call endpoint. – 
Vincent
 CommentedSep 2, 2021 at 11:39
Show 2 more comments
243

This error occurs when the client URL and server URL don't match, including the port number. In this case you need to enable your service for CORS which is cross origin resource sharing.

If you are hosting a Spring REST service then you can find it in the blog post CORS support in Spring Framework.

If you are hosting service using a Node.js server then

Stop the Node.js server.
npm install cors --save
Add following lines to your server.js
const cors=require("cors");
const corsOptions ={
   origin:'*', 
   credentials:true,            //access-control-allow-credentials:true
   optionSuccessStatus:200,
}

app.use(cors(corsOptions)) // Use this after the variable declaration
Share
Improve this answer
Follow
edited Aug 30, 2021 at 9:10
Yashwardhan Pauranik's user avatar
Yashwardhan Pauranik
5,58655 gold badges4545 silver badges7777 bronze badges
answered Oct 28, 2017 at 9:17
Rakesh's user avatar
Rakesh
4,28222 gold badges2121 silver badges3131 bronze badges
2
Note: you also need to remove your mode: 'no-cores' settings from your fetch() options on the client. Your JSON body won't be sent unless mode: 'cores' in your request. – 
Kyle Baker
 CommentedMar 14, 2021 at 3:25 
Add a comment
175

The problem arose because you added the following code as the request header in your front-end:

headers.append('Access-Control-Allow-Origin', 'http://localhost:3000');
headers.append('Access-Control-Allow-Credentials', 'true');
Those headers belong to the response, not request. So remove them, including the line:

headers.append('GET', 'POST', 'OPTIONS');
Your request had 'Content-Type: application/json', hence triggered what is called CORS preflight. This caused the browser sent the request with the OPTIONS method. See CORS preflight for detailed information.

Therefore in your back-end, you have to handle this preflighted request by returning the response headers which include:

Access-Control-Allow-Origin : http://localhost:3000
Access-Control-Allow-Credentials : true
Access-Control-Allow-Methods : GET, POST, OPTIONS
Access-Control-Allow-Headers : Origin, Content-Type, Accept
Of course, the actual syntax depends on the programming language you use for your back-end.

In your front-end, it should be like so:

function performSignIn() {
    let headers = new Headers();

    headers.append('Content-Type', 'application/json');
    headers.append('Accept', 'application/json');
    headers.append('Authorization', 'Basic ' + base64.encode(username + ":" +  password));
    headers.append('Origin','http://localhost:3000');

    fetch(sign_in, {
        mode: 'cors',
        credentials: 'include',
        method: 'POST',
        headers: headers
    })
    .then(response => response.json())
    .then(json => console.log(json))
    .catch(error => console.log('Authorization failed: ' + error.message));
}
Share
Improve this answer
Follow
edited Jan 5, 2022 at 22:18
Peter Mortensen's user avatar
Peter Mortensen
31.5k2222 gold badges110110 silver badges134134 bronze badges
answered Jan 24, 2019 at 8:30
Lex Soft's user avatar
Lex Soft
2,50622 gold badges1515 silver badges1313 bronze badges
1
hey, whats 'Header()' please? – 
mitsu
 CommentedDec 17, 2019 at 20:44
2
@mitsu If you refer to the line : let headers = new Headers(); above, then it is an interface of fetch API to do actions with http request or response headers. Visit developer.mozilla.org/en-US/docs/Web/API/Headers to get the details as well as examples on using it. – 
Lex Soft
 CommentedJan 3, 2020 at 13:41
Note that we don't have to specify the OPTIONS method to allow. I.e., Access-Control-Allow-Methods : GET, POST would be enough; see this question. – 
starriet 차주녕
 CommentedMar 29, 2024 at 6:06
Add a comment
29

In my case, I use the below solution.

Front-end or Angular:

post(
    this.serverUrl, dataObjToPost,
    {
      headers: new HttpHeaders({
           'Content-Type':  'application/json',
         })
    }
)
Back-end (I use PHP):

header("Access-Control-Allow-Origin: http://localhost:4200");
header('Access-Control-Allow-Methods: GET, POST, OPTIONS');
header("Access-Control-Allow-Headers: Content-Type, Authorization");

$postdata = file_get_contents("php://input");
$request = json_decode($postdata);
print_r($request);

****************

The problem here is that CORS mechanism before each request, makes an preflight request using http OPTIONS method.

According to mozilla website:

CORS also relies on a mechanism by which browsers make a "preflight" request to the server hosting the cross-origin resource, in order to check that the server will permit the actual request. In that preflight, the browser sends headers that indicate the HTTP method and headers that will be used in the actual request.

[SOLUTION]

Then, to fix this problem we need to expose for each route the http OPTIONS method. Then, in the API gateway configuration file (kong.ymal) for each route I created a copy of the route (named route_name_preflight) with http OPTIONS method and without plugins and authentications as you can see bellow (kong.yaml):

added preflight route for each route

It is important to reinforce that as I said above, my backend microservices controller classes have @CrossOrigin annotation, that probably exposes the preflight request for each controller request, this being necessary.

In the frontend app (vuejs) I started using axios() instead of axios.method(), because is better for add headers (f.e. Authorization with jwt tokens) and other stuff:

const options = {
    url: process.env.VUE_APP_BACKEND_URL + '/api/client/authenticate',
    method: 'POST',
    headers: {
        'Accept': 'application/json',
        'Content-Type': 'application/json;charset=UTF-8',
        'Authorization': 'Basic ' + window.btoa(this.username + ':' + this.password),
    }
};
await axios(options).then(...)


************
Preflight OPTIONS request not handled in Kong gateway #4859
Closed
Closed
Preflight OPTIONS request not handled in Kong gateway
#4859
@nchaudhu
Description
nchaudhu
opened on Aug 2, 2019
I have configured the cors plugin in kong . I am now trying to make a POSt request from the front end . This initiates a Preflight OPTIONS request . But Kong gives 404 not found error.

By doing a curl I am able to hit my upstream service , but from front end app I am facing this cors issue inspite of having configured cors plugin on the service.

Please help - this is urgent

Below is the cors plugin configuration

curl -i -X POST http://localhost:8001/routes/4a904652-0107-4b8f-97e3-73bd5a96c8d2/plugins \

--data "name=cors"
--data "config.origins=http://localhost:4200"
--data "config.methods=POST"
--data "config.headers=Accept"
--data "config.headers=Accept-Version"
--data "config.headers=Content-Length"
--data "config.headers=Content-MD5"
--data "config.headers=Content-Type"
--data "config.preflight_continue=false"
HTTP/1.1 201 Created
Date: Fri, 02 Aug 2019 00:05:16 GMT
Content-Type: application/json; charset=utf-8
Connection: keep-alive
Access-Control-Allow-Origin: *
Server: kong/1.2.1
Content-Length: 478

{"created_at":1564704316,"config":{"methods":["POST"],"exposed_headers":null,"max_age":null,"headers":["Accept","Accept-Version","Content-Length","Content-MD5","Content-Type"],"origins":["http://localhost:4200"],"credentials":false,"preflight_continue":false},"id":"05031336-63ee-48df-8b76-2f53aed90399","service":null,"name":"cors","protocols":["http","https"],"enabled":true,"run_on":"first","consumer":null,"route":{"id":"4a904652-0107-4b8f-97e3-73bd5a96c8d2"},"tags":null}

Activity
james-callahan
james-callahan commented on Aug 2, 2019
james-callahan
on Aug 2, 2019
Contributor
--data "config.origins=http://localhost:4200"

Is this the origin being sent by your browser? Please provide the CORS preflight request that is failing.

config.origins:
List of allowed domains for the Access-Control-Allow-Origin header. If you wish to allow all origins, add * as a single value to this configuration field. The accepted values can either be flat strings or PCRE regexes.

nchaudhu
nchaudhu commented on Aug 2, 2019
nchaudhu
on Aug 2, 2019
Author
Yes That is correct. Please find the details below ...I am getting following error whenever I am tying POST . When I use a CORS third party header it works fine.

Error (Endpoint - http://url:8000/xxx/yyy/AAAA
Access to XMLHttpRequest at 'xxxx' from origin 'http://localhost:4200' has been blocked by CORS policy: Response to preflight request doesn't pass access control check: No 'Access-Control-Allow-Origin' header is present on the requested resource.

If the endpoint is - https://cors-anywhere.herokuapp.com/http://url:8000/xxx/yyy/AAAA - it works fine

Request (with http headers)
Request URL:
Request Method: OPTIONS
Status Code: 404 Not Found
Remote Address: 54.154.83.36:8000
Referrer Policy: no-referrer-when-downgrade

Provisional headers are shown
Access-Control-Request-Headers: access-control-allow-origin,authorization,content-type
Access-Control-Request-Method: POST
Origin: http://localhost:4200

Request without http headers
Request URL:
Request Method: OPTIONS
Status Code: 404 Not Found
Remote Address: 54.154.83.36:8000
Referrer Policy: no-referrer-when-downgrade
Provisional headers are shown
Access-Control-Request-Headers: content-type
Access-Control-Request-Method: POST
Origin: http://localhost:4200

Plugin
curl -i -X POST http://localhost:8001/services/f09a225b-d484-4259-9f66-efd6abd8190e/plugins --data "name=cors" --data "config.methods=GET" --data "config.methods=POST" --data "config.origins=*" --data "config.preflight_continue=false" --data "config.exposed_headers=Access-Control-Allow-Headers,Access-Control-Allow-Origin" --data "enabled=true"
HTTP/1.1 201 Created
Date: Fri, 02 Aug 2019 01:00:05 GMT
Content-Type: application/json; charset=utf-8
Connection: keep-alive
Access-Control-Allow-Origin: *
Server: kong/1.2.1
Content-Length: 449

{"created_at":1564707605,"config":{"methods":["GET","POST"],"exposed_headers":["Access-Control-Allow-Headers,Access-Control-Allow-Origin"],"max_age":null,"headers":null,"origins":["*"],"credentials":false,"preflight_continue":false},"id":"6d56ec88-3873-4c10-a7cf-d5f63eb1a453","service":{"id":"f09a225b-d484-4259-9f66-efd6abd8190e"},"name":"cors","protocols":["http","https"],"enabled":true,"run_on":"first","consumer":null,"route":null,"tags":null}

Service on which the Plugin is configured
{"host":"54.229.55.90","created_at":1564075509,"connect_timeout":60000,"id":"f09a225b-d484-4259-9f66-efd6abd8190e","protocol":"http","name":"CreatePSDReport","read_timeout":60000,"port":9092,"path":"/gabrielreturn/psdreport","updated_at":1564075509,"retries":5,"write_timeout":60000,"tags":null}

Route for the service (configured with Path)
{"id":"4a904652-0107-4b8f-97e3-73bd5a96c8d2","tags":null,"paths":["/gabrielreturn/psdreport"],"destinations":null,"protocols":["http","https"],"created_at":1564075563,"snis":null,"hosts":null,"name":null,"preserve_host":false,"regex_priority":0,"strip_path":true,"sources":null,"updated_at":1564075563,"https_redirect_status_code":426,"service":{"id":"f09a225b-d484-4259-9f66-efd6abd8190e"},"methods":["POST"]}

nchaudhu
nchaudhu commented on Aug 2, 2019
nchaudhu
on Aug 2, 2019
Author
I have tried with config.origins=* . But still same error.

nchaudhu
nchaudhu commented on Aug 3, 2019
nchaudhu
on Aug 3, 2019
Author
@james-callahan any inputs please. This is a blocker for me and we have no clue how to solve this. Really urgent. Any help is highly appreciated. Please help.

Yamilquery
Yamilquery commented on Aug 9, 2019
Yamilquery
on Aug 9, 2019 · edited by Yamilquery
You need to ensure that you have a Route configured with the OPTIONS method.

This is because of your request, which is a special cross-domain request. The specification mandates that browsers "preflight" the request, soliciting supported methods from the server with an HTTP OPTIONS request method, and then, upon "approval" from the server, sending the actual request with the actual HTTP request method.

At first, I was confused because Kong Services doesn't allow me to create a Service with the OPTIONS method, but yesterday I discovered that the Kong Routes allows it!


