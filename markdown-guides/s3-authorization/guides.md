Authorized requests to S3 bucket
Protected S3 buckets, protected files
David (drbh) Holtz
David (drbh) Holtz

Follow
2 min read
·
Mar 21, 2018
14

This notebook shows the finished product of adding basic permissioning to an S3 bucket

We use basic auth which is an HTTP protocol for simple auth on web-accessible files. https://en.wikipedia.org/wiki/Basic_access_authentication

Basic auth isn’t very secure — however, we pair this with HTTPS and restrict access to the s3 bucket.

Set up some python stuff
In [1]:

import requests; import json
Access secure endpoint without auth
first were gonna try to access this file without any credentials

In [2]:

url = 'https://d17nii79zr8aom.cloudfront.net/success.json'
resp = requests.get(url)
resp.content
Out[2]:

'Unauthorized'
Next we add basic auth params

Access secure endpoint with auth!
In [3]:

user, password = 'user', 'pass'
resp = requests.get(url, auth=(user, password))
data = json.loads(resp.content)
print json.dumps(data, indent=4)
Out [3]:

{
"status": "success",
"secret": "yay now we can lockdown s3 files!"
}
Okay cool, hackers dont care about the front door. Lets try to acess the direct url of the S3 object

Direct S3 bucket access
In [4]:

direct_url = 'https://s3.amazonaws.com/locked-box/success.json'
resp = requests.get(direct_url)
print resp.content
Out [4]:

<?xml version="1.0" encoding="UTF-8"?>

<Error><Code>AccessDenied</Code><Message>Access Denied</Message><RequestId>58277072A5A1F927</RequestId><HostId>2CmgTzauvXbV0+bf9jMKvlXj3ViMNw4bUL1JMnu4L1QqHfOu0/eHJfG0cxunR0nq7hrVJb8HpQ0=</HostId></Error>
okay obviously that didnt work — we didnt even use the credentials. Lets pretend we know the login credentials but use them directly on the S3 bucket and not the secure endpoint.

In [5]:

user, password = 'user', 'pass'
resp = requests.get(direct_url, auth=(user, password))
print resp.content
Out [5]:

<?xml version="1.0" encoding="UTF-8"?>

<Error><Code>InvalidArgument</Code><Message>Unsupported Authorization Type</Message><ArgumentName>Authorization</ArgumentName><ArgumentValue>Basic dXNlcjpwYXNz</ArgumentValue><RequestId>97760837E823C675</RequestId><HostId>MaKcLnOik5Bq4zV+2v9fNzKqikz7JEHdEIv7TJYUP+67jJmdU4w9ekOr9jaZIbGHj+Wz68M4RcI=</HostId></Error>
that didnt access it! woooo!

success 🤘🏽

---

Uploading to Amazon S3 via ng-file-upload; Error 400
Asked 10 years, 2 months ago
Modified 5 years, 3 months ago
Viewed 5k times
Part of AWS Collective
2

I am using ng-file-upload to upload a photo to S3. I have tested my S3 bucket and my policy/signature generator with the ng-file-upload demo tool and have successfully used it to upload the photo to my bucket.

Edit: A key difference seems to be the addition of a header:

Authorization:Token 3j4fl8jk0lqfkj4izj2w3ljfopljlwep1010notreal

That is present in my code but not in the demo page. Perhaps it's my angular app.

Installed through bower, took the relevant components, tried to upload a file, and get an error that looks like this:

400: <?xml version="1.0" encoding="UTF-8"?>
<Error>
<Code>InvalidArgument</Code>
<Message>Unsupported Authorization Type</Message>
<ArgumentName>Authorization</ArgumentName>
<ArgumentValue>Token 3j4fl8jk0lqfkj4izj2w3ljfopljlwep1010notreal</ArgumentValue>
<RequestId>F1F5FK-not-real-81FED9CA</RequestId>
<HostId>CQEW98p3D2e+pVz-not-real-codeu28m7asqpGjagL3409gj3f4kijKJpofk</HostId>
</Error>
Searching around, I've noted many 400 Errors but not many cases with the ArgumentValue as Token [some-code-here].

Looking at AWS documentation, InvalidArgument is a bit opaque for someone rather new to AWS.

And here is the policy that I'm encoding (Python):

#exptime = '2100-07-31T06:23:35Z' #for debugging
policy_document = {"expiration": exptime,
"conditions": [
{"bucket": settings.AWS_STORAGE_BUCKET_NAME},
["starts-with", '$key', ""],
{"acl": "private"},
["starts-with", "$Content-Type", ""],
["starts-with", "$filename", ""],
["content-length-range", 0, 5000000]
]
}
To reiterate, this policy with the aforementioned demo worked, so I expect a problem on my front-end implementation:

$scope.upload = function(file) {
file.upload = Upload.upload({
url: 'https://mybucket.s3-us-west-2.amazonaws.com',
method: 'POST',
fields: {
key: file.name,
AWSAccessKeyId: myAccessKeyId,
acl: 'private',
policy: myPolicyEncoded,
signature: mySignatureEncoded,
"Content-Type": file.type === null ||
file.type === '' ? 'application/octet-stream' : file.type,
filename: file.name
},
file: file
});
}
pythonangularjsamazon-web-servicesfile-uploadamazon-s3
Share
Improve this question
Follow
edited Jul 13, 2015 at 1:33
asked Jul 12, 2015 at 10:53
rch's user avatar
rch
24933 silver badges1111 bronze badges
Add a comment
2 Answers
Sorted by:

Highest score (default)
6

Took me a while to find it, but as my edit pointed out, looking into the request header a token used in the Angular app's login/auth process was being sent as a default, and Amazon's service didn't like that. Removing the header in the http request for this specific case solved the issue.

The upload service on my front-end thus looks like:

$scope.upload = function(file) {
file.upload = Upload.upload({
url: 'https://mybucket.s3-us-west-2.amazonaws.com',
method: 'POST',
fields: {
key: file.name,
AWSAccessKeyId: myAccessKeyId,
acl: 'private',
policy: myPolicyEncoded,
signature: mySignatureEncoded,
"Content-Type": file.type === null ||
file.type === '' ? 'application/octet-stream' : file.type,
filename: file.name
},
headers: {
'Authorization': undefined
},
file: file
});
}
Share
Improve this answer
Follow
answered Jul 13, 2015 at 1:40
rch's user avatar
rch
24933 silver badges1111 bronze badges
Sign up to request clarification or add additional context in comments.

1 Comment

Diego Mello

---

S3 Upload - Unsupported Authorization Type #615
Closed
Closed
S3 Upload - Unsupported Authorization Type
#615
@Synergi
Description
Synergi
opened on Mar 11, 2015
Don't think this is an issue but rather a request for a little assistance if anybody out there has any suggestions. I have been on this for a while now and I'm not getting anywhere. No matter what I try, I continue to get back the same error message from Amazon.

 <Error>
    <Code>InvalidArgument</Code>
    <Message>Unsupported Authorization Type</Message>
    <ArgumentName>Authorization</ArgumentName>
    <ArgumentValue>Basic U1Q6</ArgumentValue>
    <RequestId>80B90936C37C0CEE</RequestId>
    <HostId>NphuQmN38ri6LvRAlpec8dSC2YwqOKD15ozInX7UmRPhFPZsGmFIghziNfzB/Y7o</HostId>
</Error>
I am using ng-file-upload (https://github.com/danialfarid/ng-file-upload) to upload to S3. I have followed the documentation and have used the demo installation to successfully upload a file to my bucket so I know my AWS Key and Secret Key are correct.

My controller is as follows:

AWSs3 = {
key: 'AWS-KEY-HERE',
filename: file[0].name,
fileSize: file[0].size,
fileType: file[0].type,
bucket: 'manual-attachments',
acl: 'public-read',
timestamp: timestamp
}

$http({
method: "post",
url: "../api/manuals/s3",
data: AWSs3
}).success(function(data, status, headers, config){  
 file.upload = $upload.upload({
url : 'https://bucket.s3.amazonaws.com/',
method : 'POST',
fields : {
key: file[0].name,
acl: AWSs3.acl,
AWSAccessKeyId: AWSs3.key,
policy: data.policy,
signature: data.signature,
"Content-Type" : file.type === null || file.type === '' ? 'application/octet-stream' : file.type,
filename: file[0].name
},
file : file,
});

        file.upload.then(function(response) {
            $timeout(function() {
                file.result = response.data;
            });
        }, function(response) {
            if (response.status > 0)
                $scope.errorMsg = response.status + ': ' + response.data;
        });

        file.upload.progress(function(evt) {
            file.progress = Math.min(100, parseInt(100.0 * evt.loaded / evt.total));
            });

}).error(function(data, status, headers, config){
$log.info('{Error: '+data+'}');
return false;
});
Now the PHP for generating the policy and signature is as follows:

class S3 {
public static $AWS_ACCESS_KEY = "AWS-KEY";
public static $AWS_SECRET_ACCESS_KEY = "AWS-SECRET-KEY";

    public static function get_policy_and_signature( array $data )
    {
        $policy = self::get_policy_doc( $data );
        $signature = self::get_signature( $policy );

    if ( strpos($signature, '+') !== FALSE )
    {
       $data['timestamp'] = intval(@$data['timestamp']) + 1;
       return self::get_policy_and_signature( $data );
    }

    return '{"policy": "'.$policy.'", "signature":"'.$signature.'"}';

}

public static function get_policy_doc(array $data)
{
$now = strtotime(date("Y-m-d\TG:i:s"));
$expire = date('Y-m-d\TG:i:s\Z', strtotime('+ 10 minutes', $now));

    return base64_encode(
      '{'.
           '"expiration": "'.$expire.'",'.
           '"conditions": '.
           '['.
               '{"bucket": "'.$data['bucket'].'"},'.
               '["starts-with", "$key", ""],'.
               '{"acl": "public-read"},'.
               '{"success_action_status": "201"},'.
               '["starts-with", "$Content-Type", ""],'.
               '["starts-with", "$filename", ""],'.
               '["content-length-range",0,5242880]'.
           ']'.
       '}'

);
}

public static function get_signature( $policy_doc ) {
   return base64_encode(hash_hmac(
       'sha1', $policy_doc, self::$AWS_SECRET_ACCESS_KEY, true
));
}
}
The http headers show "Authorization:Basic U1Q6" which is obviously the issue. Any assistance with this would be greatly appreciated.

Activity

Synergi
closed this as completedon Mar 12, 2015
Synergi
Synergi commented on Mar 12, 2015
Synergi
on Mar 12, 2015
Author
I'm an idiot! It's official! The header was the result of my own login/authorisation script that I completely forgot about.

Sorry for any confusion.

sensahin
sensahin commented on Jun 6, 2022
sensahin
on Jun 6, 2022
I'm an idiot! It's official! The header was the result of my own login/authorisation script that I completely forgot about.

Sorry for any confusion.

So how did you solve it? I am also having same issue.

Synergi
Synergi commented on Jun 7, 2022
Synergi
on Jun 7, 2022
Author
I'm an idiot! It's official! The header was the result of my own login/authorisation script that I completely forgot about.
Sorry for any confusion.

So how did you solve it? I am also having same issue.

I examined the headers more closely and realised that my app was configured to globally add an “Authorisation” header for all outgoing requests so it was this additional header that was causing the problem. Once that was identified it was a simple matter of creating an exception for that global header.

Hope that helps you to track it down.

sensahin
sensahin commented on Jun 7, 2022
sensahin
on Jun 7, 2022
I'm an idiot! It's official! The header was the result of my own login/authorisation script that I completely forgot about.

Sorry for any confusion.

So how did you solve it? I am also having same issue.

I examined the headers more closely and realised that my app was configured to globally add an “Authorisation” header for all outgoing requests so it was this additional header that was causing the problem. Once that was identified it was a simple matter of creating an exception for that global header.

Hope that helps you to track it down.

Thank you! I just removed the authorization from header and it worked! Thanks a lot sharing this info. I was trying to fix this issue for days!

pr1ntr
pr1ntr commented on Mar 21
pr1ntr
on Mar 21
10 years later, this issue is still paying dividends
