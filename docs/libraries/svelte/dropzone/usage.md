Title: Declarative | Dropzone

Description: How to create a Dropzone upload with HTML attributes.

- 👋Introduction
- Getting Started

- ⏬Installation

- npm or yarn
- Composer
- Stand-alone file

- ✅Setup

- Declarative
- Imperative
- Server Side Implementation
- Fallback for no JavaScript

- Configuration

- ⚙️Basics

- Configuration Options
- Layout
- Methods
- Upload Queue

- 📞Events
- 💅Theming
- 🤓Tutorials

- Combine form data with files

- Miscellaneous

- Tips & Tricks
- FAQ
- Donate

Powered by GitBook

On this page

- Configuration
- The "auto discover" feature

Was this helpful?

The easiest way to use dropzone is by creating a form element with the class `dropzone`:

Copy

```
<form action="/file-upload"
class="dropzone"
id="my-awesome-dropzone"></form>
```

That’s it. Dropzone will find all form elements with the class dropzone, automatically attach itself to it, and upload files dropped into it to the specified `action` attribute. The uploaded files can be handled just as if there would have been a html input like this:

Copy

```
<input type="file" name="file" />
```

If you want another name than `file` you can configure Dropzone with the option `paramName`.

Configuration

---

When declaring a Dropzone like this, you might be wondering how to configure it then, and fortunately it's quite easy:

1.  Give the element that you want to configure an html `id`

2.  Setup the configuration on `Dropzone.options`

####

Example:

Copy

```
<script>
// Note that the name "myDropzone" is the camelized
// id of the form.
Dropzone.options.myDropzone = {
// Configuration options go here
};
</script>
<form action="/target" class="dropzone" id="my-dropzone"></form>
```

You can create as many options on Dropzone.options as you need — for each HTML element that you created, you can simply add the configuration.

For information on available configuration options, see the configuration section.

###

The "auto discover" feature

The auto discover feature has been removed in Dropzone version 6.0.0! If you depend on this you can simply add this code to the end of your document:

Copy

```
<script>
Dropzone.discover();
</script>
```

The way this works is called "auto discover". Dropzone checks when the DOM content finished loading, and then parses all HTML elements and looks for one with the class `dropzone`. If it finds an element, it checks if the element has an `id`. If it does, then it converst the id name to camel case (`my-dropzone` -> `myDropzone`) and checks if there is an entry in `Dropzone.options` with that key. If there is, then it uses that configuration object to instantiate the Dropzone for this object.

This behaviour can be disabled in two ways:

1.  Either set `Dropzone.autoDiscover = false;` somewhere in your JS. Just make sure it's invoked before the DOM ready event is dispatched.

2.  Set `Dropzone.options.myDropzone = false;` for individual dropzones. This allows you to use the auto discover feature, and only disable it for specific elements.

Dropzone parses the document when the document has finished loading, looks for elements that have the `dropzone` class, checks if there is a configuration in `Dropzone.options` for it, and then creates a Dropzone instance for that element. If all of that happened, before you defined your configuration, then **Dropzone will miss the configuration**. So make sure, that your **configuration** code is **reached before** Dropzone **auto discovers** these elements.

If you're not too keen on this approach, don't worry. Go to the next section to see how to create them imperatively.

PreviousSetupNextImperative

Last updated 3 years ago

Was this helpful?

Title: Imperative | Dropzone

Description: How to create a Dropzone upload with JavaScript.

- 👋Introduction
- Getting Started

- ⏬Installation

- npm or yarn
- Composer
- Stand-alone file

- ✅Setup

- Declarative
- Imperative
- Server Side Implementation
- Fallback for no JavaScript

- Configuration

- ⚙️Basics

- Configuration Options
- Layout
- Methods
- Upload Queue

- 📞Events
- 💅Theming
- 🤓Tutorials

- Combine form data with files

- Miscellaneous

- Tips & Tricks
- FAQ
- Donate

Powered by GitBook

On this page

Was this helpful?

As an alternative to the declarative way, you can create Dropzones imperatively (even on non `<form>`elements) by instantiating the `Dropzone` class:

Plain JavaScript

jQuery

Copy

```
// The constructor of Dropzone accepts two arguments:
//
// 1. The selector for the HTML element that you want to add
//    Dropzone to, the second
// 2. An (optional) object with the configuration
let myDropzone = new Dropzone("div#myId", { url: "/file/post"});
```

Copy

```
// The dropzone method is added to jQuery elements and can
// be invoked with an (optional) configuration object.
$("div#myId").dropzone({ url: "/file/post" });
```

Don’t forget to specify an `url` option if you’re not using a `<form>` element, since Dropzone doesn’t know where to post to without an action attribute. On form elements, Dropzone defaults to the `action` attribute.

For a list of all configuration options, refer to the configuration section.

PreviousDeclarativeNextServer Side Implementation

Last updated 4 years ago

Was this helpful?

Server Side Implementation
How to handle uploaded files on the server.

Dropzone does not provide the server side implementation of handling the files.

The way files are uploaded is identical to a standard file upload with a standard HTML form like this:

Copy

<form action="" method="post" enctype="multipart/form-data">
  <input type="file" name="file" />
</form>
So if your server accepts files, uploaded like this, it will accept files uploaded with Dropzone.

So please look at the corresponding documentation of the server implementation you're using. Here are a few documentations, if you think I should add some, please contact me.

⚙️
Basics
In this section we'll co over the fundamentals of configuring Dropzones.

You should already be familiar on how to pass a configuration object to Dropzone in the declarative and the imperative way. If not, please visit the respective sections.

This is what a typical configuration of Dropzone would look like:

Declarative
Imperative
Copy

<form action="/target" class="dropzone" id="my-great-dropzone"></form>

<script>
  Dropzone.options.myGreatDropzone = { // camelized version of the `id`
    paramName: "file", // The name that will be used to transfer the file
    maxFilesize: 2, // MB
    accept: function(file, done) {
      if (file.name == "justinbieber.jpg") {
        done("Naha, you don't.");
      }
      else { done(); }
    }
  };
</script>

For a list of all possible options, refer to the src/options.js file.

Events
If you want to react to events happening (a file has been added or removed, an upload started, a progress changed, etc...) you want to be listening to events. Checkout the events section for more information on events.

📞
Events
Overriding default event handlers
In general you should never have to do this, and you should be familiar with the code if you decide to do it.

You can also override default event handlers. This should be approached with absolute caution since it can break if you upgrade the library and can also remove default functionality that you might not want to lose:

Copy
let myDropzone = Dropzone("#my-element", {
addedfile: file => {
// ONLY DO THIS IF YOU KNOW WHAT YOU'RE DOING!
}
});
The difference here, is that the default event handler addedfile has been changed, instead of adding a new event handler with myDropzone.on("addedfile", () => {})

Configuration Options
Here is a list of all available options for Dropzone. In case any of this information is outdated (please inform us, if that's the case) or you need more insight, you can always look at the options.js that is used in the actual library.

Name

Default

Description

url

null

Has to be specified on elements other than form (or when the form doesn't have an action attribute). You can also provide a function that will be called with files and must return the url (since v3.12.0)

method

"post"

Can be changed to "put" if necessary. You can also provide a function that will be called with files and must return the method (since v3.12.0).

withCredentials

false

Will be set on the XHRequest.

timeout

null

The timeout for the XHR requests in milliseconds (since v4.4.0). If set to null or 0, no timeout is going to be set.

parallelUploads

2

How many file uploads to process in parallel (See the Enqueuing file uploads documentation section for more info)

uploadMultiple

false

Whether to send multiple files in one request. If this it set to true, then the fallback file input element will have the multiple attribute as well. This option will also trigger additional events (like processingmultiple). See the events documentation section for more information.

chunking

false

Whether you want files to be uploaded in chunks to your server. This can't be used in combination with uploadMultiple.
See [chunksUploaded](#config-chunksUploaded) for the callback to finalise an upload.

forceChunking

false

If chunking is enabled, this defines whether **every** file should be chunked, even if the file size is below chunkSize. This means, that the additional chunk form data will be submitted and the chunksUploaded callback will be invoked.

chunkSize

2000000

If chunking is true, then this defines the chunk size in bytes.

parallelChunkUploads

false

If true, the individual chunks of a file are being uploaded simultaneously.

retryChunks

false

Whether a chunk should be retried if it fails.

retryChunksLimit

3

If retryChunks is true, how many times should it be retried.

maxFilesize

256

The maximum filesize (in bytes) that is allowed to be uploaded.

paramName

"file"

The name of the file param that gets transferred. **NOTE**: If you have the option uploadMultiple set to true, then Dropzone will append [] to the name.

createImageThumbnails

true

Whether thumbnails for images should be generated

maxThumbnailFilesize

10

In MB. When the filename exceeds this limit, the thumbnail will not be generated.

thumbnailWidth

120

If null, the ratio of the image will be used to calculate it.

thumbnailHeight

120

The same as thumbnailWidth. If both are null, images will not be resized.

thumbnailMethod

"crop"

How the images should be scaled down in case both, thumbnailWidth and thumbnailHeight are provided. Can be either contain or crop.

resizeWidth

null

If set, images will be resized to these dimensions before being **uploaded**. If only one, resizeWidth **or** resizeHeight is provided, the original aspect ratio of the file will be preserved.
The options.transformFile function uses these options, so if the transformFile function is overridden, these options don't do anything.

resizeHeight

null

See resizeWidth.

resizeMimeType

null

The mime type of the resized image (before it gets uploaded to the server). If null the original mime type will be used. To force jpeg, for example, use image/jpeg. See resizeWidth for more information.

resizeQuality

0.8

The quality of the resized images. See resizeWidth.

resizeMethod

"contain"

How the images should be scaled down in case both, resizeWidth and resizeHeight are provided. Can be either contain or crop.

filesizeBase

1000

The base that is used to calculate the **displayed** filesize. You can change this to 1024 if you would rather display kibibytes, mebibytes, etc... 1024 is technically incorrect, because 1024 bytes are 1 kibibyte not 1 kilobyte. You can change this to 1024 if you don't care about validity.

maxFiles

null

If not null defines how many files this Dropzone handles. If it exceeds, the event maxfilesexceeded will be called. The dropzone element gets the class dz-max-files-reached accordingly so you can provide visual feedback.

headers

null

An optional object to send additional headers to the server. Eg: { "My-Awesome-Header": "header value" }

clickable

true

If true, the dropzone element itself will be clickable, if false nothing will be clickable.
You can also pass an HTML element, a CSS selector (for multiple elements) or an array of those. In that case, all of those elements will trigger an upload when clicked.

ignoreHiddenFiles

true

Whether hidden files in directories should be ignored.

acceptedFiles

null

The default implementation of accept checks the file's mime type or extension against this list. This is a comma separated list of mime types or file extensions.
Eg.: image/\*,application/pdf,.psd
If the Dropzone is clickable this option will also be used as [accept](https://developer.mozilla.org/en-US/docs/HTML/Element/input#attr-accept) parameter on the hidden file input as well.

acceptedMimeTypes

null

**Deprecated!** Use acceptedFiles instead.

autoProcessQueue

true

If false, files will be added to the queue but the queue will not be processed automatically. This can be useful if you need some additional user input before sending files (or if you want want all files sent at once). If you're ready to send the file simply call myDropzone.processQueue().
See the [enqueuing file uploads](#enqueuing-file-uploads) documentation section for more information.

autoQueue

true

If false, files added to the dropzone will not be queued by default. You'll have to call enqueueFile(file) manually.

addRemoveLinks

false

If true, this will add a link to every file preview to remove or cancel (if already uploading) the file. The dictCancelUpload, dictCancelUploadConfirmation and dictRemoveFile options are used for the wording.

previewsContainer

null

Defines where to display the file previews – if null the Dropzone element itself is used. Can be a plain HTMLElement or a CSS selector. The element should have the dropzone-previews class so the previews are displayed properly.

disablePreviews

false

Set this to true if you don't want previews to be shown.

hiddenInputContainer

"body"

This is the element the hidden input field (which is used when clicking on the dropzone to trigger file selection) will be appended to. This might be important in case you use frameworks to switch the content of your page.
Can be a selector string, or an element directly.

capture

null

If null, no capture type will be specified If camera, mobile devices will skip the file selection and choose camera If microphone, mobile devices will skip the file selection and choose the microphone If camcorder, mobile devices will skip the file selection and choose the camera in video mode On apple devices multiple must be set to false. AcceptedFiles may need to be set to an appropriate mime type (e.g. "image/_", "audio/_", or "video/\*").

renameFilename

null

**Deprecated**. Use renameFile instead.

renameFile

null

A function that is invoked before the file is uploaded to the server and renames the file. This function gets the File as argument and can use the file.name. The actual name of the file that gets used during the upload can be accessed through file.upload.filename.

forceFallback

false

If true the fallback will be forced. This is very useful to test your server implementations first and make sure that everything works as expected without dropzone if you experience problems, and to test how your fallbacks will look.

---

The HTML that is generated for each file by dropzone is defined with the option previewTemplate which defaults to this:

Copy

<div class="dz-preview dz-file-preview">
  <div class="dz-details">
    <div class="dz-filename"><span data-dz-name></span></div>
    <div class="dz-size" data-dz-size></div>
    <img data-dz-thumbnail />
  </div>
  <div class="dz-progress"><span class="dz-upload" data-dz-uploadprogress></span></div>
  <div class="dz-success-mark"><span>✔</span></div>
  <div class="dz-error-mark"><span>✘</span></div>
  <div class="dz-error-message"><span data-dz-errormessage></span></div>
</div>
The container (dz-preview) gets the dz-processing class when the file gets processed, dz-success when the file got uploaded and dz-error in case the file couldn’t be uploaded. In the latter case, data-dz-errormessage will contain the text returned by the server.

To overwrite the default template, use the previewTemplate config.

You can access the HTML of the file preview in any of the events with file.previewElement.

If you decide to rewrite the previewTemplate from scratch, you should put elements with the data-dz-\* attributes inside:

data-dz-name

data-dz-size

data-dz-thumbnail (This has to be an <img /> element and the alt and src attributes will be changed by Dropzone)

data-dz-uploadprogress (Dropzone will change the style.width property from 0% to 100% whenever there’s a uploadprogress event)

data-dz-errormessage

The default options for Dropzone will look for those element and update the content for it.

If you want some specific link to remove a file (instead of the built in addRemoveLinks config), you can simply insert elements in the template with the data-dz-remove attribute. Example:

Copy
<img src="removebutton.png" alt="Click me to remove the file." data-dz-remove />
You are not forced to use those conventions though. If you override all the default event listeners you can completely rebuild your layout from scratch.

See the installation section on how to add the stylesheet if you want your Dropzone to look like the example dropzone.

See the Theming section, for a more in depth look at how to completely change Dropzone’s UI.

I created an example where I made Dropzone look and feel exactly the way jQuery File Uploader does with a few lines of configuration code. Check it out!

---

If you want to remove an added file from the dropzone, you can call .removeFile(file). This method also triggers the removedfile event.

Here’s an example that would automatically remove a file when it’s finished uploading:

Copy
myDropzone.on("complete", function(file) {
myDropzone.removeFile(file);
});
If you want to remove all files, simply use .removeAllFiles(). Files that are in the process of being uploaded won’t be removed. If you want files that are currently uploading to be canceled, call .removeAllFiles(true) which will cancel the uploads.

If you have autoProcessQueue disabled, you’ll need to call .processQueue()yourself.

This can be useful if you want to display the files and let the user click an accept button to actually upload the file(s).

To access all files in the dropzone, use myDropzone.files.

To get

all accepted files: .getAcceptedFiles()

all rejected files: .getRejectedFiles()

all queued files: .getQueuedFiles()

all uploading files: .getUploadingFiles()

If you do not need a dropzone anymore, just call .disable() on the object. This will remove all event listeners on the element, and clear all file arrays. To reenable a Dropzone use .enable().

If you don’t like the default browser modals for confirm calls, you can handle them yourself by overwriting Dropzone.confirm.

Copy
Dropzone.confirm = function(question, accepted, rejected) {
// Ask the question, and call accepted() or rejected() accordingly.
// CAREFUL: rejected might not be defined. Do nothing in that case.
};
If you want Dropzone to display an image you have on your server, you can use:

Copy
// callback and crossOrigin are optional
let mockFile = { name: "Filename", size: 12345 };
myDropzone.displayExistingFile(mockFile, 'https://image-url');

---

When a file gets added to the dropzone, its status gets set to Dropzone.QUEUED(after the accept function check passes) which means that the file is now in the queue.

If you have the option autoProcessQueue set to true then the queue is immediately processed, after a file is dropped or an upload finished, by calling.processQueue() which checks how many files are currently uploading, and if it’s less than options.parallelUploads, .processFile(file) is called.

If you set autoProcessQueue to false, then .processQueue() is never called implicitly. This means that you have to call it yourself when you want to upload all files currently queued.
